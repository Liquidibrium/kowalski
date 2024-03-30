use anyhow::Context;
use qdrant_client::client::QdrantClient;
use qdrant_client::prelude::{PointStruct, SearchPoints};
use qdrant_client::qdrant::vectors::VectorsOptions;
use qdrant_client::qdrant::{
    vectors, with_payload_selector, with_vectors_selector, SearchParams, Value, Vector, Vectors,
    WithPayloadSelector, WithVectorsSelector,
};
use std::collections::HashMap;

pub fn create_vector_store_client(url: &str) -> anyhow::Result<QdrantClient> {
    QdrantClient::from_url(url).build()
}

pub struct CodeEmbeddingData {
    pub embedding: Vec<f32>,
    pub code: String,
    pub filename: String,
}
pub struct CodeEmbeddingResult {
    pub embedding: Vec<f32>,
    pub code: String,
    pub filename: String,
    pub similarity_score: f32,
}
pub trait EmbeddingMemory {
    fn new(db_url: &str, collection: &str) -> Self;

    async fn insert_batch(&self, embeddings: &Vec<CodeEmbeddingData>) -> anyhow::Result<()>;

    async fn search(
        &self,
        embedding: Vec<f32>,
        top: u64,
    ) -> anyhow::Result<Vec<CodeEmbeddingResult>>;
}

pub struct EmbeddingMemoryQdrant {
    collection: String,
    client: QdrantClient,
}

impl EmbeddingMemory for EmbeddingMemoryQdrant {
    fn new(db_url: &str, collection: &str) -> Self {
        let client = create_vector_store_client(db_url).unwrap();
        EmbeddingMemoryQdrant {
            collection: collection.to_string(),
            client,
        }
    }

    async fn insert_batch(&self, embeddings: &Vec<CodeEmbeddingData>) -> anyhow::Result<()> {
        let embeddings: Vec<PointStruct> = embeddings
            .iter()
            .map(|data| {
                let mut payload: HashMap<String, Value> = HashMap::new();
                payload.insert("code".to_string(), Value::from(data.code.clone()));
                payload.insert("filename".to_string(), Value::from(data.filename.clone()));
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
        let response = self
            .client
            .upsert_points_batch(self.collection.clone(), None, embeddings, None, length)
            .await?;

        println!("Response: {:?}", response);
        Ok(())
    }

    async fn search(
        &self,
        embedding: Vec<f32>,
        top_k: u64,
    ) -> anyhow::Result<Vec<CodeEmbeddingResult>> {
        let search_options = SearchPoints {
            collection_name: self.collection.clone(),
            vector: embedding,
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
            let code = point.payload.get("code").unwrap().to_string();
            let filename = point.payload.get("filename").unwrap().to_string();
            results.push(CodeEmbeddingResult {
                embedding: match point.vectors.unwrap().vectors_options.unwrap() {
                    VectorsOptions::Vector(vector) => vector.data.clone(),
                    VectorsOptions::Vectors(_) => {
                        panic!("Vectors not supported")
                    }
                },
                code,
                filename,
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
