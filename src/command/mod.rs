mod ask;
mod exec;
mod prompts;

pub use self::ask::*;
pub use self::exec::*;
pub use self::prompts::*;

use crate::{
    config::{Config, PromptValue},
    Result,
};
use atty::Stream;
use clap::Parser;
use regex::Regex;
use std::result::Result as StdResult;
use std::{
    collections::HashMap,
    io::{self, Read},
};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct KeyValue {
    key: String,
    value: String,
}

impl From<KeyValue> for PromptValue {
    fn from(kv: KeyValue) -> Self {
        PromptValue {
            name: kv.key,
            value: kv.value,
        }
    }
}

fn parse_prompt(
    config: Config,
    name: Option<String>,
    values: Vec<KeyValue>,
    default_prompt: &str,
) -> Result<String> {
    let name = if let Some(name) = name {
        name
    } else {
        return Ok(default_prompt.to_string());
    };

    let prompt = if let Some(prompt) = config.load_prompt(&name) {
        prompt
    } else {
        return Ok(default_prompt.to_string());
    };

    // Merge the values from `prompt` and `values` into a single vector of `PromptValue`.
    // If `prompt` and `values` contain values with the same name, the value from `values` is used.
    let merged_values: Vec<PromptValue> = prompt
        .values
        .into_iter()
        .map(|pv| (pv.name, pv.value))
        .chain(values.into_iter().map(|kv| (kv.key, kv.value)))
        .collect::<HashMap<_, _>>()
        .into_iter()
        .map(|(name, value)| PromptValue { name, value })
        .collect();

    let mut content = prompt.content;

    for kv in merged_values {
        let re = Regex::new(&format!("\\{{{}(:\\w+)?\\}}", kv.name))?;
        content = re.replace_all(&content, kv.value.as_str()).to_string();
    }

    Ok(content)
}

fn parse_key_val(s: &str) -> StdResult<KeyValue, String> {
    let pos = s
        .find('=')
        .ok_or_else(|| format!("Invalid key=value: no `=` found in `{}`", s))?;
    Ok(KeyValue {
        key: s[..pos].to_string(),
        value: s[pos + 1..].to_string(),
    })
}

#[derive(Parser, Clone)]
pub struct CommonArgs {
    #[clap(
        short,
        long,
        help = "Model name (e.g. gpt-4o)",
        default_value = "gpt-4o"
    )]
    model: String,
    #[clap(
        short,
        long,
        help = "Temperature value to set the randomness of the output",
        default_value = "0.0"
    )]
    temperature: f32,
    #[clap(
        short,
        long,
        help = "Always copy the output to the clipboard",
        default_value = "false"
    )]
    copy: bool,
    #[clap(long, help = "Prompt to use for the completion")]
    prompt: Option<String>,
    #[arg(short, long, value_parser = parse_key_val, number_of_values = 1)]
    set: Vec<KeyValue>,
    #[arg(required = true)]
    input: String,
}

pub fn read_input(input: &str) -> Result<String> {
    let mut buffer = String::new();

    if !atty::is(Stream::Stdin) {
        let mut stdin = io::stdin();
        let mut stdin_buffer = String::new();

        // Read from stdin asynchronously
        stdin.read_to_string(&mut stdin_buffer)?;

        if !stdin_buffer.trim().is_empty() {
            buffer.push_str(&stdin_buffer);
        }
    }

    buffer.push_str(input);
    Ok(buffer)
}
