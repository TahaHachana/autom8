use thiserror::Error;

#[derive(Debug, Error)]
pub enum BotError {
    #[error("Session creation error: {0}")]
    SessionCreationError(String),

    #[error("Session closing error: {0}")]
    SessionClosingError(String),

    #[error("Navigation error: {0}")]
    NavigationError(String),

    #[error("Action error: {0}")]
    ActionError(String),

    #[error("Element error: {0}")]
    ElementError(String),

    #[error("Cookie error: {0}")]
    CookieError(String),

    #[error("JavaScript error: {0}")]
    JavaScriptError(String),

    #[error("Screenshot error: {0}")]
    ScreenshotError(String),

    #[error("Unknown error: {0}")]
    UnknownError(String),
}
