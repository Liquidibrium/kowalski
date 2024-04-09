
#[utoipa::path(get, path = "/api/health", responses((status = StatusCode::OK)))]
pub async fn health_check() -> &'static str {
    "status: OK"
}
