use std::io;

use actix_web::{web, App, HttpResponse, HttpServer, Responder};

pub fn general_routes(cfg: &mut web::ServiceConfig) {
    cfg.route("/health", web::get().to(health_check_handler));
}

pub async fn health_check_handler() -> impl Responder {
    HttpResponse::Ok().json("Hello. EzyTutors is alive and kicking")
}

#[actix_web::main]
async fn main() -> io::Result<()> {
    let app = move || App::new().configure(general_routes);
    HttpServer::new(app).bind("127.0.0.1:3000")?.run().await
}