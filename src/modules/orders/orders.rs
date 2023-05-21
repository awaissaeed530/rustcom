use actix_web::{delete, get, post, put, web, HttpResponse, Responder, Scope};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Order {
    id: String,
    reference_no: String,
    sub_total: u16,
    total_price: u16,
    discount_percentage: u16,
    discount_amount: u16,
    items: Vec<OrderItem>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OrderItem {
    id: String,
    item_id: String,
    quantity: u16,
    unit_price: u16,
    unit_cost: u16,
    sub_total: u16,
    total_price: u16,
    discount_percentage: u16,
    discount_amount: u16,
}

pub fn order_routes() -> Scope {
    web::scope("/orders")
        .service(find_all)
        .service(find_by_id)
        .service(create)
        .service(update)
        .service(delete)
}

#[get("")]
async fn find_all() -> impl Responder {
    HttpResponse::Ok().body("Find All")
}

#[get("/{id}")]
async fn find_by_id(path: web::Path<String>) -> impl Responder {
    let id = path.into_inner();
    HttpResponse::Ok().body(format!("Find By Id: {}", id))
}

#[post("")]
async fn create(order: web::Json<Order>) -> impl Responder {
    HttpResponse::Created().body("Create")
}

#[put("/{id}")]
async fn update(path: web::Path<String>, order: web::Json<Order>) -> impl Responder {
    HttpResponse::Ok().body("Update")
}

#[delete("/{id}")]
async fn delete(path: web::Path<String>) -> impl Responder {
    let id = path.into_inner();
    HttpResponse::NoContent().body(format!("Delete: {}", id))
}
