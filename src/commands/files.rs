use anyhow::Result;
use serde_json::Value;
use crate::AiderExtension;
use zed_extension::execute_command; // Import execute_command

pub async fn list_files(extension: &AiderExtension, _: Value) -> Result<()> {
    println!("Listing files...");
    Ok(())
}

pub async fn ask_aider(extension: &AiderExtension, _: Value) -> Result<()> {
    println!("Asking Aider...");
    let command_result = execute_command(
        "aider --version",
        false // requires_approval = false, since it's a safe command
    ).await;

    match command_result {
        Ok(output) => {
            println!("Aider version output:\n{}", output);
        }
        Err(error) => {
            eprintln!("Error executing aider --version: {}", error);
        }
    }
    Ok(())
}
