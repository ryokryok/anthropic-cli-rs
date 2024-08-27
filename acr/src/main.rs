use anthropic::*;
use clap::Parser;
use dotenvy::dotenv;
use std::{env, error::Error};

/// Call Anthropic API from cli.
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Prompts to input to AI
    #[arg(short, long)]
    prompt: String,
    /// AI model name.
    #[arg(short, long, default_value = "claude-3-5-sonnet-20240620")]
    model: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let args = Args::parse();

    dotenv()
        .map_err(|_| "Failed to load .env file. Please ensure it exists in the project root.")?;

    let api_key = env::var("API_KEY").map_err(|_| "API_KEY not found in .env file")?;

    let params = MessageCreateParams::new(
        &args.model,
        1024,
        vec![MessageParam::new("user", &args.prompt)],
    );

    let client = Anthropic::new(&api_key, ANTHROPIC_URL)?;

    let result = client.send(&params).await?;

    match result {
        Message::Success(success) => {
            if let Some(content) = success.content.first() {
                println!(
                    "# {prompt}\n\n{content}",
                    prompt = args.prompt,
                    content = content.text
                );
            } else {
                println!("Received empty response from Claude");
            }
        }
        Message::Error(error) => eprintln!("Error from Claude: {:#?}", error),
    }

    Ok(())
}
