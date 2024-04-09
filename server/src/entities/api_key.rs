use sqlx::FromRow;

#[derive(Debug, PartialEq, Clone, FromRow)]
pub struct ApiKey {
    pub id: sqlx::types::Uuid,
    pub user_id: sqlx::types::Uuid,
    pub key: String,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime,
}
