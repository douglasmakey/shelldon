use crate::processor::CompletionGenerator;
use crate::{Error, Result};
use async_openai::{
    config::OpenAIConfig,
    types::{
        ChatCompletionRequestSystemMessageArgs, ChatCompletionRequestUserMessageArgs,
        CreateChatCompletionRequestArgs,
    },
    Client,
};
use async_stream::stream;
use futures::stream::LocalBoxStream;

pub struct OpenAI {
    client: Client<OpenAIConfig>,
}

impl OpenAI {
    pub fn new() -> Result<OpenAI> {
        if std::env::var("OPENAI_API_KEY").is_err() {
            return Err(Error::APIKeyNotSet)?;
        }

        Ok(OpenAI {
            client: Client::new(),
        })
    }
}

impl CompletionGenerator for OpenAI {
    async fn generate_completion(
        &self,
        model: &str,
        temperature: f32,
        prompt: &str,
        input: &str,
    ) -> Result<String> {
        let messages = [
            ChatCompletionRequestSystemMessageArgs::default()
                .content(prompt)
                .build()?
                .into(),
            ChatCompletionRequestUserMessageArgs::default()
                .content(input)
                .build()?
                .into(),
        ];

        let request = CreateChatCompletionRequestArgs::default()
            .model(model)
            .temperature(temperature)
            .messages(messages)
            .build()?;

        let response = self.client.chat().create(request).await?;

        Ok(response
            .choices
            .first()
            .unwrap()
            .message
            .content
            .clone()
            .unwrap())
    }

    async fn stream_completion(
        &self,
        model: &str,
        temperature: f32,
        prompt: &str,
        input: &str,
    ) -> Result<LocalBoxStream<String>> {
        let messages = [
            ChatCompletionRequestSystemMessageArgs::default()
                .content(prompt)
                .build()?
                .into(),
            ChatCompletionRequestUserMessageArgs::default()
                .content(input)
                .build()?
                .into(),
        ];

        let request = CreateChatCompletionRequestArgs::default()
            .model(model)
            .temperature(temperature)
            .messages(messages)
            .stream(true)
            .build()?;

        let async_stream = stream! {
            let st = match self.client.chat().create_stream(request).await {
                Ok(response) => response,
                Err(_) => {
                    return;
                }
            };

            for await response in st {
                match response {
                    Ok(response) => {
                        yield response.choices[0].delta.content.clone().unwrap_or_default();
                    },
                    Err(_) => {
                        break;
                    }
                };
            }
        };

        Ok(Box::pin(async_stream))
    }
}
