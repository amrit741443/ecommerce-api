use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::domain::product::Product;

#[derive(Debug, Deserialize)]
pub struct CreateProductRequest {
    pub name: String,
    pub description: Option<String>,
    pub price: Decimal,
    pub stock: i32,
}

#[derive(Debug, Deserialize)]
pub struct UpdateProductRequest {
    pub name: Option<String>,
    pub description: Option<String>,
    pub price: Option<Decimal>,
    pub stock: Option<i32>,
}

#[derive(Debug, Deserialize)]
pub struct ReserveStockRequest {
    pub quantity: i32,
    pub order_id: Uuid,
}

#[derive(Debug, Serialize)]
pub struct ProductResponse {
    pub id: String,
    pub name: String,
    pub description: Option<String>,
    pub price: Decimal,
    pub stock: i32,
    pub created_at: String,
    pub updated_at: String,
}

impl From<Product> for ProductResponse {
    fn from(product: Product) -> Self {
        Self {
            id: product.id.to_string(),
            name: product.name,
            description: product.description,
            price: product.price,
            stock: product.stock,
            created_at: product.created_at.to_rfc3339(),
            updated_at: product.updated_at.to_rfc3339(),
        }
    }
}

#[derive(Debug, Serialize)]
pub struct ProductListResponse {
    pub total: usize,
    pub products: Vec<ProductResponse>,
}

#[derive(Debug, Serialize)]
pub struct ReserveStockResponse {
    pub reservation_id: Uuid,
    pub order_id: Uuid,
    pub product: ProductResponse,
    pub quantity: i32,
    pub status: String,
}
