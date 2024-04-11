use std::str::FromStr;
use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use strum_macros::{Display, EnumString};

#[derive(
    Serialize,
    Deserialize,
    Display,
    Debug,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Clone,
    sqlx::Type,
)]
#[sqlx(rename_all = "lowercase")]
pub enum AuthProvider {
    Local,
    Google,
    Github,
}

impl From<String> for AuthProvider {
    fn from(value: String) -> Self {
        match value.to_lowercase().as_str() {
            "local" => AuthProvider::Local,
            "google" => AuthProvider::Google,
            "github" => AuthProvider::Github,
            _ => AuthProvider::Local,
        }
    }
}

#[derive(
    Serialize,
    Deserialize,
    Display,
    Debug,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Clone,
    sqlx::Type,
)]
#[sqlx(rename_all = "lowercase")]
pub enum UserStatus {
    Active,
    Pending,
    Blocked,
}
impl From<String> for UserStatus {
    fn from(value: String) -> Self {
        match value.to_lowercase().as_str() {
            "active" => UserStatus::Active,
            "pending" => UserStatus::Pending,
            "blocked" => UserStatus::Blocked,
            _ => UserStatus::Pending,
        }
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, FromRow)]
pub struct UserEntity {
    pub id: sqlx::types::Uuid,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,

    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub email: String,
    pub password: Option<String>,

    pub status: UserStatus,
    pub provider: AuthProvider,
}
