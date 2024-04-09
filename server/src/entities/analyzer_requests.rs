use sqlx::FromRow;
use strum_macros::EnumString;

#[derive(Debug, PartialEq, Clone, EnumString)]
pub enum Status {
    Scheduled,
    Running,
    Completed,
    Failed,
}

#[derive(Debug, PartialEq, Clone, FromRow)]
pub struct AnalyzerRequests {
    pub id: sqlx::types::Uuid,
    pub team_id: sqlx::types::Uuid,
    pub user_id: Option<sqlx::types::Uuid>,

    pub analyzer_result: Option<sqlx::types::JsonValue>,

    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime,
    pub status: Status,

    pub url_link: String,
    pub repository_id: sqlx::types::Uuid,

    pub branch_from: String,
    pub head_sha: String,
    pub branch_to: String,
    pub base_sha: String,

    pub pull_request_number: String,
    pub pull_request_title: String,
}
