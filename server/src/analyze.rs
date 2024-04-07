use crate::state::AppState;
use axum::extract::State;
use axum::response::IntoResponse;
use axum::Json;
use http::StatusCode;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct PrAnalysisRequest {
    pub pr_url: String,
    pub openai_key: String,
    pub cloude_api_key: String,
}

pub async fn analyze_handler(
    State(_state): State<AppState>,
    Json(pr_analysis_request): Json<PrAnalysisRequest>,
) -> impl IntoResponse {
    (StatusCode::CREATED, Json(pr_analysis_request))
}
