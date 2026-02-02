use super::{parse_prompt, read_input, CommonArgs};
use crate::{
    backend::genai::GenAI,
    config::Config,
    processor::{CompletionProcessor, Message},
    system::{self, copy_to_clipboard, run_cmd},
    Result,
};
use clap::Parser;
use dialoguer::{console::style, theme::ColorfulTheme, Editor, Input};

/// Context for command execution that maintains conversation history.
struct ExecContext {
    processor: CompletionProcessor<GenAI>,
    model: String,
    temperature: f64,
    messages: Vec<Message>,
}

const RUN: &str = "r";
const COPY: &str = "c";
const ABORT: &str = "a";
const MODIFY: &str = "m";
const ITERATE: &str = "i";
const OPTIONS: [&str; 5] = [RUN, MODIFY, ITERATE, COPY, ABORT];

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
    let processor = CompletionProcessor::new(GenAI::new());
    let input = read_input(args.common.input.as_deref())?;
    let default_prompt = SHELL_PROMPT
        .replace("{shell}", &system::get_shell_name())
        .replace("{os}", std::env::consts::OS);

    let prompt = parse_prompt(config, args.common.prompt, args.common.set, &default_prompt)?;

    // Initialize conversation history with system prompt and user input
    let messages = vec![Message::System(prompt), Message::User(input)];

    let cmd = processor
        .chat(&messages, &args.common.model, args.common.temperature)
        .await?;

    if args.run {
        println!(
            "Command to execute: {}",
            dialoguer::console::style(&cmd).green()
        );
        run_cmd(&cmd)?;
        return Ok(());
    }

    // Create execution context with conversation history
    let mut ctx = ExecContext {
        processor,
        model: args.common.model,
        temperature: args.common.temperature,
        messages,
    };

    // Add the assistant's response to history
    ctx.messages.push(Message::Assistant(cmd.clone()));

    prompt_action_for_cmd(&mut ctx, &cmd).await
}

async fn prompt_action_for_cmd(ctx: &mut ExecContext, command: &str) -> Result<()> {
    println!(
        "Command to execute: {}",
        dialoguer::console::style(command).green()
    );

    let option = Input::with_theme(&ColorfulTheme::default())
        .with_prompt("[R]un, [M]odify, [I]terate, [C]opy, [A]bort")
        .validate_with(|input: &String| {
            if OPTIONS.contains(&input.to_lowercase().as_str()) {
                Ok(())
            } else {
                Err("Please enter 'r', 'm', 'i', 'c', or 'a'")
            }
        })
        .interact_text()?;

    match option.to_lowercase().as_str() {
        RUN => run_cmd(command),
        MODIFY => match Editor::new().edit(command) {
            Ok(Some(rv)) => {
                // Update the last assistant message with the modified command
                if let Some(Message::Assistant(msg)) = ctx.messages.last_mut() {
                    *msg = rv.clone();
                }
                Box::pin(prompt_action_for_cmd(ctx, &rv)).await
            }
            Ok(None) => {
                println!("{} Aborted", style("✖").red());
                Ok(())
            }
            Err(e) => {
                eprintln!("{} Failed to open editor: {}", style("✖").red(), e);
                Ok(())
            }
        },
        ITERATE => {
            let feedback: String = Input::with_theme(&ColorfulTheme::default())
                .with_prompt("How should I modify the command?")
                .interact_text()?;

            // Add user feedback to conversation history
            ctx.messages.push(Message::User(feedback));

            // Generate new command based on conversation history
            let new_cmd = ctx
                .processor
                .chat(&ctx.messages, &ctx.model, ctx.temperature)
                .await?;

            // Add the new response to history
            ctx.messages.push(Message::Assistant(new_cmd.clone()));

            Box::pin(prompt_action_for_cmd(ctx, &new_cmd)).await
        }
        COPY => {
            copy_to_clipboard(command)?;
            println!("{} Copied to clipboard", style("✔").green());
            Ok(())
        }
        ABORT => Ok(()),
        _ => unreachable!(),
    }
}
