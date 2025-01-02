use autom8::bot::{Bot, Capabilities, CapabilityRequest};
use tokio;

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

    // Initialize a new browser
    let mut browser = Bot::new(capabilities, "localhost", 4444);
    // Open the browser
    browser.open().await.unwrap();

    // Go to rust-lang.org
    browser.goto("https://www.rust-lang.org/").await.expect("");
    sleep(3).await;

    // Clone the browser
    browser.close().await.unwrap();
}
