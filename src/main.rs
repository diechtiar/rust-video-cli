use clap::Parser;
use rust_video_cli::{cli::Cli, logging::init_tracing};

#[tokio::main]
async fn main() {
    init_tracing();

    let cli = Cli::parse();
    let http_client = rust_video_cli::http::HttpClient::new();

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
}
