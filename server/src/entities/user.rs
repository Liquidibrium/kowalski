use sqlx::types::chrono;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone)]
pub enum Provider {
    Local,
    Google,
    Github,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone)]
pub enum Status {
    Active,
    Pending,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone)]
pub struct UserEntity {
    pub id: sqlx::types::Uuid,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,

    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub email: String,
    pub password: Option<String>,

    pub status: Status,
    pub provider: Status,
}
