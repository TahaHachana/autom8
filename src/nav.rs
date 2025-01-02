use crate::error::BotError;
use webdriverbidi::remote::browsing_context::{
    NavigateParameters, ReadinessState, TraverseHistoryParameters,
};
use webdriverbidi::session::WebDriverBiDiSession;

// --------------------------------------------------

const BACK_DELTA: i64 = -1;
const FORWARD_DELTA: i64 = 1;

// --------------------------------------------------

async fn traverse_history(
    session: &mut WebDriverBiDiSession,
    context: String,
    delta: i64,
) -> Result<(), BotError> {
    let traverse_history_params = TraverseHistoryParameters::new(context, delta);
    session
        .browsing_context_traverse_history(traverse_history_params)
        .await
        .map_err(|e| {
            BotError::NavigationError(format!(
                "Navigating the history failed: {}",
                e.to_string()
            ))
        })?;
    Ok(())
}

// --------------------------------------------------

/// Navigates to the specified URL in the given browsing context.
///
/// # Arguments
///
/// * `session` - A mutable reference to the WebDriver BiDi session.
/// * `browsing_context` - The browsing context to navigate in.
/// * `url` - The URL to navigate to.
///
/// # Returns
///
/// Ok if successful, or a BrowserError if an error occurs.
pub async fn goto(
    session: &mut WebDriverBiDiSession,
    browsing_context: String,
    url: &str,
) -> Result<(), BotError> {
    let navigate_params = NavigateParameters::new(
        browsing_context.clone(),
        url.into(),
        Some(ReadinessState::Complete),
    );
    session
        .browsing_context_navigate(navigate_params)
        .await
        .map_err(|e| BotError::NavigationError(e.to_string()))?;
    Ok(())
}

pub async fn back(
    session: &mut WebDriverBiDiSession,
    browsing_context: String,
) -> Result<(), BotError> {
    traverse_history(session, browsing_context.to_owned(), BACK_DELTA).await?;
    Ok(())
}

// --------------------------------------------------

// pub fn forward(&self) {
//     // Navigate forward
// }

// pub fn refresh(&self) {
//     // Refresh the page
// }
