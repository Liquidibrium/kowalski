use clap::ValueEnum;
use serde::Serialize;

#[derive(Debug, ValueEnum, Clone, Eq, PartialEq, Serialize)]
pub enum GitProvider {
    Github,
    Gitlab,
    Bitbucket,
}
