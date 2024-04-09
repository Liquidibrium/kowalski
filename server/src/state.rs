use kowalski_core::memory::memory_db::EmbeddingMemoryQdrant;

pub struct AppState {
    pub db: sqlx::PgPool,
    pub embedding_memory: EmbeddingMemoryQdrant,
}
