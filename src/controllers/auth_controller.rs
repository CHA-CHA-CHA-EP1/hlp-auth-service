use actix_web::web;

use crate::domain::auth::LoginRequest;

pub async fn login(request: web::Json<LoginRequest>) -> impl actix_web::Responder {
    let request = request.into_inner();
    println!("{:?}", request);
    actix_web::HttpResponse::Ok().finish()
}

pub async fn logout() -> impl actix_web::Responder {
    actix_web::HttpResponse::Ok().finish()
}

pub fn config(cfg: &mut web::ServiceConfig) {
    let scope = web::scope("/api/v1/auth");
    cfg.service(
        scope
            .route("/login", web::post().to(login))
            .route("/logout", web::post().to(logout)),
    );
}
