use crate::embeddings::embedding::EmbeddingModelLocal;
use crate::git_provider::pr::PullRequestData;
use crate::memory::memory_db::EmbeddingMemoryQdrant;

pub async fn analyze_code_changes(
    _pr_data: &PullRequestData,
    _embedding_model: &EmbeddingModelLocal,
    _memory: &EmbeddingMemoryQdrant,
) -> anyhow::Result<()> {
    Ok(())
}
