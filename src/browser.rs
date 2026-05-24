// src/browser.rs
use chromiumoxide::browser::{Browser, BrowserConfig};
use chromiumoxide::fetcher::{BrowserFetcher, BrowserFetcherOptions};
use chromiumoxide::page::Page;
use futures::StreamExt;

pub async fn launch_browser_and_goto(url: &str) -> Result<Page, Box<dyn std::error::Error>> {
    let browser_config = if let Ok(chrome_path) = which::which("google-chrome") {
        tracing::info!("Using system Chrome");
        BrowserConfig::builder()
            .chrome_executable(chrome_path)
            .build()?
    } else {
        tracing::info!("Chrome not found. Downloading with BrowserFetcher...");

        // Create the .chrome directory if it doesn't exist
        let chrome_dir = std::env::current_dir()?.join(".chrome");
        std::fs::create_dir_all(&chrome_dir)?;

        let fetcher = BrowserFetcher::new(
            BrowserFetcherOptions::builder()
                .with_path(chrome_dir)
                .build()
                .unwrap(),
        );

        let installation = fetcher.fetch().await?;
        let chrome_path = installation.executable_path.clone();

        BrowserConfig::builder()
            .chrome_executable(chrome_path)
            .build()?
    };

    let (browser, mut handler) = Browser::launch(browser_config).await?;

    let _handle = tokio::spawn(async move {
        while let Some(h) = handler.next().await {
            if let Err(e) = h {
                tracing::error!("Browser handler error: {:?}", e);
            }
        }
    });

    let page = browser.new_page(url).await?;
    tracing::info!("Browser launched and navigated to: {}", url);

    Ok(page)
}

/// Performs a login on the current page
pub async fn perform_login(
    page: &Page,
    email: &str,
    password: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    tracing::info!("Filling login form...");

    // Find email input and type
    let email_input = page
        .find_element("input[name='user[email]'], input[type='email']")
        .await?;
    email_input.type_str(email).await?;

    // Find password input and type
    let password_input = page
        .find_element("input[name='user[password]'], input[type='password']")
        .await?;
    password_input.type_str(password).await?;

    // Find and click the submit button
    let submit_button = page
        .find_element("input[type='submit'], button[type='submit']")
        .await?;
    submit_button.click().await?;

    tracing::info!("Login form submitted");

    // Wait a bit for navigation / response
    tokio::time::sleep(std::time::Duration::from_secs(3)).await;

    Ok(())
}

/// Full login flow: launch browser → navigate → fill form → submit
pub async fn login_with_browser(
    login_url: &str,
    email: &str,
    password: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    tracing::info!("Launching browser for login at: {}", login_url);

    // Step 1: Launch browser and navigate to login page
    let page = launch_browser_and_goto(login_url).await?;

    // Step 2: Perform login (fill fields + submit)
    perform_login(&page, email, password).await?;

    tracing::info!("Login flow completed (browser is still open)");

    // Note: We intentionally keep the browser open for now
    // so you can inspect what happened in the visible window.

    Ok(())
}
