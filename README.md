# acr - Anthropic cli by Rust

Call Anthropic API from cli.

<https://docs.anthropic.com/en/api/messages>

## install

```shell
# install from this repo
cargo install --git https://github.com/ryokryok/anthropic-cli-rs.git

# check
acr -h
```

## setup

This tool requires a `.env` file to execute properly.

Create a file named `.env` in the root directory of the project.

```shell
touch .env
```

`.env` is following format. replace `<YOUR_API_KEY>` with your Anthropic API keys.

```ini
API_KEY=<YOUR_API_KEY>
```

<https://console.anthropic.com/settings/keys>

## example

```shell
acr -p "Hi, please tell me about you"
> Hi, please tell me about you
Hello! I'm an AI assistant created by Anthropic to be helpful, harmless, and honest. I don't have a physical form or avatar - I'm a language model trained to engage in conversations and help with tasks. I have broad knowledge that I can use to assist with all kinds of questions and topics, but my knowledge has a cutoff date and I can't access external information. I'm here to help in whatever way I can, while being clear that I'm an AI, not a human. Please let me know if you have any other questions!
```

## license

MIT
