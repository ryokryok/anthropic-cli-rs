use reqwest::{
    header::{HeaderMap, CONTENT_TYPE},
    Client, Response,
};

use crate::{models::*, ANTHROPIC_URL};

fn build_client(api_key: &str) -> Result<reqwest::Client, reqwest::Error> {
    let mut headers: HeaderMap = HeaderMap::new();
    headers.insert("x-api-key", api_key.parse().unwrap());
    headers.insert("anthropic-version", "2023-06-01".parse().unwrap());
    headers.insert(CONTENT_TYPE, "application/json".parse().unwrap());

    let client = Client::builder().default_headers(headers).build()?;

    Ok(client)
}

async fn parse_response_body(response: Response) -> Result<AnthropicResponse, reqwest::Error> {
    if response.status().is_success() {
        let success_body: SuccessResponse = response.json().await?;
        Ok(AnthropicResponse::Success(success_body))
    } else {
        let error_body: ErrorResponse = response.json().await?;
        Ok(AnthropicResponse::Error(error_body))
    }
}

pub struct Anthropic {
    client: Client,
    base_url: String,
}

impl Anthropic {
    pub fn new(api_key: &str) -> Result<Self, reqwest::Error> {
        let client = build_client(api_key)?;

        Ok(Anthropic {
            client,
            base_url: ANTHROPIC_URL.to_string(),
        })
    }

    pub fn base_url(mut self, base_url: &str) -> Self {
        self.base_url = base_url.to_string();
        self
    }

    pub async fn send(
        &self,
        params: &AnthropicRequest,
    ) -> Result<AnthropicResponse, reqwest::Error> {
        let response = self
            .client
            .post(format!("{}/v1/messages", self.base_url))
            .json(params)
            .send()
            .await?;

        let response_body = parse_response_body(response).await?;
        Ok(response_body)
    }
}

#[cfg(test)]
mod tests {
    use serde_json::json;

    use super::*;

    #[tokio::test]
    async fn test_success_request() {
        let mut server = mockito::Server::new_async().await;
        let url = server.url();

        let response_body = json!({
            "content": [
                {
                    "text": "Hi! My name is Claude.",
                    "type": "text"
                }
            ],
            "id": "msg_013Zva2CMHLNnXjNJJKqJ2EF",
            "model": "claude-3-5-sonnet-20240620",
            "role": "assistant",
            "stop_reason": "end_turn",
            "stop_sequence": null,
            "type": "message",
            "usage": {
                "input_tokens": 2095,
                "output_tokens": 503
            }
        });

        let _mock = server
            .mock("POST", "/v1/messages")
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(response_body.to_string())
            .create_async()
            .await;

        let client = Anthropic::new("foobar").unwrap().base_url(&url);

        let params = AnthropicRequest::new("claude-3-5-sonnet-20240620", 1024)
            .message(MessageParam::new("user").text("Hello, world"));

        let result = client.send(&params).await.unwrap();

        match result {
            AnthropicResponse::Success(success) => {
                let content = success.content.first().unwrap();
                assert_eq!(content.text, "Hi! My name is Claude.");
            }
            AnthropicResponse::Error(_) => {
                panic!("The response should be success");
            }
        }
    }

    #[tokio::test]
    async fn test_error_request() {
        let mut server = mockito::Server::new_async().await;
        let url = server.url();

        let response_body = json!({
            "type": "error",
            "error": {
                "type": "invalid_request_error",
                "message": "<string>"
            }
        }
        );

        let _mock = server
            .mock("POST", "/v1/messages")
            .with_status(400)
            .with_header("content-type", "application/json")
            .with_body(response_body.to_string())
            .create_async()
            .await;

        let client = Anthropic::new("foobar").unwrap().base_url(&url);

        let params = AnthropicRequest::new("claude-3-5-sonnet-20240620", 1024)
            .message(MessageParam::new("user").text("Hello, world"));

        let result = client.send(&params).await.unwrap();

        match result {
            AnthropicResponse::Success(_) => {
                panic!("The response should be error");
            }
            AnthropicResponse::Error(error) => {
                assert_eq!(error.error_type, "error");
                assert_eq!(error.error.error_type, "invalid_request_error");
                assert_eq!(error.error.message, "<string>");
            }
        }
    }
}
