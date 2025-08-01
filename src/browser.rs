use log::debug;

// --------------------------------------------------

use webdriverbidi::model::browsing_context::GetTreeParameters;
use webdriverbidi::session::WebDriverBiDiSession;

// --------------------------------------------------

use crate::error::BrowserError;
use crate::{assertions, input, local_storage, nav, screenshot};

// --------------------------------------------------

// Alias Capabilities and CapabilityRequest from webdriverbidi for easy import
pub type CapabilitiesRequest = webdriverbidi::webdriver::capabilities::CapabilitiesRequest;
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
pub struct Browser {
    pub webdriverbidi_session: WebDriverBiDiSession,
    pub browsing_context: Option<String>,
}

// --------------------------------------------------

impl Browser {
    /// Returns the browsing context if it's not None.
    ///
    /// # Errors
    ///
    /// A `BrowserError::NavigationError` error is returned if the context value is None.
    fn get_context(&self) -> Result<String, BrowserError> {
        let ctx = self
            .browsing_context
            .as_ref()
            .ok_or_else(|| BrowserError::Navigation("No browsing context available".to_owned()))?;
        Ok(ctx.to_string())
    }
}

// --------------------------------------------------

// WebDriverBiDi session management
impl Browser {
    /// Creates a new `Browser` instance with default capabilities and the specified host and port.
    ///
    /// # Arguments
    /// - `host`: The host address of the WebDriver BiDi server.
    /// - `port`: The port number of the WebDriver BiDi server.
    ///
    /// # Returns
    /// A new instance of `Browser`.
    pub fn new(host: &str, port: u16) -> Self {
        debug!(
            "Creating a new Browser instance with host: {}, port: {}",
            host, port
        );
        let capabilities = CapabilitiesRequest::default();
        Self {
            webdriverbidi_session: WebDriverBiDiSession::new(host.to_string(), port, capabilities),
            browsing_context: None,
        }
    }

    /// Creates a new `Browser` instance with the specified capabilities, host, and port.
    ///
    /// # Arguments
    /// - `capabilities`: The capabilities required for the WebDriver BiDi session.
    /// - `host`: The host address of the WebDriver BiDi server.
    /// - `port`: The port number of the WebDriver BiDi server.
    ///
    /// # Returns
    /// A new instance of `Browser`.
    pub fn new_with_capabilities(capabilities: CapabilitiesRequest, host: &str, port: u16) -> Self {
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
    pub async fn open(&mut self) -> Result<(), BrowserError> {
        debug!("Starting the WebDriver BiDi session");
        self.webdriverbidi_session.start().await.map_err(|e| {
            BrowserError::SessionCreation(format!(
                "Starting the WebDriverBiDi session failed: {}",
                e
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
                BrowserError::SessionCreation(format!(
                    "The browsingContext.getTree command failed: {}",
                    e
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
    pub async fn close(&mut self) -> Result<(), BrowserError> {
        debug!("Closing the WebDriver BiDi session");
        self.webdriverbidi_session.close().await.map_err(|e| {
            BrowserError::SessionClosing(format!(
                "Closing the WebDriver BiDi session failed: {}",
                e
            ))
        })?;
        debug!("WebDriver BiDi session closed successfully");
        Ok(())
    }
}

// --------------------------------------------------

// Navigation
impl Browser {
    /// Navigates to the specified URL within the current browsing context.
    ///
    /// # Arguments
    /// - `url`: The URL to navigate to.
    ///
    /// # Errors
    /// Returns a `BrowserError::NavigationError` if no browsing context is available
    /// or if the navigation command fails.
    pub async fn load(&mut self, url: &str) -> Result<(), BrowserError> {
        debug!("Navigating to URL: {}", url);
        let ctx = self.get_context()?;
        nav::load(&mut self.webdriverbidi_session, ctx, url).await?;
        debug!("Navigation to URL: {} completed successfully", url);
        Ok(())
    }

    /// Navigates to the previous page in history.
    ///
    /// # Errors
    /// Returns a `BrowserError::NavigationError` if no browsing context is available
    /// or if navigating back failed.
    pub async fn go_back(&mut self) -> Result<(), BrowserError> {
        let ctx = self.get_context()?;
        nav::go_back(&mut self.webdriverbidi_session, ctx).await?;
        Ok(())
    }

    /// Navigates to the next page in history.
    ///
    /// # Errors
    /// Returns a `BrowserError::NavigationError` if no browsing context is available
    /// or if navigating forward failed.
    pub async fn go_forward(&mut self) -> Result<(), BrowserError> {
        let ctx = self.get_context()?;
        nav::go_forward(&mut self.webdriverbidi_session, ctx).await?;
        Ok(())
    }

    /// Reloads the current page.
    ///
    /// # Errors
    /// Returns a `BrowserError::NavigationError` if no browsing context is available
    /// or if navigating forward failed.
    pub async fn reload(&mut self) -> Result<(), BrowserError> {
        let ctx = self.get_context()?;
        nav::reload(&mut self.webdriverbidi_session, ctx).await?;
        Ok(())
    }

    /// Waits for the current page to finish loading.
    ///
    /// # Arguments
    /// - `timeout_ms`: Maximum time to wait for page load in milliseconds (default: 10000)
    ///
    /// # Errors
    /// Returns a `BrowserError::NavigationError` if the page doesn't load within the timeout.
    pub async fn wait_for_page_load(&mut self, timeout_ms: Option<u64>) -> Result<(), BrowserError> {
        let ctx = self.get_context()?;
        nav::wait_for_page_load(&mut self.webdriverbidi_session, ctx, timeout_ms).await?;
        Ok(())
    }
}

// --------------------------------------------------

// Screenshots
impl Browser {
    /// Takes a screenshot of the current page and returns the data as a base64-encoded string.
    ///
    /// # Errors
    /// Returns a `BrowserError::ScreenshotError` if no browsing context is available
    /// or if taking the screenshot fails.
    pub async fn take_screenshot(&mut self) -> Result<String, BrowserError> {
        let ctx = self.get_context()?;
        let data = screenshot::take_screenshot(&mut self.webdriverbidi_session, ctx).await?;
        Ok(data)
    }
}

// --------------------------------------------------

// Local storage
impl Browser {
    /// Sets a value in the local storage of the current browsing context.
    ///
    /// # Arguments
    ///
    /// - `key`: The key to set in the local storage.
    /// - `value`: The value to set in the local storage.
    ///
    /// # Errors
    ///
    /// Returns a `BrowserError::LocalStorageError` if no browsing context is available
    /// or if setting the local storage value fails.
    pub async fn set_local_storage_value(
        &mut self,
        key: &str,
        value: &str,
    ) -> Result<(), BrowserError> {
        let ctx = self.get_context()?;
        local_storage::set_local_storage(&mut self.webdriverbidi_session, ctx.as_str(), key, value)
            .await?;
        Ok(())
    }

    /// Gets a value from the local storage of the current browsing context.
    ///
    /// # Arguments
    ///
    /// - `key`: The key to get from the local storage.
    ///
    /// # Errors
    ///
    /// Returns a `BrowserError::LocalStorageError` if no browsing context is available
    /// or if getting the local storage value fails.
    pub async fn get_local_storage_value(
        &mut self,
        key: &str,
    ) -> Result<Option<String>, BrowserError> {
        let ctx = self.get_context()?;
        local_storage::get_local_storage(&mut self.webdriverbidi_session, ctx.as_str(), key).await
    }
}

// Assertions
impl Browser {
    /// Asserts that an element is present in the current page by checking if it can be selected
    /// using the provided CSS selector.
    ///
    /// # Arguments
    /// - `selector`: The CSS selector of the element to check.
    ///
    /// # Errors
    /// Returns a `BrowserError::AssertionError` if script evaluation fails.
    pub async fn assert_element_present(&mut self, selector: &str) -> Result<bool, BrowserError> {
        let ctx = self.get_context()?;
        assertions::assert_element_present(&mut self.webdriverbidi_session, ctx.as_str(), selector)
            .await
    }
}

// --------------------------------------------------

// Input/Interaction
impl Browser {
    /// Clicks on an element identified by a CSS selector.
    ///
    /// # Arguments
    /// - `selector`: CSS selector to identify the element to click
    ///
    /// # Errors
    /// Returns a `BrowserError::Action` if the element is not found or clicking fails.
    pub async fn click_element(&mut self, selector: &str) -> Result<(), BrowserError> {
        let ctx = self.get_context()?;
        input::click_element(&mut self.webdriverbidi_session, ctx.as_str(), selector).await
    }

    /// Clicks on an element after waiting for it to become clickable.
    ///
    /// # Arguments
    /// - `selector`: CSS selector to identify the element to click
    /// - `timeout_ms`: Maximum time to wait for element to be clickable (default: 5000ms)
    ///
    /// # Errors
    /// Returns a `BrowserError::Action` if the element is not found or doesn't become clickable within timeout.
    pub async fn wait_and_click_element(&mut self, selector: &str, timeout_ms: Option<u64>) -> Result<(), BrowserError> {
        let ctx = self.get_context()?;
        input::wait_and_click_element(&mut self.webdriverbidi_session, ctx.as_str(), selector, timeout_ms).await
    }

    /// Clicks an element and then waits for page load to complete.
    /// This is useful for clicking links or buttons that navigate to a new page.
    ///
    /// # Arguments
    /// - `selector`: CSS selector to identify the element to click
    /// - `page_load_timeout_ms`: Maximum time to wait for page load (default: 10000ms)
    ///
    /// # Errors
    /// Returns a `BrowserError` if clicking fails or page doesn't load within timeout.
    pub async fn click_and_wait(&mut self, selector: &str, page_load_timeout_ms: Option<u64>) -> Result<(), BrowserError> {
        // Click the element
        self.click_element(selector).await?;
        
        // Wait for any resulting page navigation to complete
        self.wait_for_page_load(page_load_timeout_ms).await?;
        
        Ok(())
    }
}
