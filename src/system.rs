use crate::{Error, Result};
use std::{
    env,
    io::Write,
    process::{Command, Stdio},
};

pub fn get_current_shell() -> String {
    if cfg!(target_os = "windows") {
        todo!("Windows platform is not supported yet.");
    } else {
        env::var("SHELL").unwrap_or_else(|_| "/bin/sh".to_string())
    }
}

pub fn copy_to_clipboard(text: &str) -> Result<()> {
    if cfg!(target_os = "windows") {
        todo!("Windows platform is not supported yet.");
    } else {
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

        Ok(())
    }
}

pub fn run_cmd(command: &str) -> Result<()> {
    // Detect the current platform
    if cfg!(target_os = "windows") {
        todo!("Windows platform is not supported yet.");
    } else {
        let shell = env::var("SHELL").unwrap_or_else(|_| "/bin/sh".to_string());
        let status = Command::new(shell).arg("-c").arg(command).status()?;
        if !status.success() {
            Err(Error::CommandFailed {
                command: command.to_string(),
            })?
        }

        Ok(())
    }
}
