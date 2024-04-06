use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct OpenaiCompletionResponse {
    #[serde(rename = "created")]
    created: i64,

    #[serde(rename = "usage")]
    usage: Usage,

    #[serde(rename = "model")]
    model: String,

    #[serde(rename = "id")]
    id: String,

    #[serde(rename = "system_fingerprint")]
    system_fingerprint: String,

    #[serde(rename = "choices")]
    pub choices: Vec<Choice>,

    #[serde(rename = "object")]
    object: String,
}

#[derive(Serialize, Deserialize)]
pub struct Choice {
    #[serde(rename = "finish_reason")]
    pub finish_reason: String,

    #[serde(rename = "index")]
    index: i64,

    #[serde(rename = "message")]
    pub message: Message,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Message {
    #[serde(rename = "role")]
    pub role: String,

    #[serde(rename = "content")]
    pub content: String,
}

#[derive(Serialize, Deserialize)]
pub struct Usage {
    #[serde(rename = "completion_tokens")]
    completion_tokens: i64,

    #[serde(rename = "prompt_tokens")]
    prompt_tokens: i64,

    #[serde(rename = "total_tokens")]
    total_tokens: i64,
}
