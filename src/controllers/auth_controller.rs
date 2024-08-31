use actix_web::{web, HttpResponse, Responder};
use serde::{Deserialize, Serialize};
use crate::domain::{auth::{LoginRequest, Username}, BaseResponse};

#[derive(Debug, Deserialize)]
pub struct RawLoginRequest {
    pub username: String,
    pub password: String,
}

#[derive(Debug, Serialize)]
pub struct LoginResponse {
    #[serde(flatten)]
    base: BaseResponse,
    data: Username,
}


pub async fn login(
    raw_request: web::Json<RawLoginRequest>
) -> impl Responder {
    let login_request = LoginRequest::from_str(&raw_request.username, &raw_request.password);
    match login_request {
        Ok(v) => {
            println!("Login request: {:?}", v);
            let response = LoginResponse {
                base: BaseResponse {
                    code: "200".to_string(),
                    message: "OK".to_string(),
                },
                data: v.username,
            };
            HttpResponse::Ok().json(response)
        },
        Err(e) => {
            println!("Login request error: {:?}", e);
            HttpResponse::BadRequest().json(BaseResponse {
                code: "400".to_string(),
                message: e.to_string(),
            })
        }
    }
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
