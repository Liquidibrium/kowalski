use kowalski_core::git_provider::git::GitProvider;
use sqlx::FromRow;

#[derive(Debug, PartialEq, Clone, FromRow)]
pub struct Repository {
    pub id: sqlx::types::Uuid,
    pub team_id: sqlx::types::Uuid,

    pub name: String,
    pub owner: String,
    pub url: String,

    pub git_provider: GitProvider,

    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime,
}
