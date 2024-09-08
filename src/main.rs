use actix_cors::Cors;
use actix_web::{web, App, HttpServer};

use auth_service::controllers;
use auth_service::services;
use auth_service::AppState;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("Starting server...");
    println!("Server running at http://0.0.0.0:8080/");
    println!("Press Ctrl+C to stop the server");

    let auth_service = services::auth_service::AuthServiceImpl::new();

    HttpServer::new(move || {
        let cors = Cors::default();
        App::new()
            .wrap(cors)
            .app_data(web::Data::new(AppState {
                auth_service: auth_service.clone(),
            }))
            .route(
                "/health-check",
                web::get().to(controllers::health_check::health_check),
            )
            .configure(controllers::auth_controller::config)
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}
