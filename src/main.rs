mod analyzer;
mod cli;
mod code_processor;
mod device;
mod diff;
mod embeddings;
mod git_provider;
mod init;
mod llm;
mod memory;
mod utils;

use crate::analyzer::analyze_code_changes;
use crate::code_processor::code::process_source_code;
use crate::embeddings::embedding::{EmbeddingCalculator, EmbeddingModelLocal};
use crate::git_provider::client::fetch_pull_request;
use crate::git_provider::git::GitProvider;
use crate::git_provider::pr::FetchPullRequest;
use crate::memory::memory_db::{init_memory, EmbeddingMemory, EmbeddingMemoryQdrant};
use clap::Parser;
use cli::Cli;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let kowalski_cli = Cli::parse();
    match &kowalski_cli.command {
        Some(command) => match command {
            cli::Commands::Analysis {
                owner,
                repository,
                pull_request,
                pr_link,
                git_token,
                git_provider,
                ..
            } => {
                let mut embedding = EmbeddingModelLocal::new("tmp");

                let pr_info = get_pr_repository_info(
                    owner,
                    repository,
                    pull_request,
                    pr_link,
                    git_token,
                    git_provider,
                );
                let pr_data = fetch_pull_request(&pr_info).await?;
                let now = chrono::offset::Local::now().timestamp();
                let local_path =
                    format!("/tmp/kowalski/{}/{}/{}", pr_info.owner, pr_info.repo, now);

                let collection = format!(
                    "kowalski-{}-{}-{}-{}",
                    pr_info.owner, pr_info.repo, pr_info.pull_request, now
                );
                init_memory()?;
                let memory =
                    EmbeddingMemoryQdrant::new("http://localhost:6333", collection.as_str());
                process_source_code(
                    local_path.as_str(),
                    &pr_data,
                    pr_info,
                    &mut embedding,
                    &memory,
                )
                .await?;

                analyze_code_changes(&pr_data, &mut embedding, &memory).await?;
            }
        },
        None => {
            println!("use: kowalski analysis --help for more details");
        }
    }

    return Ok(());
}

fn get_pr_repository_info(
    owner: &Option<String>,
    repository: &Option<String>,
    pull_request: &Option<u64>,
    pr_link: &Option<String>,
    token: &Option<String>,
    git_provider: &GitProvider,
) -> FetchPullRequest {
    let pr_info: FetchPullRequest = if let Some(pr_link) = pr_link {
        println!("Analyzing pull request from link: {}", pr_link);
        // https://github.com/<owner>/<repository>/pull/<pull_request>
        let parts: Vec<&str> = pr_link.split('/').collect();
        if parts.len() < 7 {
            panic!("Invalid pull request link provided");
        }
        FetchPullRequest {
            owner: parts[3].to_string(),
            repo: parts[4].to_string(),
            pull_request: parts[6].parse().expect("Invalid pull request number"),
            git_provider: git_provider.clone(),
            github_token: token.clone(),
        }
    } else {
        println!(
            "Analyzing pull request from owner: {}, repository: {}, pull_request: {}",
            owner.as_ref().unwrap(),
            repository.as_ref().unwrap(),
            pull_request.as_ref().unwrap()
        );
        FetchPullRequest {
            owner: owner
                .as_ref()
                .expect("repository owner was not provided")
                .to_string(),
            repo: repository
                .as_ref()
                .expect("repository was not provided")
                .to_string(),
            pull_request: pull_request
                .expect("pull request number was not provided")
                .to_owned(),
            git_provider: git_provider.clone(),
            github_token: token.clone(),
        }
    };
    pr_info
}
