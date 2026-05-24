use clap::Parser;

#[derive(Parser, Debug)]
#[command(
    author,
    version,
    about = "Rust Video CLI - Download videos from websites"
)]
pub struct Cli {
    /// Enable debug output
    #[arg(short, long)]
    pub debug: bool,

    /// Target website URL
    #[arg(long)]
    pub url: Option<String>,

    /// Username for login
    #[arg(long)]
    pub username: Option<String>,

    /// Password for login
    #[arg(long)]
    pub password: Option<String>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_debug_flag() {
        let cli = Cli::try_parse_from(["rust-video-cli", "--debug"]).unwrap();
        assert!(cli.debug);
    }
}
