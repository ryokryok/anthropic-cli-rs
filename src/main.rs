use anthropic_cli_rs::anthropic::{Anthropic, Message, MessageCreateParams, MessageParam};
use dotenvy::dotenv;
use reqwest;
use std::env;

#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {
    dotenv().ok();

    let api_key = env::var("API_KEY").unwrap();

    let anthropic = Anthropic::new(&api_key)?;

    let params = MessageCreateParams::new(
        "claude-3-5-sonnet-20240620",
        1024,
        vec![MessageParam::new("user", "Hello, world")],
    );

    let result = anthropic.send(params).await?;

    match result {
        Message::Success(success) => println!("{:#?}", success),
        Message::Error(error) => println!("{:#?}", error),
    }

    Ok(())
}
