use crate::constants::LLAMA_MODEL_VERSION;
use indicatif::{ProgressBar, ProgressStyle};
use ollama_rs::{generation::completion::request::GenerationRequest, Ollama};
use std::time::Duration;
use std::process::Command;

pub fn check_ollama_setup() -> Result<(), Box<dyn std::error::Error>> {
    // Check if ollama is installed
    let ollama_installed = Command::new("ollama").arg("--version").output().is_ok();
    if !ollama_installed {
        return Err("Ollama is not installed".into());
    }

    // Check if the model server is running
    let server_running = Command::new("curl")
        .arg("http://localhost:11434")
        .output()
        .is_ok();
    if !server_running {
        return Err("Model server is not running".into());
    }

    Ok(())
}

pub async fn ask_llama(prompt: String) -> Result<String, Box<dyn std::error::Error>> {
    // By default it will connect to localhost:11434
    let ollama = Ollama::default();
    let model = LLAMA_MODEL_VERSION;

    // Create a spinner to keep the CLI busy while model is generating a response
    let spinner = ProgressBar::new_spinner();
    spinner.set_message("Generating response...");
    spinner.enable_steady_tick(Duration::from_millis(100)); // Spin every 100ms
    spinner.set_style(ProgressStyle::default_spinner().tick_strings(&["-", "\\", "|", "/"]));

    let res = ollama
        .generate(GenerationRequest::new(model.to_string(), prompt))
        .await;

    spinner.finish_and_clear(); // Stop the spinner and clear it

    match res {
        Ok(res) => Ok(res.response),
        Err(e) => Err(Box::<dyn std::error::Error>::from(e)),
    }
}
