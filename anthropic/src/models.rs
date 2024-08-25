pub mod request {
    use serde::Serialize;

    #[derive(Serialize, Debug, PartialEq)]
    pub struct AnthropicRequest {
        pub model: String,
        pub max_tokens: u32,
        pub messages: Vec<Message>,
    }

    impl AnthropicRequest {
        pub fn new(model: &str, max_tokens: u32, messages: Vec<Message>) -> Self {
            AnthropicRequest {
                model: model.to_string(),
                max_tokens,
                messages,
            }
        }
    }

    #[derive(Serialize, Debug, PartialEq)]
    pub struct Message {
        pub role: Role,
        pub content: MessageContent,
    }

    impl Message {
        pub fn new(role: Role, content: MessageContent) -> Self {
            Message { role, content }
        }
    }

    #[derive(Serialize, Debug, PartialEq)]
    #[serde(rename_all = "lowercase")]
    pub enum Role {
        User,
        Assistant,
    }

    #[derive(Serialize, Debug, PartialEq)]
    pub enum MessageContent {
        String(String),
        Array(Vec<Content>),
    }

    impl MessageContent {
        pub fn new(text: &str) -> Self {
            MessageContent::String(text.to_string())
        }
    }

    #[derive(Serialize, Debug, PartialEq)]
    #[serde(tag = "type")]
    pub enum Content {
        #[serde(rename = "text")]
        Text { text: String },
        #[serde(rename = "image")]
        Image { source: ImageSource },
    }

    impl Content {
        pub fn text(text: &str) -> Self {
            Content::Text {
                text: text.to_string(),
            }
        }

        pub fn image(media_type: &str, data: &str) -> Self {
            Content::Image {
                source: ImageSource {
                    source_type: "base64".to_string(),
                    media_type: media_type.to_string(),
                    data: data.to_string(),
                },
            }
        }
    }

    #[derive(Serialize, Debug, PartialEq)]
    pub struct ImageSource {
        #[serde(rename = "type")]
        pub source_type: String,
        pub media_type: String,
        pub data: String,
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn test_simple_request_serialization() {
            let request = AnthropicRequest::new(
                "claude-3-5-sonnet-20240620",
                1024,
                vec![Message::new(
                    Role::User,
                    MessageContent::new("Hello, world"),
                )],
            );

            assert_eq!(request.model, "claude-3-5-sonnet-20240620");
            assert_eq!(request.max_tokens, 1024);
            assert_eq!(request.messages[0].role, Role::User);
            assert_eq!(
                request.messages[0].content,
                MessageContent::String("Hello, world".to_string())
            );
        }

        #[test]
        fn test_complex_request_serialization() {
            let request = AnthropicRequest::new(
                "claude-3-5-sonnet-20240620",
                1024,
                vec![Message::new(
                    Role::User,
                    MessageContent::Array(vec![
                        Content::image("image/jpeg", "/9j/4AAQSkZJRg..."),
                        Content::text("What is in this image?"),
                    ]),
                )],
            );

            assert_eq!(request.model, "claude-3-5-sonnet-20240620");
            assert_eq!(request.max_tokens, 1024);
            assert_eq!(request.messages[0].role, Role::User);
            assert_eq!(
                request.messages[0].content,
                MessageContent::Array(vec![
                    Content::Image {
                        source: ImageSource {
                            source_type: "base64".to_string(),
                            media_type: "image/jpeg".to_string(),
                            data: "/9j/4AAQSkZJRg...".to_string(),
                        }
                    },
                    Content::Text {
                        text: "What is in this image?".to_string()
                    }
                ])
            );
        }
    }
}

pub mod response {
    use serde::Deserialize;

    #[derive(Deserialize, Debug)]
    #[serde(tag = "type")]
    pub enum AnthropicResponse {
        #[serde(rename = "message")]
        Message(MessageResponse),
        #[serde(rename = "error")]
        Error(ErrorResponse),
    }

    #[derive(Deserialize, Debug)]
    pub struct MessageResponse {
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

    #[derive(Deserialize, Debug)]
    pub struct Content {
        #[serde(rename = "type")]
        pub content_type: String,
        pub text: String,
    }

    #[derive(Deserialize, Debug)]
    pub struct Usage {
        pub input_tokens: u32,
        pub output_tokens: u32,
    }

    #[derive(Deserialize, Debug)]
    pub struct ErrorResponse {
        pub error: Error,
    }

    #[derive(Deserialize, Debug)]
    pub struct Error {
        #[serde(rename = "type")]
        pub error_type: String,
        pub message: String,
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn test_success_response_deserialization() {
            let json_str = r#"
            {
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
            }
            "#;

            let response = serde_json::from_str::<MessageResponse>(&json_str).unwrap();
            assert_eq!(response.content[0].text, "Hi! My name is Claude.");
            assert_eq!(response.content[0].content_type, "text");
            assert_eq!(response.id, "msg_013Zva2CMHLNnXjNJJKqJ2EF");
            assert_eq!(response.model, "claude-3-5-sonnet-20240620");
            assert_eq!(response.role, "assistant");
            assert_eq!(response.stop_reason, "end_turn");
            assert_eq!(response.stop_sequence, None);
            assert_eq!(response.response_type, "message");
            assert_eq!(response.usage.input_tokens, 2095);
            assert_eq!(response.usage.output_tokens, 503);
        }

        #[test]
        fn test_error_response_deserialization() {
            let json_str = r#"
            {
                "type": "error",
                    "error": {
                        "type": "invalid_request_error",
                        "message": "<string>"
                    }
            }
            "#;

            let response = serde_json::from_str::<ErrorResponse>(&json_str).unwrap();
            assert_eq!(response.error.error_type, "invalid_request_error");
            assert_eq!(response.error.message, "<string>");
        }
    }
}
