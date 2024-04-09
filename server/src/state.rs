use kowalski_core::memory::memory_db::EmbeddingMemoryQdrant;

#[derive(Clone)]
pub struct AppState {
    pub db: sqlx::PgPool,
    pub embedding_memory: EmbeddingMemoryQdrant,
}
