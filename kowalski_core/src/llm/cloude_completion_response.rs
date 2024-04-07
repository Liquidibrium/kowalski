use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct CloudeCompletionResponse {
    #[serde(rename = "completion")]
    pub completion: String,

    #[serde(rename = "stop_reason")]
    pub stop_reason: String,

    #[serde(rename = "model")]
    model: String,

    #[serde(rename = "id")]
    id: String,

    #[serde(rename = "type")]
    cloude_completion_response_type: String,
}
