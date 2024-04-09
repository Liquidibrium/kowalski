use clap::ValueEnum;
use serde::Serialize;
use strum_macros::EnumString;

#[derive(Debug, ValueEnum, Clone, Eq, PartialEq, Serialize, EnumString)]
pub enum GitProvider {
    Github,
    Gitlab,
    Bitbucket,
}
