use anyhow::Result;
use tokio::time;

// --------------------------------------------------

use autom8::Browser;

// --------------------------------------------------

const LOCALHOST: &str = "localhost";
const PORT: u16 = 4444;

// --------------------------------------------------

async fn sleep_for_secs(secs: u64) {
    time::sleep(tokio::time::Duration::from_secs(secs)).await
}

// --------------------------------------------------

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize and open a new browser
    let mut browser = Browser::new(LOCALHOST, PORT);
    browser.open().await?;

    // Load rust-lang.org
    browser.load("https://www.rust-lang.org/").await?;

    sleep_for_secs(2).await;

    // Close the browser window
    browser.close().await?;
    Ok(())
}
