use rust_decimal::Decimal;
use sqlx::PgPool;
use uuid::Uuid;

use crate::domain::product::Product;

#[derive(Debug, Clone)]
pub struct ProductRepository {
    pub db: PgPool,
}

impl ProductRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { db: pool }
    }

    pub async fn find_by_id(&self, id: Uuid) -> Result<Option<Product>, sqlx::Error> {
        let product = sqlx::query_as!(
            Product,
            r#"
            SELECT id, name, description, price, stock, created_at, updated_at
            FROM products
            WHERE id = $1
            "#,
            id
        )
        .fetch_optional(&self.db)
        .await?;

        Ok(product)
    }

    pub async fn find_all(&self, limit: i64, offset: i64) -> Result<Vec<Product>, sqlx::Error> {
        let products = sqlx::query_as::<_, Product>(
            r#"
            SELECT id, name, description, price, stock, created_at, updated_at FROM products
            ORDER BY created_at DESC
            
            LIMIT $1 OFFSET $2
            "#,
        )
        .bind(limit)
        .bind(offset)
        .fetch_all(&self.db)
        .await?;

        Ok(products)
    }

    pub async fn create(
        &self,
        name: &str,
        description: Option<&str>,
        price: Decimal,
        stock: i32,
    ) -> Result<Product, sqlx::Error> {
        let id = Uuid::new_v4();

        let product = sqlx::query_as::<_, Product>(
            r#"
            INSERT INTO products (
                id,
                name,
                description,
                price,
                stock
            )
            VALUES ($1, $2, $3, $4, $5)
            RETURNING
                id,
                name,
                description,
                price,
                stock,
                created_at,
                updated_at
            "#,
        )
        .bind(id)
        .bind(name)
        .bind(description)
        .bind(price)
        .bind(stock)
        .fetch_one(&self.db)
        .await?;
        Ok(product)
    }
}
