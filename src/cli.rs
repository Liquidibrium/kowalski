use crate::git_provider::git::GitProvider;
use crate::llm::models::{EmbeddingModel, LlmModel};
use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    /// Turn debugging information on
    #[arg(short, long, action = clap::ArgAction::Count)]
    pub debug: u8,

    #[command(subcommand)]
    pub command: Option<Commands>,
}

#[derive(Subcommand)]
pub enum Commands {
    #[command(about = "Analyze code changes in a pull request", alias = "analyze")]
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
        #[arg(long, env("GIT_TOKEN"))]
        git_token: Option<String>,

        #[clap(value_enum, default_value_t = GitProvider::Github)]
        #[arg(short, long)]
        git_provider: GitProvider,

        #[clap(value_enum, default_value_t = LlmModel::Gpt3_5Turbo)]
        #[arg(short, long)]
        llm_model: LlmModel,

        #[clap(value_enum, default_value_t = EmbeddingModel::OpenAiTextEmbedding3Small)]
        #[arg(short, long)]
        embedding_model: EmbeddingModel,

        #[arg(long, env = "OPENAI_API_KEY")]
        openai_api_key: Option<String>,

        #[arg(long, env = "ANTHROPIC_API_KEY")]
        anthropic_api_key: Option<String>,
    },
}
