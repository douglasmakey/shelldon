use derive_more::{Display, From};

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, Display, From)]
pub enum Error {
    PromptAlreadyExists {
        name: String,
    },
    PromptNotFound {
        name: String,
    },
    CommandFailed {
        command: String,
    },
    #[display(fmt = "API key not set")]
    APIKeyNotSet,

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
}
