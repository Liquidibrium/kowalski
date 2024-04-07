use anyhow::{Context, Error};

use crate::llm::models::LlmModel;
use crate::llm::openai_completion_response::{Message, OpenaiCompletionResponse};
use crate::llm::text_model::TextModelApiAsync;
use serde_json::json;

pub struct OpenaiTextModel {
    api_url: String,
    api_key: String,
    models: Vec<LlmModel>,
    system_prompt: Option<String>,
    client: reqwest::Client,
    max_tokens: u32,
}

impl OpenaiTextModel {
    pub fn new(api_key: &str, models: Vec<LlmModel>, system_prompt: Option<String>) -> Self {
        if api_key.is_empty() {
            panic!("Open AI API key is empty");
        }
        OpenaiTextModel {
            api_url: "https://api.openai.com/v1/chat/completions".to_string(),
            api_key: api_key.to_string(),
            models,
            system_prompt,
            client: reqwest::Client::new(),
            max_tokens: 1000,
        }
    }
}

pub async fn generate_text(
    client: reqwest::Client,
    messages: &Vec<Message>,
    api_url: &str,
    api_key: &str,
    model: &str,
    max_tokens: &u32,
) -> Result<Result<String, Error>, Error> {
    let response = client
        .post(api_url)
        .header("Authorization", format!("Bearer {}", api_key))
        .json(&json!({
            "max_tokens": max_tokens,
            "model": model,
            "messages": messages,
            "temperature": 0.5,
            "top_p": 1.0,
            "frequency_penalty": 0.0,
            "presence_penalty": 0.0,
            "stream": false,
        }))
        .send()
        .await?;
    let result = response.json::<OpenaiCompletionResponse>().await?;
    Ok(Ok(result
        .choices
        .first()
        .context("no choices found")?
        .message
        .content
        .clone()))
}

fn get_messages(prompt: &str, system_prompt: &Option<String>) -> Vec<Message> {
    let mut chat_messages = vec![];
    if let Some(prompt) = system_prompt {
        chat_messages.push(Message {
            role: "system".to_string(),
            content: prompt.clone(),
        });
    }

    chat_messages.push(Message {
        role: "user".to_string(),
        content: prompt.to_string(),
    });

    chat_messages
}

impl TextModelApiAsync for OpenaiTextModel {
    async fn complete(&self, prompt: &str) -> anyhow::Result<String> {
        let messages = get_messages(prompt, &self.system_prompt);
        generate_text(
            self.client.clone(),
            &messages,
            &self.api_url,
            &self.api_key,
            &self.models[0].get_model_name(),
            &self.max_tokens,
        )
        .await?
    }

    async fn complete_batch(&self, prompts: &Vec<&str>) -> anyhow::Result<Vec<String>> {
        let mut results = vec![];
        for prompt in prompts {
            let messages = get_messages(prompt, &self.system_prompt);
            let result = generate_text(
                self.client.clone(),
                &messages,
                &self.api_url,
                &self.api_key,
                &self.models[0].get_model_name(),
                &self.max_tokens,
            )
            .await?;
            results.push(result?);
        }
        Ok(results)
    }

    async fn complete_streaming(&self, prompt: &str) -> anyhow::Result<String> {
        let messages = get_messages(prompt, &self.system_prompt);
        generate_text(
            self.client.clone(),
            &messages,
            &self.api_url,
            &self.api_key,
            &self.models[0].get_model_name(),
            &self.max_tokens,
        )
        .await?
    }
}
