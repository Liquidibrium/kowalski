use crate::embeddings::embedding::{EmbeddingCalculator, EmbeddingModelLocal};
use crate::git_provider::client::clone_remote_repository;
use crate::git_provider::pr::{FetchPullRequest, PullRequestData};
use crate::memory::memory_db::{CodeEmbeddingData, EmbeddingMemory, EmbeddingMemoryQdrant};
use std::fs;
use std::path::{Path, PathBuf};

/// Process the source code of a pull request
/// There are steps:
/// 1. Clone the repository
/// 2. Read the files from the repository
/// 3. Process the files and code into a format that can be used by model
/// 4. Calculate code embeddings using model
/// 5. Upload the embeddings into the qdrant server
pub async fn process_source_code(
    local_repository_path: &str,
    pr_data: &PullRequestData,
    pr_info: FetchPullRequest,
    embedding_model: &mut EmbeddingModelLocal,
    memory: &EmbeddingMemoryQdrant,
) -> anyhow::Result<()> {
    // 1. clone repository
    let repo_path = clone_remote_repository(
        local_repository_path,
        &pr_data.owner,
        &pr_data.repo,
        &pr_data.head_ref,
        &pr_data.head_sha,
        &pr_info.github_token,
    )
    .await?;
    println!("Local path: {:?}", repo_path);

    // 2. Read the files from the repository
    let files = read_files(&repo_path, should_exclude)?;
    println!("Files: {:?}", files);

    process_code_files(&repo_path, &files, embedding_model, memory).await?;

    Ok(())
}

// 3. Process the files and code into a format that can be used by model
// 4. Calculate code embeddings using model
// 5. Upload the embeddings into the qdrant server
async fn process_code_files(
    repo_path: &str,
    code_file_paths: &Vec<PathBuf>,
    embedding_model: &mut EmbeddingModelLocal,
    memory: &EmbeddingMemoryQdrant,
) -> anyhow::Result<()> {
    let embeddings: Vec<CodeEmbeddingData> = code_file_paths
        .iter()
        .map(|code_file_path| {
            let code = fs::read_to_string(Path::join(Path::new(repo_path), code_file_path))
                .expect("Failed to read file");

            let embeddings = embedding_model.get_embedding(&code);
            println!("Embeddings: {:?}", embeddings);
            CodeEmbeddingData {
                filename: code_file_path.to_str().unwrap().to_string(),
                embedding: embeddings,
                code,
            }
        })
        .collect();

    memory.insert_batch(&embeddings).await
}

pub type ShouldExcludeFn = fn(&PathBuf) -> bool;

/// Read the files from the repository directory and return the list of file paths
fn read_files(
    local_repository_path: &String,
    should_exclude: ShouldExcludeFn,
) -> anyhow::Result<Vec<PathBuf>> {
    let mut files: Vec<PathBuf> = Vec::new();
    let mut dirs_stack: Vec<PathBuf> = Vec::new();
    dirs_stack.push(Path::new("").to_path_buf());

    loop {
        if dirs_stack.is_empty() {
            break;
        }
        let current_dir = dirs_stack.pop().unwrap();
        let paths = fs::read_dir(Path::join(Path::new(local_repository_path), &current_dir))?;
        for path in paths {
            let path = path?;
            // let path_str = path.path().to_str().unwrap().to_string();
            let path_str = path.file_name().to_str().unwrap().to_string();
            if should_exclude(&path.path().to_path_buf()) {
                continue;
            }
            if path.metadata()?.is_dir() {
                dirs_stack.push(Path::join(&current_dir, &path_str));
            } else {
                files.push(Path::join(current_dir.as_path(), &path_str));
            }
        }
    }

    Ok(files)
}

pub fn should_exclude(path: &PathBuf) -> bool {
    // should exclude if directory is .git
    let path_string = path.to_str();
    if path_string.is_none() {
        return true;
    }
    let path_string = path_string.unwrap();
    path.ends_with(".git") || path_string.contains(".circleci")
}
