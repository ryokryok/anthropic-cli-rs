use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct MessageParam {
    role: String,
    content: String,
}

impl MessageParam {
    pub fn new(role: &str, content: &str) -> Self {
        MessageParam {
            role: role.to_string(),
            content: content.to_string(),
        }
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct AnthropicRequest {
    // https://docs.anthropic.com/en/docs/about-claude/models#model-names
    model: String,
    max_tokens: usize,
    messages: Vec<MessageParam>,
}

impl AnthropicRequest {
    pub fn new(model: &str, max_tokens: usize, messages: Vec<MessageParam>) -> Self {
        AnthropicRequest {
            model: model.to_string(),
            max_tokens,
            messages,
        }
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Content {
    pub text: String,
    #[serde(rename = "type")]
    pub content_type: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Usage {
    pub input_tokens: usize,
    pub output_tokens: usize,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct SuccessResponse {
    pub content: Vec<Content>,
    pub id: String,
    pub model: String,
    pub role: String,
    pub stop_reason: String,
    pub stop_sequence: Option<String>,
    #[serde(rename = "type")]
    pub response_type: String,
    pub usage: Usage,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ErrorInfo {
    #[serde(rename = "type")]
    pub error_type: String,
    pub message: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ErrorResponse {
    #[serde(rename = "type")]
    pub error_type: String,
    pub error: ErrorInfo,
}

pub enum AnthropicResponse {
    Success(SuccessResponse),
    Error(ErrorResponse),
}
