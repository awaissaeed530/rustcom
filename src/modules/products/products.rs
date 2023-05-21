use actix_web::{delete, error, get, post, put, web, HttpResponse, Responder, Result, Scope};

use crate::{
    connectors::postgres::pool::DbPool,
    modules::products::models::{CreateProductDto, Product},
};

pub fn product_routes() -> Scope {
    web::scope("/products")
        .service(find_all)
        .service(find_by_id)
        .service(create)
        .service(update)
        .service(delete)
}

#[get("")]
async fn find_all(pool: web::Data<DbPool>) -> Result<impl Responder> {
    let products = web::block(move || {
        let mut conn = pool.get()?;
        super::data::get_all_products(&mut conn)
    })
    .await?
    .map_err(error::ErrorInternalServerError)?;
    Ok(HttpResponse::Ok().json(products))
}

#[get("/{id}")]
async fn find_by_id(path: web::Path<String>, pool: web::Data<DbPool>) -> Result<impl Responder> {
    let id = path.into_inner();
    let product = web::block(move || {
        let mut conn = pool.get()?;
        super::data::get_by_id(id, &mut conn)
    })
    .await?
    .map_err(error::ErrorInternalServerError)?;
    Ok(HttpResponse::Ok().json(product))
}

#[post("")]
async fn create(
    dto: web::Json<CreateProductDto>,
    pool: web::Data<DbPool>,
) -> Result<impl Responder> {
    let product = Product::from(dto.into_inner());
    let entity = web::block(move || {
        let mut conn = pool.get()?;
        super::data::add_product(product, &mut conn)
    })
    .await?
    .map_err(error::ErrorInternalServerError)?;
    Ok(HttpResponse::Created().json(entity))
}

#[put("/{id}")]
async fn update(
    path: web::Path<String>,
    product: web::Json<Product>,
    pool: web::Data<DbPool>,
) -> Result<impl Responder> {
    let product_id = path.into_inner();
    let product = product.into_inner();

    if product_id.ne(&product.id) {
        return Ok(HttpResponse::BadRequest().body("Invalid Request"));
    }

    let entity = web::block(move || {
        let mut conn = pool.get()?;
        super::data::update_product(product_id, product, &mut conn)
    })
    .await?
    .map_err(error::ErrorInternalServerError)?;
    Ok(HttpResponse::Ok().json(entity))
}

#[delete("/{id}")]
async fn delete(path: web::Path<String>, pool: web::Data<DbPool>) -> Result<impl Responder> {
    let id = path.into_inner();
    web::block(move || {
        let mut conn = pool.get()?;
        super::data::delete_product(id, &mut conn)
    })
    .await?
    .map_err(error::ErrorInternalServerError)?;
    Ok(HttpResponse::NoContent())
}
