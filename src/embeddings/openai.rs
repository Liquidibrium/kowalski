use crate::embeddings::embedding_model_service::EmbeddingModelApiAsync;
use crate::embeddings::openai_embedding_response::OpenaiEmbeddingResponse;
use anyhow::Context;
use serde_json::json;

use crate::llm::models::{EmbeddingModel, ModelProvider};

pub struct OpenaiEmbeddings {
    client: reqwest::Client,
    api_key: String,
    api_url: String,
    model: EmbeddingModel,
}

impl OpenaiEmbeddings {
    pub fn new(api_key: &str, model: &EmbeddingModel) -> Self {
        if api_key.is_empty() {
            panic!("Open AI API key is empty");
        }

        if model.get_model_provider() != ModelProvider::OpenAi {
            panic!("Model provider is not OpenAI");
        }
        OpenaiEmbeddings {
            client: reqwest::Client::new(),
            api_key: api_key.to_string(),
            api_url: "https://api.openai.com/v1/embeddings".to_string(),
            model: model.clone(),
        }
    }
}

impl EmbeddingModelApiAsync for OpenaiEmbeddings {
    fn get_embedding_input_token_size(&self) -> u64 {
        self.model.context_length() as u64
    }

    async fn get_embedding(&self, text: &str) -> anyhow::Result<Vec<f32>> {
        let response = self
            .client
            .post(self.api_url.clone())
            .header("Authorization", format!("Bearer {}", self.api_key))
            .json(&json!({
                "input": text,
                "model": self.model.get_model_name(),
                "encoding_format": "float"
            }))
            .send()
            .await?;
        let result = response.json::<OpenaiEmbeddingResponse>().await?;
        let embedding = result.data.first().context("No embedding found")?;
        Ok(embedding.embedding.to_vec())
    }

    async fn get_embedding_batch(&self, texts: &Vec<String>) -> anyhow::Result<Vec<Vec<f32>>> {
        let response = self
            .client
            .post(self.api_url.clone())
            .header("Authorization", format!("Bearer {}", self.api_key))
            .json(&json!({
                "input": texts,
                "model": self.model.get_model_name(),
                "encoding_format": "float",
            }))
            .send()
            .await?;
        let result = response.json::<OpenaiEmbeddingResponse>().await?;
        Ok(result
            .data
            .iter()
            .map(|embedding| embedding.embedding.to_vec())
            .collect())
    }
}
