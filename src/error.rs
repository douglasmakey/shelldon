use derive_more::{Display, From};

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, Display, From)]
pub enum Error {
    #[display("Prompt '{}' already exists", name)]
    PromptAlreadyExists { name: String },
    #[display("Prompt '{}' not found", name)]
    PromptNotFound { name: String },
    #[display("Command '{}' failed", command)]
    CommandFailed { command: String },
    #[display("API key not set")]
    APIKeyNotSet,
    #[display("Empty response")]
    EmptyResponse,
    #[display("{}", _0)]
    InvalidInput(String),
    #[display("Could not find configuration directory")]
    ConfigDirNotFound,
    #[display("Failed to create directory: {}", path)]
    CreateDirFailed { path: String },
    #[display("Clipboard stdin unavailable")]
    ClipboardStdinUnavailable,

    #[from]
    Io(std::io::Error),
    #[from]
    Regex(regex::Error),
    #[from]
    Serde(serde_json::Error),
    #[from]
    Dialoguer(dialoguer::Error),
    #[from]
    GenAI(genai::Error),
}
