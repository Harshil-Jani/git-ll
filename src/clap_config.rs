use clap::{Parser, Subcommand};

// Define the CLI structure
#[derive(Parser, Debug)]
#[command(name = "git-ll")]
#[command(about = "git-ll integrates Git with the power of the LLaMA 3.1 language model. It also enhances developer productivity.", long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

// Define the subcommands
#[derive(Subcommand, Debug)]
pub enum Commands {
    // git-ll ask "How do I merge two branches locally in git?"
    Ask { prompt: String },
}
