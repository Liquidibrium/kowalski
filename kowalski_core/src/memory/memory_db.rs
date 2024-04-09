use anyhow::Context;
use log::info;
use qdrant_client::client::QdrantClient;
use qdrant_client::prelude::{PointStruct, SearchPoints};
use qdrant_client::qdrant::vectors::VectorsOptions;
use qdrant_client::qdrant::{
    vectors, with_payload_selector, with_vectors_selector, SearchParams, Value, Vector, Vectors,
    WithPayloadSelector, WithVectorsSelector,
};
use std::collections::HashMap;
use std::sync::Arc;

pub fn create_vector_store_client(url: &str) -> anyhow::Result<QdrantClient> {
    QdrantClient::from_url(url).build()
}

pub struct Chunk {
    pub content: String,
    pub file_path: String,
    pub start_line: u64,
    pub end_line: u64,
    pub token_size: u64,
}

pub struct FileAndContent {
    pub content: String,
    pub file_path: String,
}

pub struct CodeEmbeddingData {
    pub embedding: Vec<f32>,
    pub chunk: Chunk,
}

pub struct CodeEmbeddingResult {
    pub embedding: Vec<f32>,
    pub chunk: Chunk,
    pub similarity_score: f32,
}

pub trait EmbeddingMemory {
    fn new(db_url: &str) -> Self;

    async fn insert_batch(
        &self,
        embeddings: &Vec<CodeEmbeddingData>,
        collection: &str,
    ) -> anyhow::Result<()>;

    async fn search(
        &self,
        embedding: &Vec<f32>,
        collection: &str,
        top: u64,
    ) -> anyhow::Result<Vec<CodeEmbeddingResult>>;
}

#[derive(Clone)]
pub struct EmbeddingMemoryQdrant {
    client: Arc<QdrantClient>,
}

impl EmbeddingMemory for EmbeddingMemoryQdrant {
    fn new(db_url: &str) -> Self {
        let client = create_vector_store_client(db_url).unwrap();
        EmbeddingMemoryQdrant {
            client: Arc::new(client),
        }
    }

    async fn insert_batch(
        &self,
        embeddings: &Vec<CodeEmbeddingData>,
        collection: &str,
    ) -> anyhow::Result<()> {
        let embeddings: Vec<PointStruct> = embeddings
            .iter()
            .map(|data| {
                let mut payload: HashMap<String, Value> = HashMap::new();
                payload.insert(
                    "content".to_string(),
                    Value::from(data.chunk.content.clone()),
                );
                payload.insert(
                    "file_path".to_string(),
                    Value::from(data.chunk.file_path.clone()),
                );
                payload.insert(
                    "start_line".to_string(),
                    Value::from(data.chunk.start_line as i64),
                );
                payload.insert(
                    "end_line".to_string(),
                    Value::from(data.chunk.end_line as i64),
                );
                payload.insert(
                    "tokens_size".to_string(),
                    Value::from(data.chunk.token_size as i64),
                );
                PointStruct {
                    id: None,
                    vectors: Some(Vectors {
                        vectors_options: Some(vectors::VectorsOptions::Vector(Vector {
                            data: data.embedding.clone(),
                            indices: None,
                        })),
                    }),
                    payload,
                }
            })
            .collect();

        let length = embeddings.len();
        info!(
            "Inserting embeddings into collection: {} len: {}",
            collection, length
        );

        let response = self
            .client
            .upsert_points_batch(collection, None, embeddings, None, length)
            .await?;

        println!("Qdrant upsert points batch Response: {:?}", response);
        Ok(())
    }

    async fn search(
        &self,
        embedding: &Vec<f32>,
        collection: &str,
        top_k: u64,
    ) -> anyhow::Result<Vec<CodeEmbeddingResult>> {
        let search_options = SearchPoints {
            collection_name: collection.to_string(),
            vector: embedding.clone(),
            limit: top_k,
            score_threshold: Some(0.3),
            params: Some(SearchParams {
                exact: Some(true),
                hnsw_ef: Some(300),
                indexed_only: None,
                quantization: None,
            }),
            filter: None,
            offset: None,
            with_payload: Some(WithPayloadSelector {
                selector_options: Some(with_payload_selector::SelectorOptions::Enable(true)),
            }),
            with_vectors: Some(WithVectorsSelector {
                selector_options: Some(with_vectors_selector::SelectorOptions::Enable(true)),
            }),
            vector_name: None,
            read_consistency: None,
            timeout: None,
            shard_key_selector: None,
            sparse_indices: None,
        };
        let response = self.client.search_points(&search_options).await?;

        let mut results: Vec<CodeEmbeddingResult> = Vec::new();
        for point in response.result {
            let chunk = Chunk {
                content: point.payload.get("content").unwrap().to_string(),
                file_path: point.payload.get("file_path").unwrap().to_string(),
                start_line: point
                    .payload
                    .get("start_line")
                    .unwrap()
                    .to_string()
                    .parse()
                    .unwrap(),
                end_line: point
                    .payload
                    .get("end_line")
                    .unwrap()
                    .to_string()
                    .parse()
                    .unwrap(),
                token_size: point
                    .payload
                    .get("tokens_size")
                    .unwrap()
                    .to_string()
                    .parse()
                    .unwrap(),
            };
            results.push(CodeEmbeddingResult {
                embedding: match point.vectors.unwrap().vectors_options.unwrap() {
                    VectorsOptions::Vector(vector) => vector.data.clone(),
                    VectorsOptions::Vectors(_) => {
                        panic!("Vectors not supported")
                    }
                },
                chunk,
                similarity_score: point.score,
            });
        }

        Ok(results)
    }
}

pub fn init_memory() -> anyhow::Result<()> {
    // execute docker compose up
    std::process::Command::new("docker-compose")
        .arg("up")
        .arg("-d")
        .output()
        .context("failed to execute process")?;
    Ok(())
}
