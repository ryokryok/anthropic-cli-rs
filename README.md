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

### simple text

```shell
acr -p "Hi, please tell me about you"
# Hi, please tell me about you

Hello! I'm an AI assistant created by Anthropic to be helpful, ...
```

### describe image

```shell
acr -p "What is in this image?" -i image.jpg
# What is in this image?

This image is an iconic scene from ...
```

## license

MIT
