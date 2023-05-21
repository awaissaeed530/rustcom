pub mod orders;
pub mod products;

use actix_web::{web, Scope};

use orders::orders::order_routes;
use products::products::product_routes;

pub fn app_routes() -> Scope {
    web::scope("")
        .service(order_routes())
        .service(product_routes())
}
