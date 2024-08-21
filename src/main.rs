use anthropic_cli_rs::anthropic::v1::message::{ApiClient, ApiResponse, Message, RequestBody};
use dotenvy::dotenv;
use reqwest;
use std::env;

#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {
    dotenv().ok();

    let api_key = env::var("API_KEY").unwrap();

    let client = ApiClient::new(&api_key)?;

    let body = RequestBody::new(
        "claude-3-5-sonnet-20240620",
        1024,
        vec![Message::new("user", "Hello, world")],
    );

    let response = client.send(body).await?;

    match response {
        ApiResponse::Success(success) => println!("{:#?}", success),
        ApiResponse::Error(error) => println!("{:#?}", error),
    }

    Ok(())
}
