use crate::diff::SerializablePatchedFile;
use crate::git_provider::git::GitProvider;
use serde::Serialize;

pub struct FetchPullRequest {
    pub owner: String,
    pub repo: String,
    pub pull_request: u64,

    pub git_provider: GitProvider,
    pub github_token: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct PullRequestData {
    pub owner: String,
    pub repo: String,
    pub pull_request: u64,
    pub git_provider: GitProvider,

    pub title: Option<String>,
    pub description: Option<String>,

    pub created_at: Option<chrono::DateTime<chrono::Utc>>,
    pub updated_at: Option<chrono::DateTime<chrono::Utc>>,

    pub head_sha: String,
    pub head_ref: String,
    pub base_sha: String,
    pub base_ref: String,

    pub changed_files: Vec<SerializablePatchedFile>,
}
