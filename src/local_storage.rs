use webdriverbidi::local::script::{EvaluateResult, RemoteValue};
use webdriverbidi::remote::script::{
    CallFunctionParameters, ContextTarget, LocalValue, PrimitiveProtocolValue, StringValue, Target,
};
use webdriverbidi::session::WebDriverBiDiSession;

use crate::error::BrowserError;

// --------------------------------------------------

// pub const HOST: &str = "localhost";
// const PORT: u16 = 4444;
// pub const TMP_ROUTE: &str = "/tmp.html";

// --------------------------------------------------

fn local_value(str: &str) -> LocalValue {
    LocalValue::PrimitiveProtocolValue(PrimitiveProtocolValue::StringValue(StringValue::new(
        str.to_string(),
    )))
}

fn target_context(context: &str) -> Target {
    Target::ContextTarget(ContextTarget::new(context.to_string(), None))
}

/// Sets the value for the key in the context's localStorage.
pub async fn set_local_storage(
    bidi_session: &mut WebDriverBiDiSession,
    context: &str,
    key: &str,
    value: &str,
) -> Result<(), BrowserError> {
    let function_declaration = "(key, value) => localStorage.setItem(key, value)".to_string();
    let key_local_value = local_value(key);
    let value_local_value = local_value(value);
    let args = Some(vec![key_local_value, value_local_value]);
    let params = CallFunctionParameters::new(
        function_declaration,
        false,
        target_context(context),
        args,
        None,
        None,
        None,
        None,
    );
    bidi_session
        .script_call_function(params)
        .await
        .map_err(|e| {
            BrowserError::LocalStorage(format!("Setting the local storage value failed: {}", e))
        })?;

    Ok(())
}

/// Returns the value identified by the key from the context's localStorage.
pub async fn get_local_storage(
    bidi_session: &mut WebDriverBiDiSession,
    context: &str,
    key: &str,
) -> Result<Option<String>, BrowserError> {
    let function_declaration = "(key) => localStorage.getItem(key)".to_string();
    let key_local_value = local_value(key);

    let args = Some(vec![key_local_value]);
    let params = CallFunctionParameters::new(
        function_declaration,
        false,
        target_context(context),
        args,
        None,
        None,
        None,
        None,
    );
    let eval_result = bidi_session
        .script_call_function(params)
        .await
        .map_err(|e| {
            BrowserError::LocalStorage(format!("Getting the local storage value failed: {}", e))
        })?;

    match eval_result {
        EvaluateResult::EvaluateResultSuccess(eval_rslt_success) => {
            let remote_value = eval_rslt_success.result;
            match remote_value {
                RemoteValue::PrimitiveProtocolValue(
                    webdriverbidi::local::script::PrimitiveProtocolValue::StringValue(string_value),
                ) => Ok(Some(string_value.value)),
                _ => Ok(None),
            }
        }
        _ => Ok(None),
    }
}
