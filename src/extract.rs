use log::debug;
use webdriverbidi::session::WebDriverBiDiSession;
use webdriverbidi::model::script::{
    EvaluateParameters, Target, ContextTarget, EvaluateResult, RemoteValue, PrimitiveProtocolValue
};

// --------------------------------------------------

use crate::error::BrowserError;

// --------------------------------------------------

/// Extracts the inner HTML of an element identified by a CSS selector.
/// 
/// # Arguments
/// - `session`: The WebDriverBiDiSession to use for script execution
/// - `context`: The browsing context where the element should be found
/// - `selector`: CSS selector to identify the element
/// 
/// # Returns
/// - `Ok(String)` containing the innerHTML of the element if found
/// - `Err(BrowserError)` if the element was not found or extraction failed
/// 
/// # Errors
/// Returns a `BrowserError::Element` if:
/// - The element cannot be found with the given selector
/// - The script evaluation fails
pub async fn extract_inner_html(
    session: &mut WebDriverBiDiSession,
    context: &str,
    selector: &str,
) -> Result<String, BrowserError> {
    debug!("Extracting inner HTML for element with selector: {}", selector);
    
    // Escape double quotes in the selector to prevent JavaScript syntax errors
    let escaped_selector = selector.replace("\"", "\\\"");
    
    // JavaScript that finds the element and returns its innerHTML
    let script = format!(
        r#"
        (() => {{
            const element = document.querySelector("{}");
            if (element) {{
                return element.innerHTML;
            }} else {{
                return null;
            }}
        }})()
        "#,
        escaped_selector
    );
    
    let target = Target::ContextTarget(ContextTarget::new(context.to_string(), None));
    let params = EvaluateParameters::new(script, target, false, None, None, None);
    
    let result = session
        .script_evaluate(params)
        .await
        .map_err(|e| BrowserError::Element(format!("Script evaluation failed: {}", e)))?;

    match result {
        EvaluateResult::EvaluateResultSuccess(success) => {
            match success.result {
                RemoteValue::PrimitiveProtocolValue(
                    PrimitiveProtocolValue::StringValue(string_val)
                ) => {
                    debug!("Successfully extracted inner HTML for selector: {}", selector);
                    Ok(string_val.value)
                }
                RemoteValue::PrimitiveProtocolValue(
                    PrimitiveProtocolValue::NullValue(_)
                ) => {
                    Err(BrowserError::Element(format!("Element not found with selector: {}", selector)))
                }
                _ => {
                    debug!("Unexpected result type from innerHTML extraction: {:?}", success.result);
                    Err(BrowserError::Element("Unexpected result type from innerHTML extraction".to_string()))
                }
            }
        }
        EvaluateResult::EvaluateResultException(exception) => {
            Err(BrowserError::Element(format!("Script exception during innerHTML extraction: {:?}", exception.exception_details)))
        }
        EvaluateResult::EmptyResult(_) => {
            Err(BrowserError::Element("Empty result from innerHTML extraction script".to_string()))
        }
    }
}

/// Extracts the inner text of an element identified by a CSS selector.
/// This is equivalent to JavaScript's innerText property.
/// 
/// # Arguments
/// - `session`: The WebDriverBiDiSession to use for script execution
/// - `context`: The browsing context where the element should be found
/// - `selector`: CSS selector to identify the element
/// 
/// # Returns
/// - `Ok(String)` containing the innerText of the element if found
/// - `Err(BrowserError)` if the element was not found or extraction failed
pub async fn extract_inner_text(
    session: &mut WebDriverBiDiSession,
    context: &str,
    selector: &str,
) -> Result<String, BrowserError> {
    debug!("Extracting inner text for element with selector: {}", selector);
    
    let escaped_selector = selector.replace("\"", "\\\"");
    
    let script = format!(
        r#"
        (() => {{
            const element = document.querySelector("{}");
            if (element) {{
                return element.innerText;
            }} else {{
                return null;
            }}
        }})()
        "#,
        escaped_selector
    );
    
    let target = Target::ContextTarget(ContextTarget::new(context.to_string(), None));
    let params = EvaluateParameters::new(script, target, false, None, None, None);
    
    let result = session
        .script_evaluate(params)
        .await
        .map_err(|e| BrowserError::Element(format!("Script evaluation failed: {}", e)))?;

    match result {
        EvaluateResult::EvaluateResultSuccess(success) => {
            match success.result {
                RemoteValue::PrimitiveProtocolValue(
                    PrimitiveProtocolValue::StringValue(string_val)
                ) => {
                    debug!("Successfully extracted inner text for selector: {}", selector);
                    Ok(string_val.value)
                }
                RemoteValue::PrimitiveProtocolValue(
                    PrimitiveProtocolValue::NullValue(_)
                ) => {
                    Err(BrowserError::Element(format!("Element not found with selector: {}", selector)))
                }
                _ => {
                    debug!("Unexpected result type from innerText extraction: {:?}", success.result);
                    Err(BrowserError::Element("Unexpected result type from innerText extraction".to_string()))
                }
            }
        }
        EvaluateResult::EvaluateResultException(exception) => {
            Err(BrowserError::Element(format!("Script exception during innerText extraction: {:?}", exception.exception_details)))
        }
        EvaluateResult::EmptyResult(_) => {
            Err(BrowserError::Element("Empty result from innerText extraction script".to_string()))
        }
    }
}

/// Extracts the value of a specific attribute from an element identified by a CSS selector.
/// 
/// # Arguments
/// - `session`: The WebDriverBiDiSession to use for script execution
/// - `context`: The browsing context where the element should be found
/// - `selector`: CSS selector to identify the element
/// - `attribute`: The name of the attribute to extract
/// 
/// # Returns
/// - `Ok(Some(String))` containing the attribute value if the element and attribute exist
/// - `Ok(None)` if the element exists but the attribute doesn't
/// - `Err(BrowserError)` if the element was not found or extraction failed
pub async fn extract_attribute(
    session: &mut WebDriverBiDiSession,
    context: &str,
    selector: &str,
    attribute: &str,
) -> Result<Option<String>, BrowserError> {
    debug!("Extracting attribute '{}' for element with selector: {}", attribute, selector);
    
    let escaped_selector = selector.replace("\"", "\\\"");
    let escaped_attribute = attribute.replace("\"", "\\\"");
    
    let script = format!(
        r#"
        (() => {{
            const element = document.querySelector("{}");
            if (element) {{
                return element.getAttribute("{}");
            }} else {{
                return undefined;
            }}
        }})()
        "#,
        escaped_selector, escaped_attribute
    );
    
    let target = Target::ContextTarget(ContextTarget::new(context.to_string(), None));
    let params = EvaluateParameters::new(script, target, false, None, None, None);
    
    let result = session
        .script_evaluate(params)
        .await
        .map_err(|e| BrowserError::Element(format!("Script evaluation failed: {}", e)))?;

    match result {
        EvaluateResult::EvaluateResultSuccess(success) => {
            match success.result {
                RemoteValue::PrimitiveProtocolValue(
                    PrimitiveProtocolValue::StringValue(string_val)
                ) => {
                    debug!("Successfully extracted attribute '{}' for selector: {}", attribute, selector);
                    Ok(Some(string_val.value))
                }
                RemoteValue::PrimitiveProtocolValue(
                    PrimitiveProtocolValue::NullValue(_)
                ) => {
                    Ok(None) // Attribute doesn't exist or is null
                }
                RemoteValue::PrimitiveProtocolValue(
                    PrimitiveProtocolValue::UndefinedValue(_)
                ) => {
                    Err(BrowserError::Element(format!("Element not found with selector: {}", selector)))
                }
                _ => {
                    debug!("Unexpected result type from attribute extraction: {:?}", success.result);
                    Err(BrowserError::Element("Unexpected result type from attribute extraction".to_string()))
                }
            }
        }
        EvaluateResult::EvaluateResultException(exception) => {
            Err(BrowserError::Element(format!("Script exception during attribute extraction: {:?}", exception.exception_details)))
        }
        EvaluateResult::EmptyResult(_) => {
            Err(BrowserError::Element("Empty result from attribute extraction script".to_string()))
        }
    }
}