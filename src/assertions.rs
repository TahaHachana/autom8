use log::debug;
use webdriverbidi::session::WebDriverBiDiSession;
use webdriverbidi::model::script::{
    EvaluateParameters, Target, ContextTarget, EvaluateResult
};

use crate::error::BrowserError;

/// Assert that an element is present in the current page.
pub async fn assert_element_present(
    session: &mut WebDriverBiDiSession,
    context: &str,
    selector: &str,
) -> Result<bool, BrowserError> {
    // Use double quotes for the JavaScript string to avoid conflicts with CSS selectors
    let escaped_selector = selector.replace("\"", "\\\"");
    let script = format!("document.querySelector(\"{}\") !== null", escaped_selector);
    let target = Target::ContextTarget(ContextTarget::new(context.to_string(), None));
    let params = EvaluateParameters::new(script, target, false, None, None, None);
    
    let result = session
        .script_evaluate(params)
        .await
        .map_err(|e| BrowserError::Assertion(format!("Script evaluation failed: {}", e)))?;
  
    match result {
        EvaluateResult::EvaluateResultSuccess(success) => {
            match success.result {
                webdriverbidi::model::script::RemoteValue::PrimitiveProtocolValue(
                    webdriverbidi::model::script::PrimitiveProtocolValue::BooleanValue(bool_val)
                ) => Ok(bool_val.value),
                _ => {
                    debug!("Unexpected result type: {:?}", success.result);
                    Ok(false)
                }
            }
        }
        EvaluateResult::EvaluateResultException(exception) => {
            Err(BrowserError::Assertion(format!("Script exception: {:?}", exception.exception_details)))
        }
        EvaluateResult::EmptyResult(_) => {
            Err(BrowserError::Assertion("Empty result from script evaluation".to_string()))
        }
    }
}