use crate::memory::memory_db::{
    Chunk, CodeEmbeddingData, CodeEmbeddingResult, EmbeddingMemory, FileAndContent,
};
use lazy_static::lazy_static;
use log::info;
use std::iter::zip;
use tiktoken_rs::{cl100k_base, CoreBPE};

lazy_static! {
    static ref TIKTOKEN_TOKENIZER: CoreBPE = cl100k_base().unwrap();
}

pub fn calculate_token_size(content: &str) -> u64 {
    TIKTOKEN_TOKENIZER.encode_with_special_tokens(content).len() as u64
}

pub trait EmbeddingModelApiAsync {
    fn get_embedding_input_token_size(&self) -> u64;
    async fn get_embedding(&self, text: &str) -> anyhow::Result<Vec<f32>>;
    async fn get_embedding_batch(&self, texts: &Vec<String>) -> anyhow::Result<Vec<Vec<f32>>>;
}

pub trait EmbeddingService<ModelApi: EmbeddingModelApiAsync, Memory: EmbeddingMemory> {
    fn new(embedding_model_api: ModelApi, embedding_memory: Memory) -> Self;

    async fn calculate_and_save_embeddings(
        &self,
        data: &Vec<FileAndContent>,
        collection_name: &str,
    ) -> anyhow::Result<()>;
    async fn find_similar_chunks(
        &self,
        embedding: &Vec<f32>,
        collection_name: &str,
        top: u64,
    ) -> anyhow::Result<Vec<CodeEmbeddingResult>>;

    async fn calculate_text_embedding(&self, data: &str) -> anyhow::Result<Vec<f32>>;
}

pub struct EmbeddingModelService<ModelApi: EmbeddingModelApiAsync, Memory: EmbeddingMemory> {
    embedding_model_api: ModelApi,
    embedding_memory: Memory,
}

impl<ModelApi: EmbeddingModelApiAsync, Memory: EmbeddingMemory> EmbeddingService<ModelApi, Memory>
    for EmbeddingModelService<ModelApi, Memory>
{
    fn new(embedding_model_api: ModelApi, embedding_memory: Memory) -> Self {
        EmbeddingModelService {
            embedding_model_api,
            embedding_memory,
        }
    }

    async fn calculate_and_save_embeddings(
        &self,
        data: &Vec<FileAndContent>,
        collection_name: &str,
    ) -> anyhow::Result<()> {
        let max_token_size = self.embedding_model_api.get_embedding_input_token_size();
        info!(
            "Calculating embeddings for {} files - max token size {}",
            data.len(),
            max_token_size
        );
        let chunks: Vec<Chunk> = data
            .iter()
            .flat_map(|file| split_into_chunks(file, max_token_size))
            .collect();
        info!("Calculated {} chunks", chunks.len());
        let embedding_texts: Vec<String> = chunks
            .iter()
            .map(|chunk| format!("{}\n{}", chunk.file_path, chunk.content))
            .collect();
        let embeddings = self
            .embedding_model_api
            .get_embedding_batch(&embedding_texts)
            .await?;

        let embeddings_and_chunks: Vec<CodeEmbeddingData> = zip(chunks, embeddings)
            .map(|(chunk, embedding)| CodeEmbeddingData { embedding, chunk })
            .collect();

        self.embedding_memory
            .insert_batch(&embeddings_and_chunks, collection_name)
            .await?;

        Ok(())
    }

    async fn find_similar_chunks(
        &self,
        embedding: &Vec<f32>,
        collection_name: &str,
        top: u64,
    ) -> anyhow::Result<Vec<CodeEmbeddingResult>> {
        let similar_chunks = self
            .embedding_memory
            .search(embedding, collection_name, top)
            .await?;

        Ok(similar_chunks)
    }

    async fn calculate_text_embedding(&self, data: &str) -> anyhow::Result<Vec<f32>> {
        self.embedding_model_api.get_embedding(data).await
    }
}

fn split_into_chunks(file_contents: &FileAndContent, max_token_size: u64) -> Vec<Chunk> {
    let mut chunks = Vec::new();
    let mut start_line = 0;
    let mut end_line = 0;
    let mut token_size = 0;
    let mut content = String::new();
    for (i, line) in file_contents.content.lines().enumerate() {
        let line_token_size = calculate_token_size(line);
        if token_size + line_token_size > max_token_size {
            chunks.push(Chunk {
                content: content.clone(),
                file_path: file_contents.file_path.clone(),
                start_line,
                end_line,
                token_size,
            });
            content = String::new();
            start_line = i as u64;
            token_size = 0;
        }
        content.push_str(line);
        content.push('\n');
        end_line = i as u64;
        token_size += line_token_size;
    }
    chunks.push(Chunk {
        content,
        file_path: file_contents.file_path.clone(),
        start_line,
        end_line,
        token_size,
    });
    chunks
}
