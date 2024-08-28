use actix_cors::Cors;
use actix_web::{get, App, HttpServer, Responder};

use auth_service::controllers;

#[get("/health-check")]
async fn index() -> impl Responder {
    "Hello world!"
}


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("Starting server...");
    println!("Server running at http://0.0.0.0:8080/");
    println!("Press Ctrl+C to stop the server");

    HttpServer::new( move || {
        let cors = Cors::default();
        App::new()
            .wrap(cors)
            .configure(controllers::auth_controller::config)
            .service(index)
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}
