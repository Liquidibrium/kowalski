use serde::Serialize;

#[derive(Serialize)]
pub struct AppResponseError {
    pub message: String,
}

impl AppResponseError {
    pub fn new(message: &str) -> Self {
        AppResponseError {
            message: message.to_string(),
        }
    }
}
