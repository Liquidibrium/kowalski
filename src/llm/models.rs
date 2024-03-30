use clap::ValueEnum;
use serde::Serialize;

#[derive(Debug, ValueEnum, Clone, Eq, PartialEq, Serialize)]
pub enum LlmModel {
    Gpt3_5Turbo,
    Gpt4_32k,
    Gpt4TurboPreview,

    Claude3Haiku,
    Claude3Sonnet,
    Claude3Opus,
}

impl LlmModel {
    pub fn get_model_name(&self) -> String {
        match self {
            LlmModel::Gpt3_5Turbo => "gpt-3.5-turbo-0125".to_string(),
            LlmModel::Gpt4_32k => "gpt-4-32k".to_string(),
            LlmModel::Gpt4TurboPreview => "gpt-4-turbo-preview".to_string(),
            LlmModel::Claude3Haiku => "claude-3-haiku-20240307".to_string(),
            LlmModel::Claude3Sonnet => "claude-3-sonnet-20240229".to_string(),
            LlmModel::Claude3Opus => "claude-3-opus-20240229".to_string(),
        }
    }
    pub fn context_window(&self) -> usize {
        match self {
            LlmModel::Gpt3_5Turbo => 16_385,
            LlmModel::Gpt4_32k => 32_768,
            LlmModel::Gpt4TurboPreview => 128_000,
            LlmModel::Claude3Haiku => 200_000,
            LlmModel::Claude3Sonnet => 200_000,
            LlmModel::Claude3Opus => 200_000,
        }
    }

    pub fn get_model_provider(&self) -> LlmProvider {
        match self {
            LlmModel::Gpt3_5Turbo => LlmProvider::OpenAi,
            LlmModel::Gpt4_32k => LlmProvider::OpenAi,
            LlmModel::Gpt4TurboPreview => LlmProvider::OpenAi,

            LlmModel::Claude3Haiku => LlmProvider::Anthropic,
            LlmModel::Claude3Sonnet => LlmProvider::Anthropic,
            LlmModel::Claude3Opus => LlmProvider::Anthropic,
        }
    }
}

#[derive(Debug, ValueEnum, Clone, Eq, PartialEq, Serialize)]
pub enum EmbeddingModel {
    OpenAiTextEmbedding3Large,
    OpenAiTextEmbedding3Small,
    AnthropicVoyageLarge2,
    AnthropicVoyageCode2,
}

impl EmbeddingModel {
    pub fn get_model_name(&self) -> String {
        match self {
            EmbeddingModel::OpenAiTextEmbedding3Large => "text-embedding-3-large".to_string(),
            EmbeddingModel::OpenAiTextEmbedding3Small => "text-embedding-3-small".to_string(),
            EmbeddingModel::AnthropicVoyageLarge2 => "voyage-large-2".to_string(),
            EmbeddingModel::AnthropicVoyageCode2 => "voyage-code-2".to_string(),
        }
    }

    pub fn context_length(&self) -> usize {
        match self {
            EmbeddingModel::OpenAiTextEmbedding3Large => 16_385,
            EmbeddingModel::OpenAiTextEmbedding3Small => 16_385,
            EmbeddingModel::AnthropicVoyageLarge2 => 16_000,
            EmbeddingModel::AnthropicVoyageCode2 => 16_000,
        }
    }

    pub fn output_dimension(&self) -> usize {
        match self {
            EmbeddingModel::OpenAiTextEmbedding3Large => 3_072,
            EmbeddingModel::OpenAiTextEmbedding3Small => 1_536,
            EmbeddingModel::AnthropicVoyageLarge2 => 1_536,
            EmbeddingModel::AnthropicVoyageCode2 => 1_536,
        }
    }

    pub fn get_model_provider(&self) -> LlmProvider {
        match self {
            EmbeddingModel::OpenAiTextEmbedding3Large => LlmProvider::OpenAi,
            EmbeddingModel::OpenAiTextEmbedding3Small => LlmProvider::OpenAi,
            EmbeddingModel::AnthropicVoyageLarge2 => LlmProvider::Anthropic,
            EmbeddingModel::AnthropicVoyageCode2 => LlmProvider::Anthropic,
        }
    }
}

pub enum LlmProvider {
    OpenAi,
    Anthropic,
    Local,
}
