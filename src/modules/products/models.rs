use diesel::{Queryable, Insertable, Identifiable, AsChangeset};
use serde::{Serialize, Deserialize};
use uuid::Uuid;

use crate::connectors::postgres::schemas::products;

#[derive(Debug, Serialize, Deserialize, Queryable, Insertable, Identifiable, AsChangeset)]
#[diesel(table_name = products)]
pub struct Product {
    pub id: String,
    pub name: String,
    pub description: Option<String>,
}

impl From<CreateProductDto> for Product {
    fn from(dto: CreateProductDto) -> Self {
        Product {
            id: Uuid::new_v4().to_string(),
            name: dto.name,
            description: dto.description,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateProductDto {
    pub name: String,
    pub description: Option<String>,
}