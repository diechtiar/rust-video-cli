// src/main.rs
use clap::Parser;
use rust_video_cli::cli::Cli;

fn main() {
    let cli = Cli::parse();

    if cli.debug {
        println!("[DEBUG] Debug mode enabled");
    }

    println!("rust-video-cli v{} started", env!("CARGO_PKG_VERSION"));
}
