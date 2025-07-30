use thiserror::Error;

#[derive(Debug, Error)]
pub enum BrowserError {
    #[error("Session creation error: {0}")]
    SessionCreation(String),

    #[error("Session closing error: {0}")]
    SessionClosing(String),

    #[error("Navigation error: {0}")]
    Navigation(String),

    #[error("Action error: {0}")]
    Action(String),

    #[error("Element error: {0}")]
    Element(String),

    #[error("Cookie error: {0}")]
    Cookie(String),

    #[error("JavaScript error: {0}")]
    JavaScript(String),

    #[error("LocalStorage error: {0}")]
    LocalStorage(String),

    #[error("Screenshot error: {0}")]
    Screenshot(String),

    #[error("Assertion error: {0}")]
    Assertion(String),

    #[error("Unknown error: {0}")]
    Unknown(String),
}
