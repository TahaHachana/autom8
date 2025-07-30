use webdriverbidi::model::browsing_context::{
    CaptureScreenshotParameters, CaptureScreenshotParametersOrigin, ImageFormat,
};
use webdriverbidi::session::WebDriverBiDiSession;

// --------------------------------------------------

use crate::error::BrowserError;

// --------------------------------------------------

/// Takes a screenshot of the current page.
pub async fn take_screenshot(
    session: &mut WebDriverBiDiSession,
    context: String,
) -> Result<String, BrowserError> {
    let origin = Some(CaptureScreenshotParametersOrigin::Document);
    let format = Some(ImageFormat {
        // TODO - Strongly typed image format
        image_format_type: "png".to_owned(),
        quality: None,
    });
    let params = CaptureScreenshotParameters {
        context,
        origin,
        format,
        clip: None,
    };
    let rslt = session
        .browsing_context_capture_screenshot(params)
        .await
        .map_err(|e| BrowserError::Screenshot(format!("Taking the screenshot failed: {}", e)))?;

    Ok(rslt.data)
}
