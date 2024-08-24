use anthropic::*;
use clap::Parser;
use dotenvy::dotenv;
use std::{env, error::Error};

/// Simple program to use Claude
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Prompts to input to AI
    #[arg(short, long)]
    prompt: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    dotenv().ok();
    let api_key = env::var("API_KEY").unwrap();

    let args = Args::parse();

    println!("> {}", args.prompt);

    let params = MessageCreateParams::new(
        "claude-3-5-sonnet-20240620",
        1024,
        vec![MessageParam::new("user", &args.prompt)],
    );

    let client = Anthropic::new(&api_key)?;

    let result = client.send(&params).await?;

    match result {
        Message::Success(success) => println!("{}", success.content[0].text),
        Message::Error(error) => println!("{:#?}", error),
    }

    Ok(())
}
