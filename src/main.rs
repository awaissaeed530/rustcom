mod connectors;
mod modules;

use actix_web::{middleware, web, App, HttpServer};
use connectors::postgres::pool::configure_database;
use dotenv::dotenv;
use modules::app_routes;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    HttpServer::new(|| {
        App::new()
            .app_data(web::Data::new(configure_database().clone()))
            .wrap(middleware::Logger::default())
            .service(app_routes())
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
