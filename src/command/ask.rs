use super::CommonArgs;
use crate::{
    backend::openai::OpenAI,
    command::{parse_prompt, read_input},
    config::Config,
    processor::CompletionProcessor,
    system, Result,
};
use clap::Parser;
use futures::StreamExt;
use std::io::{stdout, Write};

#[derive(Parser)]
pub struct AskArgs {
    #[command(flatten)]
    common: CommonArgs,
}

pub async fn handle_ask(config: Config, args: AskArgs) -> Result<()> {
    let processor = CompletionProcessor::new(OpenAI::new()?);
    let input = read_input(&args.common.input)?;
    println!("Input: {}", input);
    let prompt = parse_prompt(config, args.common.prompt, args.common.set, "")?;
    let mut completion = processor
        .generate_stream(
            &prompt,
            input.as_str(),
            &args.common.model,
            args.common.temperature,
        )
        .await?;

    let mut whole_buf = String::new();

    let mut lock = stdout().lock();
    while let Some(content) = completion.next().await {
        write!(lock, "{}", content)?;
        whole_buf.push_str(&content);
    }

    writeln!(lock)?;

    if args.common.copy {
        system::copy_to_clipboard(whole_buf.as_str())?;
    }

    Ok(())
}
