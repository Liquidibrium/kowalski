use crate::state::AppState;
use axum::extract::State;
use axum::response::IntoResponse;
use axum::Json;
use http::StatusCode;
use crate::models::analyze::PrAnalysisRequest;

#[utoipa::path(post, path = "/api/analyze", responses((status = StatusCode::CREATED, body = PrAnalysisRequest)))]
pub async fn analyze_handler(
    State(_state): State<AppState>,
    Json(pr_analysis_request): Json<PrAnalysisRequest>,
) -> impl IntoResponse {
    println!("Received request to analyze PR: {}", pr_analysis_request.pr_url);
    (StatusCode::CREATED, Json(pr_analysis_request))
}
