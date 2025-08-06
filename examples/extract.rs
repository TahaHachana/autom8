use anyhow::Result;
use autom8::Browser;

const LOCALHOST: &str = "localhost";
const PORT: u16 = 4444;

#[tokio::main]
async fn main() -> Result<()> {
    // Create a new browser instance
    let mut browser = Browser::new(LOCALHOST, PORT);
    browser.open().await?;

    // Navigate to a test page
    browser.load("https://example.com").await?;
    
    // Wait for page to load
    browser.wait_for_page_load(Some(5000)).await?;

    // Extract inner HTML from the h1 element
    println!("=== Testing extract_inner_html ===");
    match browser.extract_inner_html("h1").await {
        Ok(html) => println!("H1 inner HTML: {}", html),
        Err(e) => println!("Error extracting H1 inner HTML: {}", e),
    }

    // Extract inner text from the h1 element
    println!("\n=== Testing extract_inner_text ===");
    match browser.extract_inner_text("h1").await {
        Ok(text) => println!("H1 inner text: {}", text),
        Err(e) => println!("Error extracting H1 inner text: {}", e),
    }

    // Extract href attribute from the first link
    println!("\n=== Testing extract_attribute ===");
    match browser.extract_attribute("a", "href").await {
        Ok(Some(href)) => println!("First link href: {}", href),
        Ok(None) => println!("First link has no href attribute"),
        Err(e) => println!("Error extracting href: {}", e),
    }

    // Try to extract from a non-existent element
    println!("\n=== Testing with non-existent element ===");
    match browser.extract_inner_html("div.non-existent").await {
        Ok(html) => println!("Unexpected success: {}", html),
        Err(e) => println!("Expected error for non-existent element: {}", e),
    }

    // Extract from the body to see more content
    println!("\n=== Testing extract from body ===");
    match browser.extract_inner_text("body").await {
        Ok(text) => {
            let truncated = if text.len() > 200 {
                format!("{}...", &text[..200])
            } else {
                text
            };
            println!("Body text (first 200 chars): {}", truncated);
        },
        Err(e) => println!("Error extracting body text: {}", e),
    }

    // Close the browser
    browser.close().await?;

    Ok(())
}
