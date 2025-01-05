use webdriverbidi::remote::browsing_context::{
    CaptureScreenshotParameters, CaptureScreenshotParametersOrigin, ImageFormat,
};
use webdriverbidi::session::WebDriverBiDiSession;

// --------------------------------------------------

use crate::error::BotError;

// --------------------------------------------------

/// Traverses the browsing history in the given context by the specified delta.
pub async fn take_screenshot(
    session: &mut WebDriverBiDiSession,
    context: String,
) -> Result<String, BotError> {
    let params = CaptureScreenshotParameters {
        context,
        origin: Some(CaptureScreenshotParametersOrigin::Document),
        format: Some(ImageFormat {
            image_format_type: "png".to_owned(),
            quality: None,
        }),
        clip: None,
    };

    let rslt = session
        .browsing_context_capture_screenshot(params)
        .await
        .map_err(|e| {
            BotError::ScreenshotError(format!("Taking the screenshot failed: {}", e.to_string()))
        })?;

    Ok(rslt.data)
}
