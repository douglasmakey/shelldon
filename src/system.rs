use crate::{Error, Result};
use std::{
    env,
    io::Write,
    process::{Command, Stdio},
};

/// Returns the current shell path.
///
/// On Unix-like systems, reads from SHELL environment variable.
/// Falls back to /bin/sh if not set.
///
/// # Panics
///
/// Currently panics on Windows (not yet supported).
pub fn get_current_shell() -> String {
    if cfg!(target_os = "windows") {
        todo!("Windows platform is not supported yet.");
    }
    env::var("SHELL").unwrap_or_else(|_| "/bin/sh".to_string())
}

/// Returns just the shell name (e.g., "bash", "zsh") without the full path.
pub fn get_shell_name() -> String {
    let shell = get_current_shell();
    shell.rsplit('/').next().unwrap_or("sh").to_string()
}

pub fn copy_to_clipboard(text: &str) -> Result<()> {
    if cfg!(target_os = "windows") {
        todo!("Windows platform is not supported yet.");
    }

    let tool = if cfg!(target_os = "macos") {
        "pbcopy"
    } else {
        "xclip"
    };

    // Execute the command
    let mut child = Command::new(tool).stdin(Stdio::piped()).spawn()?;
    child
        .stdin
        .as_mut()
        .ok_or(Error::ClipboardStdinUnavailable)?
        .write_all(text.as_bytes())?;

    Ok(())
}

pub fn run_cmd(command: &str) -> Result<()> {
    // Detect the current platform
    if cfg!(target_os = "windows") {
        todo!("Windows platform is not supported yet.");
    }

    let shell = get_current_shell();
    let status = Command::new(&shell).arg("-c").arg(command).status()?;
    if !status.success() {
        Err(Error::CommandFailed {
            command: command.to_string(),
        })?
    }

    Ok(())
}