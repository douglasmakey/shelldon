use crate::processor::{CompletionGenerator, Message};
use crate::{Error, Result};
use async_stream::stream;
use futures::{stream::LocalBoxStream, StreamExt};
use genai::chat::ChatOptions;
use genai::{
    chat::{ChatMessage, ChatRequest, ChatStreamEvent, StreamChunk},
    Client,
};

pub struct GenAI {
    client: Client,
}

impl GenAI {
    pub fn new() -> Self {
        Self {
            client: Client::default(),
        }
    }
}

impl CompletionGenerator for GenAI {
    async fn stream_completion(
        &self,
        model: &str,
        temperature: f64,
        prompt: &str,
        input: &str,
    ) -> Result<LocalBoxStream<'_, String>> {
        let req = ChatRequest::new(vec![ChatMessage::system(prompt), ChatMessage::user(input)]);
        let options = ChatOptions::default().with_temperature(temperature);
        let resp = self
            .client
            .exec_chat_stream(model, req.clone(), Some(&options))
            .await?;

        let async_stream = stream! {
                let mut stream = resp.stream;
                while let Some(Ok(stream_event)) = stream.next().await {
                    if let ChatStreamEvent::Chunk(StreamChunk { content }) = stream_event {
                        yield content;
                    }
                };
        };

        Ok(Box::pin(async_stream))
    }

    async fn chat_completion(
        &self,
        model: &str,
        temperature: f64,
        messages: &[Message],
    ) -> Result<String> {
        let chat_messages: Vec<ChatMessage> = messages
            .iter()
            .map(|m| match m {
                Message::System(content) => ChatMessage::system(content),
                Message::User(content) => ChatMessage::user(content),
                Message::Assistant(content) => ChatMessage::assistant(content),
            })
            .collect();

        let req = ChatRequest::new(chat_messages);
        let options = ChatOptions::default().with_temperature(temperature);
        let resp = self
            .client
            .exec_chat(model, req, Some(&options))
            .await?;
        resp.content.joined_texts().ok_or(Error::EmptyResponse)
    }
}
