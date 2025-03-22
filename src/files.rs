use anyhow::Result;
use serde_json::Value;
use crate::AiderExtension;
use zed_extension::{execute_command, workspace::current_workspace_path}; // Import execute_command and current_workspace_path

pub async fn list_files(extension: &AiderExtension, _: Value) -> Result<()> {
    println!("Listing files...");
    if let Some(workspace_path) = current_workspace_path() {
        let command = format!("find {} -type f", workspace_path.display());
        let command_result = execute_command(
            &command,
            false // requires_approval = false, since it's a safe command
        ).await;

        match command_result {
            Ok(output) => {
                println!("List of files:\n{}", output);
            }
            Err(error) => {
                eprintln!("Error listing files: {}", error);
            }
        }
    } else {
        println!("No workspace path found.");
    }
    Ok(())
}

pub async fn ask_aider(extension: &AiderExtension, args: Value) -> Result<()> {
    println!("Asking Aider...");
    let prompt = args["prompt"].as_str().unwrap_or_default();
    let command = format!("aider {}", prompt);
    let command_result = execute_command(
        &command,
        false // requires_approval = false, since it's a safe command for now
    ).await;

    match command_result {
        Ok(output) => {
            println!("Aider output:\n{}", output);
        }
        Err(error) => {
            eprintln!("Error executing aider: {}", error);
        }
    }
    Ok(())
}
