use log::debug;
use webdriverbidi::remote::browsing_context::GetTreeParameters;
use webdriverbidi::session::WebDriverBiDiSession;

// --------------------------------------------------

use crate::error::BotError;
use crate::nav;

// --------------------------------------------------

// Alias Capabilities and CapabilityRequest from webdriverbidi for easy import
pub type Capabilities = webdriverbidi::webdriver::capabilities::Capabilities;
pub type CapabilityRequest = webdriverbidi::webdriver::capabilities::CapabilityRequest;

// --------------------------------------------------

/// The `Browser` struct provides an interface for managing a WebDriver BiDi session
/// and performing browser operations such as opening, closing, and navigating to URLs.
///
/// # Fields
/// - `webdriverbidi_session`: An instance of `WebDriverBiDiSession` which manages the WebDriver BiDi session.
/// - `browsing_context`: An optional `String` that holds the current browsing context identifier.
///
/// # Errors
/// Methods in this struct return `Result` types and may produce errors related to session creation,
/// navigation, and other browser operations. These errors are encapsulated in the `BrowserError` enum.
pub struct Bot {
    pub webdriverbidi_session: WebDriverBiDiSession,
    pub browsing_context: Option<String>,
}

// --------------------------------------------------

// WebDriverBiDi session management
impl Bot {
    /// Creates a new `Browser` instance with the specified capabilities, host, and port.
    ///
    /// # Arguments
    /// - `capabilities`: The capabilities required for the WebDriver BiDi session.
    /// - `host`: The host address of the WebDriver BiDi server.
    /// - `port`: The port number of the WebDriver BiDi server.
    ///
    /// # Returns
    /// A new instance of `Browser`.
    pub fn new(capabilities: Capabilities, host: &str, port: u16) -> Self {
        debug!(
            "Creating a new Browser instance with host: {}, port: {}, capabilities: {:?}",
            host, port, capabilities
        );
        Self {
            webdriverbidi_session: WebDriverBiDiSession::new(host.to_string(), port, capabilities),
            browsing_context: None,
        }
    }

    /// Starts a new WebDriver BiDi session and retrieves the browsing context.
    ///
    /// # Errors
    /// Returns a `BrowserError::SessionCreationError` if the session could not be started
    /// or if the `browsingContext.getTree` command fails.
    pub async fn open(&mut self) -> Result<(), BotError> {
        debug!("Starting the WebDriver BiDi session");
        let _ = self.webdriverbidi_session.start().await.map_err(|e| {
            BotError::SessionCreationError(format!(
                "Starting the WebDriverBiDi session failed: {}",
                e.to_string()
            ))
        })?;
        debug!("WebDriver BiDi session started successfully");

        debug!("Retrieving the browsing context tree");
        let get_tree_params = GetTreeParameters::new(None, None);
        let get_tree_rslt = self
            .webdriverbidi_session
            .browsing_context_get_tree(get_tree_params)
            .await
            .map_err(|e| {
                BotError::SessionCreationError(format!(
                    "The browsingContext.getTree command failed: {}",
                    e.to_string()
                ))
            })?;
        self.browsing_context = Some(get_tree_rslt.contexts[0].context.clone());
        debug!("Browsing context retrieved: {:?}", self.browsing_context);
        Ok(())
    }

    /// Closes the WebDriver BiDi session.
    ///
    /// # Errors
    /// Returns a `BrowserError::SessionClosingError` if the session could not be closed.
    pub async fn close(&mut self) -> Result<(), BotError> {
        debug!("Closing the WebDriver BiDi session");
        self.webdriverbidi_session.close().await.map_err(|e| {
            BotError::SessionClosingError(format!(
                "Closing the WebDriver BiDi session failed: {}",
                e.to_string()
            ))
        })?;
        debug!("WebDriver BiDi session closed successfully");
        Ok(())
    }
}

// --------------------------------------------------

// Navigation
impl Bot {
    /// Navigates to the specified URL within the current browsing context.
    ///
    /// # Arguments
    /// - `url`: The URL to navigate to.
    ///
    /// # Errors
    /// Returns a `BrowserError::NavigationError` if no browsing context is available
    /// or if the navigation command fails.
    pub async fn goto(&mut self, url: &str) -> Result<(), BotError> {
        debug!("Navigating to URL: {}", url);
        nav::goto(
            &mut self.webdriverbidi_session,
            self.browsing_context
                .as_ref()
                .ok_or_else(|| {
                    BotError::NavigationError("No browsing context available".to_owned())
                })?
                .to_string(),
            url,
        )
        .await?;
        debug!("Navigation to URL: {} completed successfully", url);
        Ok(())
    }
}
