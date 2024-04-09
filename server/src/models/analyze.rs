use serde::{Deserialize, Serialize};
use utoipa::{ToResponse, ToSchema};

#[derive(Deserialize, Serialize, ToSchema, ToResponse, Debug)]
pub struct PrAnalysisRequest {
    pub pr_url: String,
    pub openai_key: String,
    pub cloude_api_key: String,
}

#[derive(Serialize, ToSchema, ToResponse, Debug)]
pub struct ScheduledAnalysis {
    pub analysis_id: String,
}
