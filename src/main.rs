use ollama_rs::{generation::completion::request::GenerationRequest, Ollama};
use tokio; // Import the Tokio runtime
use indicatif::{ProgressBar, ProgressStyle};
use std::time::Duration;

#[tokio::main] // Marks the entry point as an async function
async fn main() {
    // By default it will connect to localhost:11434
    let ollama = Ollama::default();
    let model = "llama3.1".to_string();
    let prompt = "Why is the sky blue?".to_string();

    // Create a spinner to keep the CLI busy
    let spinner = ProgressBar::new_spinner();
    spinner.set_message("Generating response...");
    spinner.enable_steady_tick(Duration::from_millis(100)); // Spin every 100ms
    spinner.set_style(ProgressStyle::default_spinner().tick_strings(&["-", "\\", "|", "/"]));

    let res = ollama.generate(GenerationRequest::new(model, prompt)).await;

    spinner.finish_and_clear(); // Stop the spinner and clear it

    if let Ok(res) = res {
        println!("{}", res.response);
    } else {
        eprintln!("Error generating response: {:?}", res);
    }
}
