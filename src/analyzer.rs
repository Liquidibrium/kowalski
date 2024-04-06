use crate::code_processor::code::should_exclude;
use crate::embeddings::embedding_model_service::{EmbeddingModelService, EmbeddingService};
use crate::embeddings::openai::OpenaiEmbeddings;
use crate::git_provider::pr::PullRequestData;
use crate::llm::text_model::TextModelApiAsync;
use crate::memory::memory_db::EmbeddingMemoryQdrant;
use std::path::PathBuf;

pub struct CodeReviewFeedback {
    file_path: String,
    feedback: String,
}

pub async fn analyze_code_changes<TextModel: TextModelApiAsync>(
    pr_data: &PullRequestData,
    embedding_service: &EmbeddingModelService<OpenaiEmbeddings, EmbeddingMemoryQdrant>,
    text_model_api: &TextModel,
    collection: &str,
) -> anyhow::Result<Vec<CodeReviewFeedback>> {
    // todo calculate diff embedding find similar codes from memory and create a prompts for llm
    let mut feedback_comments = vec![];
    for file in &pr_data.changed_files {
        let target_file_path = file.clone().0.target_file;
        if should_exclude(&PathBuf::from(target_file_path.clone())) {
            continue;
        }
        let file_changes = file
            .0
            .hunks()
            .iter()
            .map(|hunk| hunk.to_string())
            .reduce(|a, b| a + "\n" + &b)
            .unwrap();

        let diff_embedding = embedding_service
            .calculate_text_embedding(file_changes.as_str())
            .await?;
        let top_similar_files = embedding_service
            .find_similar_chunks(&diff_embedding, collection, 3)
            .await?;

        println!(
            "Top similar files: {:?} -> {:?}",
            top_similar_files.len(),
            top_similar_files
                .iter()
                .map(|elem| { elem.similarity_score })
                .collect::<Vec<f32>>()
        );

        // TODO add context files
        let feedback = text_model_api
            .complete(&format!(
                "Analyze code changes for the diff {diff}",
                diff = file_changes
            ))
            .await?;
        feedback_comments.push(CodeReviewFeedback {
            file_path: target_file_path.clone(),
            feedback,
        })
    }
    Ok(feedback_comments)
}
