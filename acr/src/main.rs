use anthropic::{
    request::{AnthropicRequest, Message, MessageContent, Role},
    response::AnthropicResponse,
    Anthropic,
};
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
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let args = Args::parse();

    dotenv()
        .map_err(|_| "Failed to load .env file. Please ensure it exists in the project root.")?;

    let api_key = env::var("API_KEY").map_err(|_| "API_KEY not found in .env file")?;

    println!("> {}", args.prompt);

    let params = AnthropicRequest::new(
        "claude-3-5-sonnet-20240620",
        1024,
        vec![Message::new(Role::User, MessageContent::new(&args.prompt))],
    );

    let client = Anthropic::new(&api_key)?;

    let result = client.send(&params).await?;

    match result {
        AnthropicResponse::Message(success) => {
            if let Some(content) = success.content.first() {
                println!("{}", content.text);
            } else {
                println!("Received empty response from Claude");
            }
        }
        AnthropicResponse::Error(error) => eprintln!("Error from Claude: {:#?}", error),
    }

    Ok(())
}
