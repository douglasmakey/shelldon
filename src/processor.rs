use crate::Result;
use futures::stream::LocalBoxStream;

/// Represents a message in a conversation.
#[derive(Debug, Clone)]
pub enum Message {
    System(String),
    User(String),
    Assistant(String),
}

pub trait CompletionGenerator {
    async fn stream_completion(
        &self,
        model: &str,
        temperature: f64,
        prompt: &str,
        input: &str,
    ) -> Result<LocalBoxStream<'_, String>>;

    /// Generate a completion from a conversation history.
    async fn chat_completion(
        &self,
        model: &str,
        temperature: f64,
        messages: &[Message],
    ) -> Result<String>;
}

pub struct CompletionProcessor<T: CompletionGenerator> {
    generator: T,
}

impl<T: CompletionGenerator> CompletionProcessor<T> {
    pub fn new(generator: T) -> Self {
        Self { generator }
    }
}

impl<T: CompletionGenerator> CompletionProcessor<T> {
    pub async fn generate_stream(
        &self,
        prompt: &str,
        input: &str,
        model: &str,
        temperature: f64,
    ) -> Result<LocalBoxStream<'_, String>> {
        self.generator
            .stream_completion(model, temperature, prompt, input)
            .await
    }

    /// Generate a completion from a conversation history.
    pub async fn chat(
        &self,
        messages: &[Message],
        model: &str,
        temperature: f64,
    ) -> Result<String> {
        self.generator
            .chat_completion(model, temperature, messages)
            .await
    }
}
