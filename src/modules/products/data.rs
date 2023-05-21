use diesel::prelude::*;
use std::error::Error;

use crate::connectors::postgres::schemas::products::dsl::*;

use super::models::Product;

type DbError = Box<dyn Error + Send + Sync>;

pub fn get_all_products(conn: &mut PgConnection) -> Result<Vec<Product>, DbError> {
    let found = products
        .load::<Product>(conn)
        .expect("Error loading products");
    Ok(found)
}

pub fn get_by_id(product_id: String, conn: &mut PgConnection) -> Result<Product, DbError> {
    let found = products
        .filter(id.eq(product_id))
        .first::<Product>(conn)
        .expect("Error loading product");
    Ok(found)
}

pub fn add_product(product: Product, conn: &mut PgConnection) -> Result<Product, DbError> {
    let entity = diesel::insert_into(products)
        .values(product)
        .get_result::<Product>(conn)
        .expect("Error creating product");
    Ok(entity)
}

pub fn update_product(
    product_id: String,
    product: Product,
    conn: &mut PgConnection,
) -> Result<Product, DbError> {
    let entity = diesel::update(products.find(product_id))
        .set(product)
        .get_result::<Product>(conn)
        .expect("Error updating product");
    Ok(entity)
}

pub fn delete_product(product_id: String, conn: &mut PgConnection) -> Result<(), DbError> {
    diesel::delete(products.find(product_id))
        .execute(conn)
        .expect("Error deleting product");
    Ok(())
}
