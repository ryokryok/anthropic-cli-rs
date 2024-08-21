pub mod v1 {
    pub mod message {
        use reqwest::{
            header::{HeaderMap, CONTENT_TYPE},
            Client, Response,
        };
        use serde::{Deserialize, Serialize};

        #[derive(Debug, Deserialize, Serialize)]
        pub struct Message {
            role: String,
            content: String,
        }

        impl Message {
            pub fn new(role: &str, content: &str) -> Self {
                Message {
                    role: role.to_string(),
                    content: content.to_string(),
                }
            }
        }

        #[derive(Debug, Deserialize, Serialize)]
        pub struct RequestBody {
            // https://docs.anthropic.com/en/docs/about-claude/models#model-names
            model: String,
            max_tokens: usize,
            messages: Vec<Message>,
        }

        impl RequestBody {
            pub fn new(model: &str, max_tokens: usize, messages: Vec<Message>) -> Self {
                RequestBody {
                    model: model.to_string(),
                    max_tokens,
                    messages,
                }
            }
        }

        #[derive(Debug, Deserialize, Serialize)]
        struct Content {
            text: String,
            #[serde(rename = "type")]
            content_type: String,
        }

        #[derive(Debug, Deserialize, Serialize)]
        struct Usage {
            input_tokens: usize,
            output_tokens: usize,
        }

        #[derive(Debug, Deserialize, Serialize)]
        pub struct SuccessResponseBody {
            content: Vec<Content>,
            id: String,
            model: String,
            role: String,
            stop_reason: String,
            stop_sequence: Option<String>,
            #[serde(rename = "type")]
            response_type: String,
            usage: Usage,
        }

        #[derive(Debug, Deserialize, Serialize)]
        struct ErrorInfo {
            #[serde(rename = "type")]
            error_type: String,
            message: String,
        }

        #[derive(Debug, Deserialize, Serialize)]
        pub struct ErrorResponseBody {
            #[serde(rename = "type")]
            error_type: String,
            error: ErrorInfo,
        }

        pub enum ApiResponse {
            Success(SuccessResponseBody),
            Error(ErrorResponseBody),
        }

        const ANTHROPIC_URL: &str = "https://api.anthropic.com";

        pub struct ApiClient {
            client: Client,
        }

        impl ApiClient {
            pub fn new(api_key: &str) -> Result<Self, reqwest::Error> {
                let client = build_client(api_key)?;
                Ok(ApiClient { client })
            }

            pub async fn send(
                &self,
                request_body: RequestBody,
            ) -> Result<ApiResponse, reqwest::Error> {
                let response = self
                    .client
                    .post(format!("{}/v1/messages", ANTHROPIC_URL))
                    .json(&request_body)
                    .send()
                    .await?;

                let response_body = parse_response_body(response).await?;
                Ok(response_body)
            }
        }

        fn build_client(api_key: &str) -> Result<reqwest::Client, reqwest::Error> {
            let mut headers: HeaderMap = HeaderMap::new();
            headers.insert("x-api-key", api_key.parse().unwrap());
            headers.insert("anthropic-version", "2023-06-01".parse().unwrap());
            headers.insert(CONTENT_TYPE, "application/json".parse().unwrap());

            let client = Client::builder().default_headers(headers).build()?;

            Ok(client)
        }

        async fn parse_response_body(response: Response) -> Result<ApiResponse, reqwest::Error> {
            if response.status().is_success() {
                let success_body: SuccessResponseBody = response.json().await?;
                Ok(ApiResponse::Success(success_body))
            } else {
                let error_body: ErrorResponseBody = response.json().await?;
                Ok(ApiResponse::Error(error_body))
            }
        }
    }
}
