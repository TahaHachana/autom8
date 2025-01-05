use autom8::bot::{Bot, Capabilities, CapabilityRequest};
use base64::prelude::*;
use std::fs::File;
use std::io::Write;
use tokio;

fn save_screenshot(base64_data: &str, file_path: &str) -> std::io::Result<()> {
    // Decode the Base64 string into bytes
    let decoded_data = BASE64_STANDARD
        .decode(base64_data)
        .expect("Failed to decode Base64 data");

    // Create a new file and write the decoded bytes
    let mut file = File::create(file_path)?;
    file.write_all(&decoded_data)?;

    println!("Screenshot saved to {}", file_path);
    Ok(())
}

// --------------------------------------------------

async fn sleep(secs: u64) {
    tokio::time::sleep(tokio::time::Duration::from_secs(secs)).await
}

// --------------------------------------------------

#[tokio::main]
async fn main() {
    // Define the expected WebDriver capabilities
    let always_match = CapabilityRequest::new();
    let capabilities = Capabilities::new(always_match);

    // Initialize a new Bot instance
    let mut bot = Bot::new(capabilities, "localhost", 4444);
    // Open the browser window
    bot.open().await.unwrap();

    // Go to rust-lang.org
    bot.load("https://www.rust-lang.org/").await.expect("");

    sleep(3).await;
    
    let png = bot.take_screenshot().await.unwrap();
    // Save the screenshot to a file
    if let Err(e) = save_screenshot(png.as_str(), "screenshot.png") {
        eprintln!("Error saving screenshot: {}", e);
    }

    // Clone the browser window
    bot.close().await.unwrap();
}
