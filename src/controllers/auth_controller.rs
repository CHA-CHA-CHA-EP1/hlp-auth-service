use actix_web::{web, HttpResponse, Responder};
use serde::{Deserialize, Serialize};
use validator::Validate;
use crate::{domain::{auth::{LoginRequest, Username}, user::UserSignup, BaseResponse}, services::auth_service::AuthService, AppState};

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
    raw_request: web::Json<RawLoginRequest>,
    service: web::Data<AppState>
) -> impl Responder {
    let login_request = LoginRequest::from_str(&raw_request.username, &raw_request.password);
    if let Err(e) = login_request {
        return HttpResponse::BadRequest().json(BaseResponse {
            code: "400".to_string(),
            message: e.to_string(),
        });
    }

    let login_request = login_request.unwrap();
    let token = service.auth_service.login(login_request.username, login_request.password).await;
    
    if let Err(e) = token {
        return HttpResponse::Unauthorized().json(BaseResponse {
            code: "401".to_string(),
            message: e,
        });
    }

    HttpResponse::Ok().json(BaseResponse {
        code: "200".to_string(),
        message: "Success".to_string(),
        //data: login_request.username,
    })
}

pub async fn logout() -> impl actix_web::Responder {
    actix_web::HttpResponse::Ok().finish()
}

pub async fn register(
        request: web::Json<UserSignup>
    ) -> impl actix_web::Responder {
    if let Err(e) = request.validate() {
        return actix_web::HttpResponse::BadRequest().json(BaseResponse {
            code: "400".to_string(),
            message: e.to_string(),
        });
    }
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
