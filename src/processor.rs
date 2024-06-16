use crate::Result;
use futures::stream::LocalBoxStream;

pub trait CompletionGenerator {
    async fn generate_completion(
        &self,
        model: &str,
        temperature: f32,
        prompt: &str,
        input: &str,
    ) -> Result<String>;

    async fn stream_completion(
        &self,
        model: &str,
        temperature: f32,
        prompt: &str,
        input: &str,
    ) -> Result<LocalBoxStream<String>>;
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
    pub async fn generate(
        &self,
        prompt: &str,
        input: &str,
        model: &str,
        temperature: f32,
    ) -> Result<String> {
        self.generator
            .generate_completion(model, temperature, prompt, input)
            .await
    }

    pub async fn generate_stream(
        &self,
        prompt: &str,
        input: &str,
        model: &str,
        temperature: f32,
    ) -> Result<LocalBoxStream<String>> {
        self.generator
            .stream_completion(model, temperature, prompt, input)
            .await
    }
}
