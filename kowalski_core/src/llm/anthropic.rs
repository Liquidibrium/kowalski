use crate::llm::cloude_completion_response::CloudeCompletionResponse;
use crate::llm::models::LlmModel;
use crate::llm::text_model::TextModelApiAsync;
use serde_json::json;

pub struct CloudeTextModel {
    api_url: String,
    api_key: String,
    models: Vec<LlmModel>,
    system_prompt: Option<String>,
    client: reqwest::Client,
    max_tokens: u32,
}

impl CloudeTextModel {
    pub fn new(api_key: &str, models: Vec<LlmModel>, system_prompt: Option<String>) -> Self {
        if api_key.is_empty() {
            panic!("Open AI API key is empty");
        }
        CloudeTextModel {
            api_url: "https://api.anthropic.com/v1/complete".to_string(),
            api_key: api_key.to_string(),
            models,
            system_prompt,
            client: reqwest::Client::new(),
            max_tokens: 1000,
        }
    }
}

async fn generate_text(
    client: reqwest::Client,
    prompt: &str,
    api_url: &str,
    api_key: &str,
    model: &str,
    max_tokens: &u32,
) -> anyhow::Result<String> {
    let response = client
        .post(api_url)
        .header("x-api-key", api_key)
        .header("anthropic-version", "2023-06-01")
        .json(&json!({
            "max_tokens_to_sample": max_tokens,
            "model": model,
            "prompt": prompt,
        }))
        .send()
        .await?;
    let result = response.json::<CloudeCompletionResponse>().await?;
    Ok(result.completion)
}

impl TextModelApiAsync for CloudeTextModel {
    async fn complete(&self, prompt: &str) -> anyhow::Result<String> {
        let prompt = get_prompt(prompt, &self.system_prompt);
        generate_text(
            self.client.clone(),
            &prompt,
            &self.api_url,
            &self.api_key,
            &self.models[0].get_model_name(),
            &self.max_tokens,
        )
        .await
    }

    async fn complete_batch(&self, prompts: &Vec<&str>) -> anyhow::Result<Vec<String>> {
        let mut results = vec![];
        for prompt in prompts {
            let prompt = get_prompt(prompt, &self.system_prompt);
            let result = generate_text(
                self.client.clone(),
                &prompt,
                &self.api_url,
                &self.api_key,
                &self.models[0].get_model_name(),
                &self.max_tokens,
            )
            .await?;
            results.push(result);
        }
        Ok(results)
    }

    async fn complete_streaming(&self, prompt: &str) -> anyhow::Result<String> {
        let prompt = get_prompt(prompt, &self.system_prompt);
        generate_text(
            self.client.clone(),
            &prompt,
            &self.api_url,
            &self.api_key,
            &self.models[0].get_model_name(),
            &self.max_tokens,
        )
        .await
    }
}

fn get_prompt(user_prompt: &str, system_prompt: &Option<String>) -> String {
    // "\n\nHuman: {userQuestion}\n\nAssistant:"
    let mut prompt = String::new();
    if let Some(system_prompt) = system_prompt {
        prompt.push_str(system_prompt);
    }

    prompt.push_str(user_prompt);

    format!("\n\nHuman: {}\n\nAssistant:", prompt)
}
