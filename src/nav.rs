use std::time::Duration;
use log::debug;
use webdriverbidi::model::browsing_context::{
    NavigateParameters, ReadinessState, ReloadParameters, TraverseHistoryParameters,
};
use webdriverbidi::model::script::{EvaluateParameters, Target, ContextTarget, EvaluateResult, RemoteValue, PrimitiveProtocolValue};
use webdriverbidi::session::WebDriverBiDiSession;

// --------------------------------------------------

use crate::error::BrowserError;

// --------------------------------------------------

const BACK_DELTA: i64 = -1;
const FORWARD_DELTA: i64 = 1;

// --------------------------------------------------

/// Traverses the browsing history in the given context by the specified delta.
async fn traverse_history(
    session: &mut WebDriverBiDiSession,
    context: String,
    delta: i64,
) -> Result<(), BrowserError> {
    let traverse_history_params = TraverseHistoryParameters::new(context, delta);
    session
        .browsing_context_traverse_history(traverse_history_params)
        .await
        .map_err(|e| BrowserError::Navigation(format!("Navigating the history failed: {}", e)))?;
    Ok(())
}

// --------------------------------------------------

/// Navigates to the specified URL in the given browsing context.
pub async fn load(
    session: &mut WebDriverBiDiSession,
    browsing_context: String,
    url: &str,
) -> Result<(), BrowserError> {
    let navigate_params = NavigateParameters::new(
        browsing_context.clone(),
        url.into(),
        Some(ReadinessState::Complete),
    );
    session
        .browsing_context_navigate(navigate_params)
        .await
        .map_err(|e| BrowserError::Navigation(e.to_string()))?;
    Ok(())
}

// --------------------------------------------------

/// Navigates to the previous page in history.
pub async fn go_back(
    session: &mut WebDriverBiDiSession,
    context: String,
) -> Result<(), BrowserError> {
    traverse_history(session, context.to_owned(), BACK_DELTA).await?;
    Ok(())
}

// --------------------------------------------------

/// Navigates to the next page in history.
pub async fn go_forward(
    session: &mut WebDriverBiDiSession,
    context: String,
) -> Result<(), BrowserError> {
    traverse_history(session, context.to_owned(), FORWARD_DELTA).await?;
    Ok(())
}

// --------------------------------------------------

/// Reloads the current page.
pub async fn reload(
    session: &mut WebDriverBiDiSession,
    context: String,
) -> Result<(), BrowserError> {
    let reload_params = ReloadParameters::new(context, None, Some(ReadinessState::Complete));
    session
        .browsing_context_reload(reload_params)
        .await
        .map_err(|e| BrowserError::Navigation(e.to_string()))?;
    Ok(())
}

// --------------------------------------------------

/// Waits for any ongoing page loading to complete.
/// This uses JavaScript to check document.readyState and waits for "complete" status.
/// If the page is already loaded, it returns immediately.
///
/// # Arguments
/// - `session`: The WebDriverBiDiSession to use
/// - `context`: The browsing context to check
/// - `timeout_ms`: Maximum time to wait for page load in milliseconds (default: 10000)
///
/// # Errors
/// Returns a `BrowserError::NavigationError` if the page doesn't load within the timeout
pub async fn wait_for_page_load(
    session: &mut WebDriverBiDiSession,
    context: String,
    timeout_ms: Option<u64>,
) -> Result<(), BrowserError> {
    let timeout = Duration::from_millis(timeout_ms.unwrap_or(10000));
    let start_time = std::time::Instant::now();
    
    debug!("Checking page load status for context: {}", context);
    
    while start_time.elapsed() < timeout {
        // Check document ready state
        let script = "document.readyState";
        let target = Target::ContextTarget(ContextTarget::new(context.clone(), None));
        let params = EvaluateParameters::new(script.to_string(), target, false, None, None, None);
        
        match session.script_evaluate(params).await {
            Ok(result) => {
                if let EvaluateResult::EvaluateResultSuccess(success) = result {
                    if let RemoteValue::PrimitiveProtocolValue(
                        PrimitiveProtocolValue::StringValue(state)
                    ) = success.result {
                        debug!("Document ready state: {}", state.value);
                        
                        match state.value.as_str() {
                            "complete" => {
                                debug!("Page is fully loaded");
                                return Ok(());
                            }
                            "interactive" => {
                                debug!("Page is interactive, DOM loaded but resources may still be loading");
                                // For many use cases, interactive is sufficient
                                // But we'll continue waiting for complete state
                            }
                            "loading" => {
                                debug!("Page is still loading");
                            }
                            _ => {
                                debug!("Unknown ready state: {}", state.value);
                            }
                        }
                    }
                }
            }
            Err(e) => {
                debug!("Failed to check document ready state: {}", e);
                // If we can't check the state, assume there might be navigation happening
            }
        }
        
        // Wait a bit before checking again
        tokio::time::sleep(Duration::from_millis(200)).await;
    }
    
    Err(BrowserError::Navigation(format!(
        "Page load timeout after {} milliseconds", 
        timeout.as_millis()
    )))
}
