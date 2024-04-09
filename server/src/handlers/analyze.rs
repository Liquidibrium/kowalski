use crate::models::analyze::{PrAnalysisRequest, ScheduledAnalysis};
use crate::state::AppState;
use axum::extract::State;
use axum::response::IntoResponse;
use axum::Json;
use http::StatusCode;

#[utoipa::path(post, path = "/api/analyze", responses((status = StatusCode::CREATED, body = ScheduledAnalysis)))]
pub async fn analyze_handler(
    State(_state): State<AppState>,
    Json(pr_analysis_request): Json<PrAnalysisRequest>,
) -> impl IntoResponse {
    println!(
        "Received request to analyze PR: {}",
        pr_analysis_request.pr_url
    );

    let id = uuid::Uuid::now_v7();
    let response = ScheduledAnalysis {
        analysis_id: id.to_string(),
    };
    (StatusCode::CREATED, Json(response))
}
