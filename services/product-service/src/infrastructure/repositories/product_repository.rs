use rust_decimal::Decimal;
use sqlx::PgPool;
use uuid::Uuid;

use crate::{domain::product::Product, error::RepositoryError};

#[derive(Debug, Clone)]
pub struct ProductRepository {
    pub db: PgPool,
}

impl ProductRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { db: pool }
    }

    pub async fn find_by_id(&self, id: Uuid) -> Result<Option<Product>, RepositoryError> {
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

    pub async fn find_all(&self, limit: i64, offset: i64) -> Result<Vec<Product>, RepositoryError> {
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

        println!("Found {} products", products.len());
        Ok(products)
    }

    pub async fn create(
        &self,
        name: &str,
        description: Option<&str>,
        price: Decimal,
        stock: i32,
    ) -> Result<Product, RepositoryError> {
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

    pub async fn update(
        &self,
        id: Uuid,
        name: Option<&str>,
        description: Option<&str>,
        price: Option<Decimal>,
        stock: Option<i32>,
    ) -> Result<Product, RepositoryError> {
        let product = sqlx::query_as::<_, Product>(
            r#"
UPDATE products
SET
name = COALESCE($2, name),
            description = COALESCE($3, description),
            price = COALESCE($4, price),
            stock = COALESCE($5, stock),
            updated_at = NOW()
WHERE id= $1
RETURNING
id, name, description, price, stock, created_at, updated_at
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

    pub async fn delete(&self, id: Uuid) -> Result<(), RepositoryError> {
        sqlx::query(
            r#"
            DELETE FROM products
            WHERE id = $1
            "#,
        )
        .bind(id)
        .execute(&self.db)
        .await?;
        Ok(())
    }

    pub async fn reserve_stock(
        &self,
        product_id: Uuid,
        quantity: i32,
    ) -> Result<Option<Product>, RepositoryError> {
        let product = sqlx::query_as::<_, Product>(
            r#"
        UPDATE products
        SET 
        stock = stock-$2,
        updated_at= NOW()
        WHERE id = $1
        AND stock >=2
        RETURNING
        id, name, description, price, stock, created_at, updated_at
        
        "#,
        )
        .bind(product_id)
        .bind(quantity)
        .fetch_optional(&self.db)
        .await?;

        Ok(product)
    }

    pub async fn release_stock(
        &self,
        product_id: Uuid,
        quantity: i32,
    ) -> Result<Option<Product>, RepositoryError> {
        let product = sqlx::query_as::<_, Product>(
            r#"
        UPDATE products 
        SET
        stock = stock+ $2,
        updated_at = NOW()
         WHERE id = $1
        RETURNING
        id, name, description, price, stock, created_at, updated_at
        
        "#,
        )
        .bind(product_id)
        .bind(quantity)
        .fetch_optional(&self.db)
        .await?;

        Ok(product)
    }
}
