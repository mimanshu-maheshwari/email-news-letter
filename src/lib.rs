
pub mod configuration;
pub mod routes;
pub mod startup;

use std::net::TcpListener;
use actix_web::{dev::Server, web, App, HttpRequest, HttpResponse, HttpServer, Responder};

#[derive(serde::Deserialize)]
struct FormData {
    email: String, 
    name: String
}

async fn health_check(_req: HttpRequest) -> impl Responder {
    HttpResponse::Ok()
}

async fn subscriptions(_form: web::Form<FormData>) -> impl Responder {
    HttpResponse::Ok().finish()
}

pub fn run(listener: TcpListener) -> Result<Server, std::io::Error> {
    let server = HttpServer::new(|| {
        App::new()
            .route("/health_check", web::get().to(health_check))
            .route("/subscriptions", web::post().to(subscriptions))
    })
    .listen(listener)?
    .run();
    Ok(server)
}
