use serde::Serialize;

pub mod auth;

#[derive(Debug, Serialize)]
pub struct BaseResponse {
    pub code: String,
    pub message: String,
}
