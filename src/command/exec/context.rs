use crate::{
    Result,
    backend::genai::GenAI,
    processor::{CompletionProcessor, Message},
};

/// Context for command execution that maintains conversation history.
pub struct ExecContext {
    processor: CompletionProcessor<GenAI>,
    model: String,
    temperature: f64,
    messages: Vec<Message>,
}

impl ExecContext {
    pub fn new(processor: CompletionProcessor<GenAI>, model: String, temperature: f64, messages: Vec<Message>) -> Self {
        Self {
            processor,
            model,
            temperature,
            messages,
        }
    }

    pub async fn chat(&mut self, user_message: String) -> Result<String> {
        // Add the user's message to history
        self.messages.push(Message::User(user_message));
        let reply = self.processor.chat(&self.messages, &self.model, self.temperature).await?;
        // Add the assistant's response to history
        self.messages.push(Message::Assistant(reply.clone()));
        Ok(reply)
    }

    pub fn get_last_assistant_message(&self) -> Option<&str> {
        self.messages.last().and_then(|m| match m {
            Message::Assistant(c) => Some(c.as_str()),
            _ => None,
        })
    }

    pub fn update_last_assistant_message(&mut self, message: String) {
        if let Some(Message::Assistant(msg)) = self.messages.last_mut() {
            *msg = message;
        }
    }
}

