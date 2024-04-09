use sqlx::FromRow;

#[derive(Debug, PartialEq, Clone, FromRow)]
pub struct Team {
    pub id: sqlx::types::Uuid,
    pub name: String,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime,
}
