use derive_more::{Display, From};

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, Display, From)]
pub enum Error {
    #[display(fmt = "Prompt '{}' already exists", name)]
    PromptAlreadyExists { name: String },
    #[display(fmt = "Prompt '{}' not found", name)]
    PromptNotFound { name: String },
    #[display(fmt = "Command '{}' failed", command)]
    CommandFailed { command: String },
    #[display(fmt = "API key not set")]
    APIKeyNotSet,
    #[display(fmt = "Empty response")]
    EmptyResponse,

    #[from]
    OpenAI(async_openai::error::OpenAIError),
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
