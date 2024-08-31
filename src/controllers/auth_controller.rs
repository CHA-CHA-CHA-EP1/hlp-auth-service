use actix_web::{web, HttpResponse, Responder};
use serde::Deserialize;
use crate::domain::auth::{LoginRequest, LoginRequestError};

#[derive(Debug, Deserialize)]
pub struct RawLoginRequest {
    pub username: String,
    pub password: String,
}

pub async fn login(
    raw_request: web::Json<RawLoginRequest>
) -> Result<impl Responder, LoginRequestError> {
    // Attempt to create a `LoginRequest` from the provided strings
    let login_request = LoginRequest::from_str(&raw_request.username, &raw_request.password)?;

    // Handle successful login request creation
    // You can proceed with further processing like authentication here
    Ok(HttpResponse::Ok().json(login_request))
}

pub async fn logout() -> impl actix_web::Responder {
    actix_web::HttpResponse::Ok().finish()
}

pub async fn register() -> impl actix_web::Responder {
    actix_web::HttpResponse::Ok().finish()
}

pub fn config(cfg: &mut web::ServiceConfig) {
    let scope = web::scope("/api/v1/auth");
    cfg.service(
        scope
            .route("/login", web::post().to(login))
            .route("/logout", web::post().to(logout))
            .route("/register", web::post().to(register))
    );
}
