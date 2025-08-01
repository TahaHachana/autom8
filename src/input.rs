use log::debug;
use webdriverbidi::session::WebDriverBiDiSession;
use webdriverbidi::model::script::{
    EvaluateParameters, Target, ContextTarget, EvaluateResult, RemoteValue, PrimitiveProtocolValue
};

// --------------------------------------------------

use crate::error::BrowserError;

// --------------------------------------------------

/// Clicks on an element identified by a CSS selector.
/// 
/// # Arguments
/// - `session`: The WebDriverBiDiSession to use for script execution
/// - `context`: The browsing context where the element should be found
/// - `selector`: CSS selector to identify the element to click
/// 
/// # Returns
/// - `Ok(())` if the element was found and clicked successfully
/// - `Err(BrowserError)` if the element was not found or clicking failed
/// 
/// # Errors
/// Returns a `BrowserError::Action` if:
/// - The element cannot be found with the given selector
/// - The script evaluation fails
/// - The element exists but cannot be clicked
pub async fn click_element(
    session: &mut WebDriverBiDiSession,
    context: &str,
    selector: &str,
) -> Result<(), BrowserError> {
    debug!("Attempting to click element with selector: {}", selector);
    
    // Escape double quotes in the selector to prevent JavaScript syntax errors
    let escaped_selector = selector.replace("\"", "\\\"");
    
    // JavaScript that finds the element, checks if it exists, and clicks it
    let script = format!(
        r#"
        (() => {{
            const element = document.querySelector("{}");
            if (element) {{
                // Scroll element into view if needed
                element.scrollIntoView({{ behavior: 'auto', block: 'center' }});
                
                // Click the element
                element.click();
                
                return true;
            }} else {{
                return false;
            }}
        }})()
        "#,
        escaped_selector
    );
    
    let target = Target::ContextTarget(ContextTarget::new(context.to_string(), None));
    let params = EvaluateParameters::new(script, target, false, None, None, None); // awaitPromise = false
    
    let result = session
        .script_evaluate(params)
        .await
        .map_err(|e| BrowserError::Action(format!("Script evaluation failed: {}", e)))?;

    match result {
        EvaluateResult::EvaluateResultSuccess(success) => {
            match success.result {
                RemoteValue::PrimitiveProtocolValue(
                    PrimitiveProtocolValue::BooleanValue(bool_val)
                ) => {
                    if bool_val.value {
                        debug!("Successfully clicked element with selector: {}", selector);
                        Ok(())
                    } else {
                        Err(BrowserError::Action(format!("Element not found with selector: {}", selector)))
                    }
                }
                _ => {
                    debug!("Unexpected result type from click script: {:?}", success.result);
                    Err(BrowserError::Action("Unexpected result type from click operation".to_string()))
                }
            }
        }
        EvaluateResult::EvaluateResultException(exception) => {
            Err(BrowserError::Action(format!("Script exception during click: {:?}", exception.exception_details)))
        }
        EvaluateResult::EmptyResult(_) => {
            Err(BrowserError::Action("Empty result from click script evaluation".to_string()))
        }
    }
}

/// Clicks on an element and waits for it to be clickable first.
/// This is useful for elements that might not be immediately clickable due to loading states.
/// 
/// # Arguments
/// - `session`: The WebDriverBiDiSession to use for script execution
/// - `context`: The browsing context where the element should be found
/// - `selector`: CSS selector to identify the element to click
/// - `timeout_ms`: Maximum time to wait for element to be clickable (default: 5000ms)
/// 
/// # Returns
/// - `Ok(())` if the element was found, became clickable, and was clicked successfully
/// - `Err(BrowserError)` if the element was not found or didn't become clickable within timeout
pub async fn wait_and_click_element(
    session: &mut WebDriverBiDiSession,
    context: &str,
    selector: &str,
    timeout_ms: Option<u64>,
) -> Result<(), BrowserError> {
    let timeout = std::time::Duration::from_millis(timeout_ms.unwrap_or(5000));
    let start_time = std::time::Instant::now();
    
    debug!("Waiting for element to be clickable with selector: {}", selector);
    
    let escaped_selector = selector.replace("\"", "\\\"");
    
    while start_time.elapsed() < timeout {
        // Check if element exists and is clickable
        let check_script = format!(
            r#"
            (() => {{
                const element = document.querySelector("{}");
                if (element) {{
                    const rect = element.getBoundingClientRect();
                    const style = window.getComputedStyle(element);
                    
                    // Check if element is visible and not disabled
                    const isVisible = rect.width > 0 && rect.height > 0 && 
                                    style.visibility !== 'hidden' && 
                                    style.display !== 'none';
                    const isEnabled = !element.disabled;
                    
                    return isVisible && isEnabled;
                }}
                return false;
            }})()
            "#,
            escaped_selector
        );
        
        let target = Target::ContextTarget(ContextTarget::new(context.to_string(), None));
        let params = EvaluateParameters::new(check_script, target, false, None, None, None);
        
        match session.script_evaluate(params).await {
            Ok(EvaluateResult::EvaluateResultSuccess(success)) => {
                if let RemoteValue::PrimitiveProtocolValue(
                    PrimitiveProtocolValue::BooleanValue(bool_val)
                ) = success.result {
                    if bool_val.value {
                        debug!("Element is now clickable, proceeding with click");
                        return click_element(session, context, selector).await;
                    }
                }
            }
            Ok(_) => {
                debug!("Unexpected result while checking element clickability");
            }
            Err(e) => {
                debug!("Error checking element clickability: {}", e);
            }
        }
        
        // Wait a bit before checking again
        tokio::time::sleep(std::time::Duration::from_millis(100)).await;
    }
    
    Err(BrowserError::Action(format!(
        "Element with selector '{}' did not become clickable within {} milliseconds",
        selector,
        timeout.as_millis()
    )))
}
