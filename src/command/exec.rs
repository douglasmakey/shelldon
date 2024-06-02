use super::{parse_prompt, read_input, CommonArgs};
use crate::{
    backend::openai::OpenAI,
    config::Config,
    processor::CompletionProcessor,
    system::{self, copy_to_clipboard, run_cmd},
    Result,
};
use clap::Parser;
use dialoguer::{console::style, theme::ColorfulTheme, Editor, Input};

const RUN: &str = "r";
const COPY: &str = "c";
const ABORT: &str = "a";
const MODIFY: &str = "m";
const OPTIONS: [&str; 4] = [RUN, MODIFY, COPY, ABORT];

const SHELL_PROMPT: &str = r#"Let's think step by step and act as {shell} expert for {os}.
Provide only {shell} commands without any descriptions.
If details are insufficient, provide the most logical solution.
Ensure the output is a valid shell command.
If multiple steps are required, combine them using &&.
Do not use Markdown formatting."#;

#[derive(Parser)]
pub struct ExecArgs {
    #[command(flatten)]
    common: CommonArgs,
    #[clap(
        short,
        long,
        help = "Always run the command without asking for confirmation",
        default_value = "false"
    )]
    run: bool,
}

pub async fn handle_exec(config: Config, args: ExecArgs) -> Result<()> {
    let processor = CompletionProcessor::new(OpenAI::new()?);
    let input = read_input(&args.common.input)?;
    let default_prompt = SHELL_PROMPT
        .replace("{shell}", &system::get_current_shell())
        .replace("{os}", std::env::consts::OS);

    let prompt = parse_prompt(config, args.common.prompt, args.common.set, &default_prompt)?;

    let cmd = processor
        .generate(
            &prompt,
            input.as_str(),
            &args.common.model,
            args.common.temperature,
        )
        .await?;

    if args.run {
        println!(
            "Command to execute: {}",
            dialoguer::console::style(&cmd).green()
        );
        run_cmd(&cmd)?;
    }

    prompt_action_for_cmd(&cmd)
}

fn prompt_action_for_cmd(command: &str) -> Result<()> {
    println!(
        "Command to execute: {}",
        dialoguer::console::style(command).green()
    );

    let option = Input::with_theme(&ColorfulTheme::default())
        .with_prompt("[R]un, [M]odify, [C]opy, [A]bort")
        .validate_with(|input: &String| {
            if OPTIONS.contains(&input.to_lowercase().as_str()) {
                Ok(())
            } else {
                Err("Please enter 'r', 'm', 'c', 'a'")
            }
        })
        .interact_text()?;

    match option.to_lowercase().as_str() {
        RUN => run_cmd(command),
        MODIFY => {
            if let Some(rv) = Editor::new().edit(command).unwrap() {
                prompt_action_for_cmd(&rv)?;
            }
            Ok(())
        }
        COPY => {
            copy_to_clipboard(command)?;
            println!("\n {} Copied to clipboard", style("✔").green());
            Ok(())
        }
        ABORT => Ok(()),
        _ => unreachable!(), // This should never happen due to the validation
    }
}
