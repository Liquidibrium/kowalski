use std::env;
use crate::embeddings::embedding_model_service::{EmbeddingModelApiAsync, EmbeddingModelService, EmbeddingService};
use crate::embeddings::openai::OpenaiEmbeddings;
use crate::llm::anthropic::CloudeTextModel;
use crate::llm::models::{EmbeddingModel, LlmModel, ModelProvider};
use crate::llm::openai::OpenaiTextModel;
use crate::llm::text_model::TextModelApiAsync;
use crate::memory::memory_db::{EmbeddingMemory, EmbeddingMemoryQdrant};

pub enum Stage {
    Production,
    Development,
    Test,
}

pub struct CreatorFactory {
    embedding_model: EmbeddingModel,
    model_provider: ModelProvider,
    openai_api_key: Option<String>,
    anthropic_api_key: Option<String>,
    stage: Stage,
}


impl CreatorFactory {
    pub fn new(embedding_model: EmbeddingModel, openai_api_key: Option<String>, anthropic_api_key: Option<String>) -> Self {
        let stage = match env::var("STAGE").unwrap_or("production".to_string()).to_lowercase().as_str() {
            "test" => Stage::Test,
            _ => Stage::Production,
        };

        CreatorFactory {
            embedding_model,
            openai_api_key,
            anthropic_api_key,
            stage,
            model_provider: ModelProvider::Anthropic
        }
    }

    pub fn create_embedding_memory(&self) -> impl EmbeddingMemory {
        EmbeddingMemoryQdrant::new("http://localhost:6333")
    }

    pub fn create_embedding_model_api(&self) -> impl EmbeddingModelApiAsync {
        match self.embedding_model.get_model_provider() {
            ModelProvider::OpenAi => {
                if self.openai_api_key.is_none() {
                    panic!("Open AI API key is empty");
                }

                return OpenaiEmbeddings::new(
                    self.openai_api_key.as_ref()
                        .expect("empty openai key"),
                    &self.embedding_model,
                );
            }
            ModelProvider::Anthropic => {
                panic!("Anthropic embedding models is not supported yet")
            }

            ModelProvider::Local => {
                panic!("Anthropic local embedding model is not supported yet")
            }
        }
    }

    pub fn create_embedding_service(&self) -> EmbeddingModelService<impl EmbeddingModelApiAsync, impl EmbeddingMemory> {
        let memory = self.create_embedding_memory();
        let embedding_api= self.create_embedding_model_api();
        EmbeddingModelService::new(embedding_api, memory)
    }
    
    pub fn create_llm_model_api_client(&self) -> Box<dyn TextModelApiAsync> {
        match self.model_provider {
            ModelProvider::OpenAi => {
                let openai_api_key = self.openai_api_key.clone().expect("OPENAI API Key not found");
                return Box::new(OpenaiTextModel::new(openai_api_key.as_str(), vec![LlmModel::Gpt3_5Turbo], None)); 
            }
            ModelProvider::Anthropic => {
                let anthropic_api_key = self.anthropic_api_key.clone().expect("Anthropic api key");
                return Box::new(CloudeTextModel::new(anthropic_api_key.as_str(), vec![LlmModel::Claude3Haiku], None))
            }
            ModelProvider::Local => {
                panic!("local llm model is not supported")
            }
        }
        
    }
}