use clap::Parser;
use rust_video_cli::browser::login_with_browser;
use rust_video_cli::{cli::Cli, logging::init_tracing};

#[tokio::main]
async fn main() {
    init_tracing();

    let cli = Cli::parse();

    if cli.debug {
        tracing::debug!("Debug mode enabled");
    }

    tracing::info!("rust-video-cli v{} started", env!("CARGO_PKG_VERSION"));

    // Create HTTP client
    // let http_client = match HttpClient::new() {
    //     Ok(client) => client,
    //     Err(e) => {
    //         tracing::error!("Failed to create HTTP client: {}", e);
    //         return;
    //     }
    // };

    // === Login Flow ===
    // if let (Some(url), Some(username), Some(password)) = (&cli.url, &cli.username, &cli.password) {
    //     tracing::info!("Attempting login for user: {}", username);

    //     let credentials = [
    //         ("username", username.as_str()),
    //         ("password", password.as_str()),
    //     ];

    //     match http_client.login(url, &credentials).await {
    //         Ok(response) => {
    //             tracing::info!(
    //                 "Login request completed. Response length: {} bytes",
    //                 response.len()
    //             );
    //             // TODO: Later we can check if login was successful
    //         }
    //         Err(e) => {
    //             tracing::error!("Login failed: {}", e);
    //         }
    //     }
    // } else {
    //     tracing::warn!("No login credentials provided. Use --url, --username and --password");
    // }

    if let (Some(url), Some(username), Some(password)) = (&cli.url, &cli.username, &cli.password) {
        if let Err(e) = login_with_browser(url, username, password).await {
            tracing::error!("Browser login failed: {}", e);
        }
    }
}
