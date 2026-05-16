// src/logging.rs
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

pub fn init_tracing() {
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "rust_video_cli=debug".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();
}
