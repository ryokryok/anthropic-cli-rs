# acr - Anthropic cli by Rust

Call Anthropic API from cli.

<https://docs.anthropic.com/en/api/messages>

## prepare

```shell
mv .env.sample .env
```

and replace `<YOUR_API_KEY>` with your Anthropic API keys.

<https://console.anthropic.com/settings/keys>

## CLI

```shell
# from project root
cargo run --bin cli -- -p "Write react component for display tweet"
```

## license

MIT
