use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, PartialEq, Eq)]
pub struct MessageParam {
    role: String,
    content: Vec<ContentItem>,
}

#[derive(Debug, Deserialize, Serialize, PartialEq, Eq)]
pub struct ContentItem {
    #[serde(rename = "type")]
    item_type: String,
    #[serde(flatten)]
    data: ContentData,
}

#[derive(Debug, Deserialize, Serialize, PartialEq, Eq)]
#[serde(untagged)]
pub enum ContentData {
    Text { text: String },
    Image { source: ImageSource },
}

#[derive(Debug, Deserialize, Serialize, PartialEq, Eq)]
pub struct ImageSource {
    #[serde(rename = "type")]
    source_type: String,
    media_type: String,
    data: String,
}

impl MessageParam {
    pub fn new(role: &str) -> Self {
        let content: Vec<ContentItem> = Vec::new();
        MessageParam {
            role: role.to_string(),
            content,
        }
    }

    pub fn text(mut self, text: &str) -> Self {
        self.content.push(ContentItem {
            item_type: "text".to_string(),
            data: ContentData::Text {
                text: text.to_string(),
            },
        });
        self
    }

    pub fn image(mut self, media_type: &str, data: &str) -> Self {
        self.content.push(ContentItem {
            item_type: "image".to_string(),
            data: ContentData::Image {
                source: ImageSource {
                    source_type: "base64".to_string(),
                    media_type: media_type.to_string(),
                    data: data.to_string(),
                },
            },
        });
        self
    }
}

#[derive(Debug, Deserialize, Serialize, PartialEq, Eq)]
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

#[derive(Debug, Deserialize, Serialize, PartialEq, Eq)]
pub struct Content {
    pub text: String,
    #[serde(rename = "type")]
    pub content_type: String,
}

#[derive(Debug, Deserialize, Serialize, PartialEq, Eq)]
pub struct Usage {
    pub input_tokens: usize,
    pub output_tokens: usize,
}

#[derive(Debug, Deserialize, Serialize, PartialEq, Eq)]
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simple_message_deserialize() {
        let json = r#"{
            "role": "user",
            "content": [
                {
                    "type": "text",
                    "text": "Hello, world"
                }
            ] 
        }"#;
        let result = serde_json::from_str::<MessageParam>(json).unwrap();
        assert_eq!(result, MessageParam::new("user").text("Hello, world"));
    }

    #[test]
    fn test_complex_message_deserialize() {
        let json = r#"{
            "role": "user",
            "content": [
                {
                    "type": "image",
                    "source": {
                        "type": "base64",
                        "media_type": "image/jpeg",
                        "data": "/9j/4AAQSkZJRg..."
                    }
                },
                {
                    "type": "text",
                    "text": "What is in this image?"
                }
            ]
        }"#;
        let result: MessageParam = serde_json::from_str::<MessageParam>(json).unwrap();
        assert_eq!(
            result,
            MessageParam::new("user")
                .image("image/jpeg", "/9j/4AAQSkZJRg...")
                .text("What is in this image?")
        );
    }

    #[test]
    fn test_request_deserialize() {
        let json = r#"{
            "model": "claude-3-5-sonnet-20240620",
            "max_tokens": 1024,
            "messages": [
                {
                    "role": "user",
                    "content": [
                        {
                            "type": "text",
                            "text": "Hello, world"
                        }
                    ]
                }
            ]
        }"#;
        let result = serde_json::from_str::<AnthropicRequest>(json).unwrap();
        assert_eq!(
            result,
            AnthropicRequest {
                model: "claude-3-5-sonnet-20240620".to_string(),
                max_tokens: 1024,
                messages: vec![MessageParam {
                    role: "user".to_string(),
                    content: vec![ContentItem {
                        item_type: "text".to_string(),
                        data: ContentData::Text {
                            text: "Hello, world".to_string()
                        }
                    }]
                }]
            }
        );
        // create struct from new method
        assert_eq!(
            result,
            AnthropicRequest::new(
                "claude-3-5-sonnet-20240620",
                1024,
                vec![MessageParam::new("user").text("Hello, world")]
            )
        );
    }

    #[test]
    fn test_response_deserialize() {
        let json = r#"{
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
        }"#;
        let result = serde_json::from_str::<SuccessResponse>(json).unwrap();
        assert_eq!(
            result,
            SuccessResponse {
                content: vec![Content {
                    text: "Hi! My name is Claude.".to_string(),
                    content_type: "text".to_string()
                }],
                id: "msg_013Zva2CMHLNnXjNJJKqJ2EF".to_string(),
                model: "claude-3-5-sonnet-20240620".to_string(),
                role: "assistant".to_string(),
                stop_reason: "end_turn".to_string(),
                stop_sequence: None,
                response_type: "message".to_string(),
                usage: Usage {
                    input_tokens: 2095,
                    output_tokens: 503
                }
            }
        );
    }
}
