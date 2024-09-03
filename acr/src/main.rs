use anthropic::*;
use base64::{engine::general_purpose, Engine};
use clap::Parser;
use dotenvy::dotenv;
use std::{env, error::Error, fs::File, io::Read};

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
    /// Image
    #[arg(short, long)]
    image: Option<String>,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let args = Args::parse();

    dotenv()
        .map_err(|_| "Failed to load .env file. Please ensure it exists in the project root.")?;

    let api_key = env::var("API_KEY").map_err(|_| "API_KEY not found in .env file")?;

    let messages = match args.image {
        Some(path) => {
            let mut buffer = Vec::new();
            File::open(&path)?
                .read_to_end(&mut buffer)
                .expect("Error: failed to read file.");
            let file_type = infer::get(&buffer).unwrap();
            let mime_type = file_type.mime_type();
            let base64_string = general_purpose::STANDARD.encode(&buffer);

            MessageParam::new("user")
                .image(&mime_type, base64_string)
                .text(&args.prompt)
        }
        None => MessageParam::new("user").text(&args.prompt),
    };

    let params = AnthropicRequest::new(&args.model, 1024).message(messages);

    let result = Anthropic::new(&api_key)?.send(&params).await?;

    match result {
        AnthropicResponse::Success(success) => {
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
        AnthropicResponse::Error(error) => eprintln!("Error from Claude: {:#?}", error),
    }

    Ok(())
}
