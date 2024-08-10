use crate::constants::LLAMA_MODEL_VERSION;
use ollama_rs::{generation::completion::request::GenerationRequest, Ollama};
use indicatif::{ProgressBar, ProgressStyle};
use std::time::Duration;

async fn ask_llama(prompt: String) -> Result<String, Box<dyn std::error::Error>> {
    // By default it will connect to localhost:11434
    let ollama = Ollama::default();
    let model = LLAMA_MODEL_VERSION;

    // Create a spinner to keep the CLI busy while model is generating a response
    let spinner = ProgressBar::new_spinner();
    spinner.set_message("Generating response...");
    spinner.enable_steady_tick(Duration::from_millis(100)); // Spin every 100ms
    spinner.set_style(ProgressStyle::default_spinner().tick_strings(&["-", "\\", "|", "/"]));

    let res = ollama.generate(GenerationRequest::new(model, prompt)).await;

    spinner.finish_and_clear(); // Stop the spinner and clear it

    match res {
        Ok(res) => Ok(res.response),
        Err(e) => Err(Box::new(e)),
    }
}