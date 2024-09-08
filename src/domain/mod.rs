use serde::Serialize;

pub mod auth;
pub mod user;

#[derive(Debug, Serialize)]
pub struct BaseResponse {
    pub code: String,
    pub message: String,
}
