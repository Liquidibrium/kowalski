use crate::embeddings::embedding::EmbeddingModelLocal;
use crate::git_provider::pr::PullRequestData;
use crate::llm::analyzer_model::CodeAnalyzerModel;
use crate::memory::memory_db::EmbeddingMemoryQdrant;

pub async fn analyze_code_changes(
    pr_data: &PullRequestData,
    embedding_model: &EmbeddingModelLocal,
    memory: &EmbeddingMemoryQdrant,
) -> anyhow::Result<()> {
    let code_analyzer_llm = CodeAnalyzerModel::new();
    // todo calculate diff embedding find similar codes from memory and create a prompts for llm

    for file in &pr_data.changed_files {
        let file_changes = file.0.hunks().iter()
            .map(|hunk| hunk.to_string())
            .reduce(|a, b| a + "\n" + &b).unwrap();
        code_analyzer_llm.generate(&format!("Analyze code changes for the diff {diff}", diff = file_changes)).await?;
    }
    Ok(())
}
