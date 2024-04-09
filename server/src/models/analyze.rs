use serde::{Deserialize, Serialize};
use utoipa::{ToResponse, ToSchema};

#[derive(Deserialize, Serialize, ToSchema, ToResponse, Debug)]
pub struct PrAnalysisRequest {
    pub pr_url: String,
    pub openai_key: String,
    pub cloude_api_key: String,
}