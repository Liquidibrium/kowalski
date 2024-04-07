use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct OpenaiEmbeddingResponse {
    #[serde(rename = "data")]
    pub data: Vec<EmbeddingData>,

    #[serde(rename = "usage")]
    usage: Usage,

    #[serde(rename = "model")]
    model: String,

    #[serde(rename = "object")]
    object: String,
}

#[derive(Serialize, Deserialize)]
pub struct EmbeddingData {
    #[serde(rename = "index")]
    index: u32,

    #[serde(rename = "embedding")]
    pub embedding: Vec<f32>,

    #[serde(rename = "object")]
    pub object: String,
}

#[derive(Serialize, Deserialize)]
pub struct Usage {
    #[serde(rename = "prompt_tokens")]
    prompt_tokens: u64,

    #[serde(rename = "total_tokens")]
    total_tokens: u64,
}
