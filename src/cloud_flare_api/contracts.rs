use serde::*;

#[derive(Debug, Serialize, Deserialize)]
pub struct ErrorContract {
    pub code: u32,
    pub message: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateModelResponse {
    pub success: bool,
    pub errors: Option<Vec<ErrorContract>>,
}
