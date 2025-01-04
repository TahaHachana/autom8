use webdriverbidi::remote::browsing_context::{
    NavigateParameters, ReadinessState, ReloadParameters, TraverseHistoryParameters,
};
use webdriverbidi::session::WebDriverBiDiSession;

// --------------------------------------------------

use crate::error::BotError;

// --------------------------------------------------

const BACK_DELTA: i64 = -1;
const FORWARD_DELTA: i64 = 1;

// --------------------------------------------------

/// Traverses the browsing history in the given context by the specified delta.
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
            BotError::NavigationError(format!("Navigating the history failed: {}", e.to_string()))
        })?;
    Ok(())
}

// --------------------------------------------------

/// Navigates to the specified URL in the given browsing context.
pub async fn load(
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

// --------------------------------------------------

/// Navigates to the previous page in history.
pub async fn go_back(session: &mut WebDriverBiDiSession, context: String) -> Result<(), BotError> {
    traverse_history(session, context.to_owned(), BACK_DELTA).await?;
    Ok(())
}

// --------------------------------------------------

/// Navigates to the next page in history.
pub async fn go_forward(
    session: &mut WebDriverBiDiSession,
    context: String,
) -> Result<(), BotError> {
    traverse_history(session, context.to_owned(), FORWARD_DELTA).await?;
    Ok(())
}

// --------------------------------------------------

/// Reloads the current page.
pub async fn reload(session: &mut WebDriverBiDiSession, context: String) -> Result<(), BotError> {
    let reload_params = ReloadParameters::new(context, None, Some(ReadinessState::Complete));
    session
        .browsing_context_reload(reload_params)
        .await
        .map_err(|e| BotError::NavigationError(e.to_string()))?;
    Ok(())
}
