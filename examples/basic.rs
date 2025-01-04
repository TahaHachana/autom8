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

    // Initialize a new Bot instance
    let mut bot = Bot::new(capabilities, "localhost", 4444);
    // Open the browser window
    bot.open().await.unwrap();

    // Go to rust-lang.org
    bot.load("https://www.rust-lang.org/").await.expect("");
    sleep(3).await;

    // Clone the browser window
    bot.close().await.unwrap();
}
