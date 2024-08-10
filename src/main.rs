pub mod clap_config;
pub mod cli;
pub mod constants;
pub mod llama;

use clap::Parser;
use clap_config::Cli;
use cli::handle_cli;

#[tokio::main]
async fn main() {
    let cli = Cli::parse();
    handle_cli(cli).await;
}
