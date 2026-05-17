use clap::Parser;
use rust_video_cli::{cli::Cli, logging::init_tracing};

#[tokio::main]
async fn main() {
    init_tracing();

    let cli = Cli::parse();
    let http_client =
        rust_video_cli::http::HttpClient::new().expect("Failed to create HTTP client");

    if cli.debug {
        tracing::debug!("Debug mode enabled");
    }

    tracing::info!("rust-video-cli v{} started", env!("CARGO_PKG_VERSION"));

    tracing::info!("Making test HTTP request...");

    match http_client.get("https://httpbin.org/ip").await {
        Ok(body) => {
            tracing::info!("Response received:\n{}", body);
        }
        Err(e) => {
            tracing::error!("HTTP request failed: {}", e);
        }
    }

    let form_data = [("name", "Wojciech"), ("project", "rust-video-cli")];

    match http_client
        .post("https://httpbin.org/post", &form_data)
        .await
    {
        Ok(body) => {
            tracing::info!("POST Response received:\n{}", body);
        }
        Err(e) => {
            tracing::error!("POST request failed: {}", e);
        }
    }
}
