use clap::{Parser, Subcommand, ValueEnum};
use serde::Serialize;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    /// Turn debugging information on
    #[arg(short, long, action = clap::ArgAction::Count)]
    pub debug: u8,

    #[command(subcommand)]
    pub command: Option<Commands>,
}

#[derive(Debug, ValueEnum, Clone, Eq, PartialEq, Serialize)]
pub enum GitProvider {
    Github,
    Gitlab,
    Bitbucket,
}

#[derive(Subcommand)]
pub enum Commands {
    #[command(about = "Analyze code changes in a pull request", alias="analyze")]
    Analysis {
        /// GitHub owner
        #[arg(short, long, required_unless_present_any = & ["pr_link"], requires = "repository")]
        owner: Option<String>,

        /// GitHub repository
        #[arg(short, long, required_unless_present_any = & ["pr_link"], requires = "owner")]
        repository: Option<String>,

        /// Pull request number to Analyze
        #[arg(short, long("pr"), required_unless_present_any = & ["pr_link"])]
        pull_request: Option<u64>,

        /// GitHub pull reqeust link to Analyze
        #[clap(required_unless_present_all = & ["owner", "repository", "pull_request"])]
        pr_link: Option<String>,

        /// GitHub token
        #[arg(short, long, env("GITHUB_TOKEN"))]
        token: Option<String>,

        #[clap(value_enum, default_value_t = GitProvider::Github)]
        #[arg(short, long)]
        git_provider: GitProvider,
    },
}
