use std::fs::File;
use std::io::Write;

// --------------------------------------------------

use anyhow::Result;
use base64::prelude::*;

// --------------------------------------------------

use autom8::Browser;

// --------------------------------------------------

const LOCALHOST: &str = "localhost";
const PORT: u16 = 4444;
const FILE_PATH: &str = "screenshot.png";

// --------------------------------------------------

fn save_screenshot(base64_data: &str, file_path: &str) -> Result<()> {
    // Decode the Base64 string into bytes
    let decoded_data = BASE64_STANDARD.decode(base64_data)?;

    // Create a new file and write the decoded bytes
    let mut file = File::create(file_path)?;
    file.write_all(&decoded_data)?;

    Ok(())
}

// --------------------------------------------------

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize and open a new browser
    let mut browser = Browser::new(LOCALHOST, PORT);
    browser.open().await?;

    // Load rust-lang.org
    browser.load("https://www.rust-lang.org/").await?;
    tokio::time::sleep(tokio::time::Duration::from_secs(2)).await;
    
    // Save a screenshot of the page
    let png = browser.take_screenshot().await?;
    save_screenshot(&png, FILE_PATH)?;

    // Close the browser
    browser.close().await?;

    Ok(())
}
