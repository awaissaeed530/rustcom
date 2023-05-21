pub mod orders;

use actix_web::{web, Scope};

use orders::orders::order_routes;

pub fn app_routes() -> Scope {
    web::scope("").service(order_routes())
}
