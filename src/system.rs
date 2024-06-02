use crate::{Error, Result};
use std::{
    env,
    io::Write,
    process::{Command, Stdio},
};

use dialoguer::console::style;

pub fn get_current_shell() -> String {
    // Detect the current platform
    if cfg!(target_os = "windows") {
        todo!("Windows platform is not supported yet.");
    } else {
        // Unix-like system, detect the shell
        env::var("SHELL").unwrap_or_else(|_| "/bin/sh".to_string())
    }
}

pub fn copy_to_clipboard(text: &str) -> Result<()> {
    // Detect the current platform
    if cfg!(target_os = "windows") {
        todo!("Windows platform is not supported yet.");
    } else {
        // Unix-like system, detect the clipboard tool
        // use pbcopy on macOS and xclip on Linux
        let tool = if cfg!(target_os = "macos") {
            "pbcopy"
        } else {
            "xclip"
        };

        // Execute the command
        Command::new(tool)
            .stdin(Stdio::piped())
            .spawn()?
            .stdin
            .unwrap()
            .write_all(text.as_bytes())?;

        println!("\n {} Copied to clipboard", style("✔").green());
        Ok(())
    }
}

pub fn run_cmd(command: &str) -> Result<()> {
    // Detect the current platform
    if cfg!(target_os = "windows") {
        todo!("Windows platform is not supported yet.");
    } else {
        // Unix-like system, detect the shell
        let shell = env::var("SHELL").unwrap_or_else(|_| "/bin/sh".to_string());
        // Execute the command
        let status = Command::new(shell).arg("-c").arg(command).status()?;

        if !status.success() {
            eprintln!("Command failed with status: {}", status);
            Err(Error::CommandFailed {
                command: command.to_string(),
            })?
        }

        Ok(())
    }
}
