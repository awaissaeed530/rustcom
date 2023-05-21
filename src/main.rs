mod modules;

use actix_web::{App, HttpServer};
use modules::app_routes;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().service(app_routes()))
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}
