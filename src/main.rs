mod backend;
mod command;
mod config;
mod error;
mod processor;
mod system;

use dialoguer::console::style;
pub use error::{Error, Result};

use clap::{Parser, Subcommand};
use command::{handle_ask, handle_exec, handle_prompts, AskArgs, ExecArgs, PromptsArgs};
use config::Config;

#[derive(Parser)]
struct App {
    #[clap(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    #[clap(about = "Generate a command to execute")]
    Exec(ExecArgs),
    #[clap(about = "Manage prompts")]
    Prompts(PromptsArgs),
    #[clap(about = "Ask a question")]
    Ask(AskArgs),
}

#[tokio::main]
async fn main() {
    let config = Config::new();
    config.initialize();

    let app = App::parse();
    let result = match app.command {
        Commands::Exec(args) => handle_exec(config, args).await,
        Commands::Prompts(args) => handle_prompts(config, args).await,
        Commands::Ask(args) => handle_ask(config, args).await,
    };

    if let Err(e) = result {
        eprintln!("{} {}", style("✖").red(), e);
        std::process::exit(1);
    }
}
