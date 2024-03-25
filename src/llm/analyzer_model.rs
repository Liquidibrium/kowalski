use anyhow::Context;
use candle_transformers::models::quantized_mixformer::MixFormerSequentialForCausalLM;
use tokenizers::Tokenizer;
use crate::llm::inference::{answer_with_context, load_model};

pub struct CodeAnalyzerModel {
    llm_model: MixFormerSequentialForCausalLM,
    tokenizer: Tokenizer
}

impl CodeAnalyzerModel {
    pub fn new() -> Self {
        let loaded_model = load_model().expect("Unable to load model");
        CodeAnalyzerModel {
            llm_model: loaded_model.0,
            tokenizer: loaded_model.1
        }
    }
    
    
    pub async fn generate(&self, prompt: &str) -> anyhow::Result<String> {
        answer_with_context(prompt, &self.llm_model.clone(), &self.tokenizer.clone())
            .await.context("Failed to generate code")
    }
}