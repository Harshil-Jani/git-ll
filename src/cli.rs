use crate::clap_config::{Cli, Commands};
use crate::llama::ask_llama;

pub async fn handle_cli(cli: Cli) {
    match &cli.command {
        Commands::Ask { prompt } => {
            println!("{}", prompt);
            let response = ask_llama(prompt.to_string()).await.unwrap();
            println!("{}", response);
        }
    }
}