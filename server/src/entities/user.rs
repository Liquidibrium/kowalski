use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use strum_macros::{Display, EnumString};

#[derive(
    EnumString,
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
#[sqlx(type_name = "provider", rename_all = "lowercase")]
pub enum AuthProvider {
    Local,
    Google,
    Github,
}

// impl FromStr for AuthProvider {
//     type Err = ();
//
//     fn from_str(s: &str) -> Result<Self, Self::Err> {
//         match s.to_lowercase().as_str() {
//             "local" => Ok(AuthProvider::Local),
//             "google" => Ok(AuthProvider::Google),
//             "github" => Ok(AuthProvider::Github),
//             _ => Err(()),
//         }
//     }
// }

#[derive(
    EnumString,
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
#[sqlx(type_name = "status", rename_all = "lowercase")]
pub enum UserStatus {
    Active,
    Pending,
    Blocked,
}
// impl FromStr for UserStatus {
//     type Err = ();
//
//     fn from_str(s: &str) -> Result<Self, Self::Err> {
//         match s.to_lowercase().as_str() {
//             "active" => Ok(UserStatus::Active),
//             "pending" => Ok(UserStatus::Pending),
//             _ => Err(()),
//         }
//     }
// }

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
