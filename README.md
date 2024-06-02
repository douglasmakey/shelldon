# Shelldon

![](./assets/shelldon.jpeg)

Shelldon is a command-line tool written in Rust. It provides a set of utilities for executing shell commands, managing prompts, and interacting with OpenAI GPT.

Yes, another CLI with GPT features. Shelldon is not intended to be a full GPT client from the terminal; there are a couple of CLIs much better for that and also a lot of applications and even the OpenAI ChatGPT apps. Shelldon is to solve some personal use cases and it is very useful for me; I hope it could be useful for you too. Also, I made it to have fun playing with Rust!

> [!IMPORTANT]
> One of the features that other tools were missing for my use case is the ability to use custom prompts for specific tasks that I need. For that reason, I created Shelldon with the ability to manage prompts and use them whenever you want for specific tasks. You can read more about this [here](https://github.com/douglasmakey/shelldon?tab=readme-ov-file#handling-prompts). Also, I plan to extend it with plugins to integrate more complex workflows.

## Installation

### Building from Source

If you prefer to build Shelldon from source, you can clone the repository and build it using cargo:

```sh
git clone https://github.com/douglasmakey/shelldon.git
cd shelldon
cargo build --release
```

## Usage

To use Shelldon, you’ll need to set your OpenAI token. You can do this by setting an environment variable. Here’s how you can set it in your terminal:

```sh
export OPENAI_API_KEY="your-openai-api-key"
```

Shelldon allows you to integrate GPT features into your shell commands easily. Here are some examples to get you started:

### Running Shell Commands

```sh
$ shelldon "Show all the graphics ports for the Vagrant machine using Libvirt."
Command to execute: vagrant ssh -c "virsh list --all | grep vagrant | awk '{print \$1}' | xargs -I {} virsh domdisplay {}"
? [R]un, [M]odify, [C]opy, [A]bort › 
```

**Analyzing Docker Logs**

Use Shelldon to analyze Docker logs and identify errors:

```sh
$ docker logs nginx | shelldon "check logs, find errors"
```

**Troubleshooting Kubernetes**

Shelldon can help you understand why a Kubernetes pod is failing:

```sh
$ k describe pod nginx | shelldon ask "why this pod is failing?"
The pod is failing because it was terminated due to an "Out of Memory" (OOM) condition. The `OOMKilled` reason indicates that the container running in the pod exceeded its memory limit, causing the system to kill the process to prevent it from affecting other processes on the node.

Here are some steps you can take to address this issue:
...
```

**Generate configuration files with the help of GPT:**

```sh
$ shelldon "Create a basic nginx configuration file"
Configuration file content:
server {
    listen 80;
    server_name example.com;

    location / {
        proxy_pass http://localhost:3000;
        proxy_set_header Host $host;
        proxy_set_header X-Real-IP $remote_addr;
        proxy_set_header X-Forwarded-For $proxy_add_x_forwarded_for;
        proxy_set_header X-Forwarded-Proto $scheme;
    }
}
...
```

**Automate routine system tasks with ease:**

```sh
$ shelldon exec "Find and delete all log files older than 30 days in /var/log"
Command to execute: find /var/log -name "*.log" -type f -mtime +30 -exec rm {} \;
? [R]un, [M]odify, [C]opy, [A]bort › 
```

**Get help with writing meaningful Git commit messages:**

```sh
$ git diff | shelldon "Generate a commit message" --copy
"Refactor logging system to improve error handling and performance. This change updates the logging library and adjusts the log levels for better clarity."
```

You can use the `--copy` command to copy the output directly to your clipboard.

### Handling Prompts

Shelldon allows you to create, edit, list, and delete custom prompts to streamline your command-line workflows. Here’s how you can manage your prompts:

**Command Overview**

```sh
$ shelldon prompts -h
Usage: shelldon prompts <COMMAND>

Commands:
  create  Create a new prompt
  edit    Edit an existing prompt
  list    List all prompts
  delete  Delete an existing prompt
  help    Print this message or the help of the given subcommand(s)

Options:
  -h, --help  Print help
```

**Listing Prompts**

To view all the prompts you have created, use the list command:

```sh
$ shelldon prompts list

╭────────────┬─────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────┬───────────╮
│ Name       ┆ Content                                                                                                                                                                                                                                                 ┆ Variables │
╞════════════╪═════════════════════════════════════════════════════════════════════════════════════════════════════════════════════════════════════════════════════════════════════════════════════════════════════════════════════════════════════════════════════════╪═══════════╡
│ script     ┆ Let’s think step by step and act as a {script:bash} code scripts expert. Provide only the {script} script code as output without any descriptions or explanations. Ensure the output is in plain text format without Markdown formatting or symbols. If ┆ script    │
│            ┆ details are insufficient, provide the most logical solution. You are not allowed to ask for more details. Just print the script directly.                                                                                                               ┆           │
├╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌╌╌┤
│ translator ┆ Let’s think step by step and act as a translator. Translate the following text from {from:english} to {to:spanish}. Make it sound natural to a native speaker of {to} while keeping the original tone. Do only minimal edits without changing the tone. ┆ from, to  │
│            ┆ Avoid using fancy words. Reply with only the translated text and nothing else. Do not provide explanations.                                                                                                                                             ┆           │
├╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌╌╌┤
│ note-taker ┆ I am software engineer and I’d like you to look at the following text I wrote and edit it to make it sound more natural to a native English speaker. Do only minimal/minor edits without changing the tone of the text, which should remain the same.   ┆           │
│            ┆ Dont use fancy words and I want you to only reply the correction, the improvements and nothing else, do not write explanations.                                                                                                                         ┆           │
╰────────────┴─────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────┴───────────╯
```

You can use the `{}` notation to add variables to the prompt, and you can override those values using the `--set key=value` option. Additionally, in the prompt template, you can define default values like `{from:spanish}`. This allows for flexible and dynamic prompts that can be customized based on user input.

Then, you can run the ask command with a defined template:

In my daily routine, I often need to generate bash and python scripts or cloud-init files. `shelldon` helps me with this task:

```sh
shelldon ask --prompt script "Generate a cloud-init script to set up an Ubuntu server with the following steps:
1. Update Ubuntu.
2. Install Nginx.
3. Create a custom HTML file to be served by Nginx.
4. Ensure Nginx is enabled and started." --set script=cloud-init > cloud-init
```

As you can see, we can redirect the output directly to a file to create the script.

Or some translations.

```sh
alias sat="shelldon ask --prompt translator"
sat "Hey guys, I'm a few minutes late for the meeting, in less than 5 minutes I'll be there."
Hola chicos, voy unos minutos tarde para la reunión, en menos de 5 minutos estaré ahí.
```

You can also modify the values for the template:

```sh
alias sat="shelldon ask --prompt translator"
sat "Chicos voy a llegar 5 minutos tarde a la reunion" --set to=english --set from=spanish
Guys, I'm going to be 5 minutes late to the meeting.
```

So the ability to handle dynamic prompts with args and use them makes Shelldon a useful tool for me.

## TODO

- [ ] Implement the best way to print the output nicely.
- [ ] Add tests.
- [ ] Improve error handling.
- [ ] Add default prompts.
- [ ] Implement OpenAI functions?
- [ ] Implement Ollama? Maybe in the future. Do you need it?

## Contributing

Contributions, suggestions, and discussions are welcome! Whether you're enhancing the functionality, refining the concept, or fixing bugs, your input is valuable.


