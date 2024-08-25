use reqwest::{
    header::{HeaderMap, CONTENT_TYPE},
    Client, Response,
};

use crate::constants::ANTHROPIC_URL;
use crate::models::{request, response};

fn build_client(api_key: &str) -> Result<reqwest::Client, reqwest::Error> {
    let mut headers: HeaderMap = HeaderMap::new();
    headers.insert("x-api-key", api_key.parse().unwrap());
    headers.insert("anthropic-version", "2023-06-01".parse().unwrap());
    headers.insert(CONTENT_TYPE, "application/json".parse().unwrap());

    let client = Client::builder().default_headers(headers).build()?;

    Ok(client)
}

async fn parse_response_body(
    response: Response,
) -> Result<response::AnthropicResponse, reqwest::Error> {
    if response.status().is_success() {
        let success_body: response::MessageResponse = response.json().await?;
        Ok(response::AnthropicResponse::Message(success_body))
    } else {
        let error_body: response::ErrorResponse = response.json().await?;
        Ok(response::AnthropicResponse::Error(error_body))
    }
}

pub struct Anthropic {
    client: Client,
}

impl Anthropic {
    pub fn new(api_key: &str) -> Result<Self, reqwest::Error> {
        let client = build_client(api_key)?;

        Ok(Anthropic { client })
    }

    pub async fn send(
        &self,
        params: &request::AnthropicRequest,
    ) -> Result<response::AnthropicResponse, reqwest::Error> {
        let response = self
            .client
            .post(format!("{}/v1/messages", ANTHROPIC_URL))
            .json(params)
            .send()
            .await?;

        let response_body = parse_response_body(response).await?;
        Ok(response_body)
    }
}
