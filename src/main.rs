use actix_cors::Cors;
use actix_web::{get, web, App, HttpServer, Responder};

use auth_service::controllers;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("Starting server...");
    println!("Server running at http://0.0.0.0:8080/");
    println!("Press Ctrl+C to stop the server");

    HttpServer::new( move || {
        let cors = Cors::default();
        App::new()
            .wrap(cors)
            .route("/health-check", 
                web::get().to(controllers::health_check::health_check),
            )
            .configure(controllers::auth_controller::config)
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}
