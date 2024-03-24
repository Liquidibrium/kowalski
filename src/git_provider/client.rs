use crate::cli::GitProvider;
use crate::diff::SerializablePatchedFile;
use crate::git_provider::pr::{FetchPullRequest, PullRequestData};
use std::path::Path;
use unidiff::PatchSet;
use url;
use url::Url;

pub async fn fetch_pull_request(pr_data: &FetchPullRequest) -> anyhow::Result<PullRequestData> {
    if pr_data.git_provider != GitProvider::Github {
        return Err(anyhow::anyhow!("Only GitHub is supported for now"));
    }
    println!("Fetching pull request from GitHub");

    let github = octocrab::instance();

    // if let Some(token) = &pr_data.github_token {
    //     github.authenticate(octocrab::auth::Auth::new(token.to_string()));
    // }
    let prs_client = github.pulls(pr_data.owner.clone(), pr_data.repo.clone());
    let pr = prs_client
        .get(pr_data.pull_request)
        .await
        .map_err(|e| anyhow::anyhow!("Failed to fetch pull request {:?}", e))?;

    let pr_diff = prs_client
        .get_diff(pr_data.pull_request)
        .await
        .map_err(|e| anyhow::anyhow!("Failed to fetch diff {:?}", e))?;

    let mut patch = PatchSet::new();
    patch.parse(pr_diff)?;
    let patched_files = patch
        .files()
        .iter()
        .map(|patched_file| {
            // println!("{} -> {}\n", patched_file.target_file, patched_file.source_file);
            // patched_file.hunks().iter().for_each(|hunk| {
            //     println!("hunk {}\n", hunk);
            // });
            SerializablePatchedFile(patched_file.clone())
        })
        .collect();

    // pretty_print(&pr);
    let pr = PullRequestData {
        owner: pr_data.owner.clone(),
        repo: pr_data.repo.clone(),
        pull_request: pr_data.pull_request,
        git_provider: pr_data.git_provider.clone(),

        title: pr.title,
        description: pr.body,

        changed_files: patched_files,

        head_sha: pr.head.sha,
        head_ref: pr.head.ref_field,
        base_sha: pr.base.sha,
        base_ref: pr.base.ref_field,

        created_at: pr.created_at,
        updated_at: pr.updated_at,
    };

    Ok(pr)
}

pub async fn clone_remote_repository(
    local_repository_path: &str,
    owner: &String,
    repository: &String,
    head_ref: &str,
    head_sha: &String,
    _token: &Option<String>,
) -> anyhow::Result<String> {
    println!("Cloning repository from GitHub");
    let github = octocrab::instance();
    // if let Some(token) = &token {
    //     github.authenticate(octocrab::auth::Auth::new(token.to_string()));
    // }
    let repo = github.repos(owner, repository);
    let repo = repo.get().await?;
    let clone_url = repo.clone_url.unwrap_or_else(|| {
        Url::parse(&format!("git@github.com:{}/{}.git", owner, repository))
            .expect("Invalid repository URL")
    });

    let local_path = format!("{}/{}", local_repository_path, head_sha);
    println!("Cloning repository to {}", local_path); 
    git2::build::RepoBuilder::new()
        .branch(head_ref)
        .clone(clone_url.as_str(), Path::new(&local_path))?;

    // println!("Cloned repository to {}", local_path);
    Ok(local_path)
}
