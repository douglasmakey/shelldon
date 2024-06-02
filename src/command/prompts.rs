use crate::config::Config;
use crate::{Error, Result};
use clap::Parser;
use comfy_table::modifiers::UTF8_ROUND_CORNERS;
use comfy_table::presets::UTF8_FULL;
use comfy_table::{ContentArrangement, Table};
use dialoguer::{console::style, Editor};

#[derive(Debug, Parser)]
pub struct PromptsArgs {
    #[clap(subcommand)]
    cmd: Command,
}

#[derive(Debug, Parser)]
pub struct NameArgs {
    #[clap(name = "Name of the prompt")]
    name: String,
}

#[derive(Debug, Parser)]
pub enum Command {
    #[clap(about = "Create a new prompt")]
    Create,
    #[clap(about = "Edit an existing prompt")]
    Edit(NameArgs),
    #[clap(about = "List all prompts")]
    List,
    #[clap(about = "Delete an existing prompt")]
    Delete(NameArgs),
}

pub async fn handle_prompts(config: Config, args: PromptsArgs) -> Result<()> {
    match args.cmd {
        Command::Create => {
            let name = dialoguer::Input::<String>::new()
                .with_prompt("Name of the prompt")
                .interact()?;

            if config.load_prompt(name.as_str()).is_some() {
                println!("{} Prompt already exists", style("✖").red());
                Err(Error::PromptAlreadyExists { name: name.clone() })?;
            }

            if let Some(new_content) = Editor::new().edit("").unwrap() {
                config.save_prompt(&name, new_content.as_str())?;
                println!("{} Prompt created", style("✔").green());
            }
        }
        Command::Edit(args) => {
            let prompt = match config.load_prompt(&args.name) {
                Some(prompt) => prompt,
                None => {
                    println!("{} Prompt not found", style("✖").red());
                    Err(Error::PromptNotFound {
                        name: args.name.clone(),
                    })?
                }
            };

            if let Some(new_content) = Editor::new().edit(&prompt.content).unwrap() {
                config.save_prompt(&prompt.name, new_content.as_str())?;
                println!("{} Prompt modified", style("✔").green());
            }
        }
        Command::List => {
            let mut table = Table::new();
            table
                .set_header(vec!["Name", "Content", "Variables"])
                .load_preset(UTF8_FULL)
                .apply_modifier(UTF8_ROUND_CORNERS)
                .set_content_arrangement(ContentArrangement::Dynamic);

            let prompts = config.load_prompts()?;
            for prompt in prompts {
                let values = prompt
                    .values
                    .iter()
                    .map(|v| v.name.clone())
                    .collect::<Vec<String>>()
                    .join(", ");

                table.add_row(&[prompt.name, prompt.content, values]);
            }
            println!("{}", table);
        }
        Command::Delete(args) => {
            let name = args.name;
            if config.load_prompt(&name).is_none() {
                println!("{} Prompt not found", style("✖").red());
                Err(Error::PromptNotFound { name: name.clone() })?;
            }

            config.delete_prompt(&name)?;
            println!("{} Prompt deleted", style("✔").green());
        }
    }

    Ok(())
}
