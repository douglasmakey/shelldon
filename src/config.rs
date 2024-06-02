use crate::Result;
use serde::{Deserialize, Serialize};
use std::{
    fs,
    path::{Path, PathBuf},
};

use regex::Regex;

const SHELLDON: &str = "shelldon";

#[derive(Serialize, Deserialize, Debug)]
pub struct PromptValue {
    pub name: String,
    pub value: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Prompt {
    pub name: String,
    pub content: String,
    pub values: Vec<PromptValue>,
}

pub struct Config {
    config_dir: PathBuf,
    prompts_dir: PathBuf,
}

impl Config {
    pub fn new() -> Self {
        let config_dir = dirs::config_dir()
            .expect("Could not find configuration directory")
            .join(SHELLDON);

        let prompts_dir = config_dir.join("prompts");
        Self {
            config_dir,
            prompts_dir,
        }
    }

    pub fn initialize(&self) {
        self.ensure_dir_exists(&self.config_dir);
        self.ensure_dir_exists(&self.prompts_dir);
    }

    fn ensure_dir_exists(&self, path: &Path) {
        if !path.exists() {
            fs::create_dir_all(path).expect("Failed to create directory");
        }
    }

    pub fn save_prompt(&self, name: &str, content: &str) -> Result<()> {
        let mut prompt_dir = self.prompts_dir.join(name);
        prompt_dir.set_extension("json");
        let prompt = parse_prompt(name, content)?;
        let prompt_json = serde_json::to_string_pretty(&prompt)?;
        fs::write(prompt_dir, prompt_json)?;
        Ok(())
    }

    pub fn delete_prompt(&self, name: &str) -> Result<()> {
        let mut prompt_dir = self.prompts_dir.join(name);
        prompt_dir.set_extension("json");
        fs::remove_file(prompt_dir)?;
        Ok(())
    }

    pub fn load_prompt(&self, name: &str) -> Option<Prompt> {
        let mut prompt_dir = self.prompts_dir.join(name);
        prompt_dir.set_extension("json");
        println!("{:?}", prompt_dir);
        let prompt_json = fs::read_to_string(prompt_dir).ok()?;
        serde_json::from_str(&prompt_json).unwrap()
    }

    pub fn load_prompts(&self) -> Result<Vec<Prompt>> {
        let mut prompts = Vec::new();

        for entry in fs::read_dir(&self.prompts_dir)? {
            let entry = entry?;
            let prompt_json = fs::read_to_string(entry.path())?;
            let prompt: Prompt = serde_json::from_str(&prompt_json)?;
            prompts.push(prompt);
        }

        Ok(prompts)
    }
}

pub fn parse_prompt(name: &str, content: &str) -> Result<Prompt> {
    let re = Regex::new(r"\{(\w+):(\w+)\}")?;
    let mut values = Vec::new();

    for cap in re.captures_iter(content) {
        values.push(PromptValue {
            name: cap[1].to_string(),
            value: cap[2].to_string(),
        });
    }

    Ok(Prompt {
        name: name.to_string(),
        content: content.to_string(),
        values,
    })
}
