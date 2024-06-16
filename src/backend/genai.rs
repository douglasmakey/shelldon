use crate::processor::CompletionGenerator;
use crate::{Error, Result};
use async_stream::stream;
use futures::{stream::LocalBoxStream, StreamExt};
use genai::{
    chat::{ChatMessage, ChatRequest, ChatStreamEvent, StreamChunk},
    client::Client,
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
    async fn generate_completion(
        &self,
        model: &str,
        _temperature: f32,
        prompt: &str,
        input: &str,
    ) -> crate::Result<String> {
        let req = ChatRequest::new(vec![ChatMessage::system(prompt), ChatMessage::user(input)]);
        let resp = self.client.exec_chat(model, req.clone(), None).await?;
        resp.content.ok_or(Error::EmptyResponse)
    }

    async fn stream_completion(
        &self,
        model: &str,
        _temperature: f32,
        prompt: &str,
        input: &str,
    ) -> Result<LocalBoxStream<String>> {
        let req = ChatRequest::new(vec![ChatMessage::system(prompt), ChatMessage::user(input)]);
        let resp = self
            .client
            .exec_chat_stream(model, req.clone(), None)
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
}
