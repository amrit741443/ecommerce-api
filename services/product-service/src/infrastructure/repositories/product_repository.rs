use rust_decimal::Decimal;
use sqlx::{PgPool, Postgres};
use uuid::Uuid;

use crate::{
    domain::{
        product::Product,
        stock_reservation::{ReservationRow, StockReservation, StockReservationResult},
    },
    error::RepositoryError,
};

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

    pub async fn find_by_id_with_executor(
        &self,
        id: Uuid,
        executor: impl sqlx::Executor<'_, Database = Postgres>,
    ) -> Result<Option<Product>, RepositoryError> {
        let product = sqlx::query_as::<_, Product>(
            r#"
                  SELECT
                      id,
                      name,
                      description,
                      price,
                      stock,
                      created_at,
                      updated_at
                  FROM products
                  WHERE id = $1
                  "#,
        )
        .bind(id)
        .fetch_optional(executor)
        .await?;

        Ok(product)
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
        order_id: Uuid,
        idempotency_key: &str,
    ) -> Result<Option<StockReservationResult>, RepositoryError> {
        let mut tx = self.db.begin().await?;

        // 1. Check whether this operation was already processed.

        let existing = sqlx::query_as::<_, ReservationRow>(
            r#"
                SELECT
                    id,
                    order_id,
                    product_id,
                    quantity,
                    status
                FROM stock_reservations
                WHERE idempotency_key = $1
                "#,
        )
        .bind(idempotency_key)
        .fetch_optional(&mut *tx)
        .await?;

        if let Some(existing) = existing {
            // The request was already processed.
            // Return the current product rather than reserving again.

            let product = self
                .find_by_id_with_executor(existing.product_id, &mut *tx)
                .await?;

            tx.commit().await?;
            return Ok(product.map(|product| StockReservationResult {
                reservation_id: existing.id,
                order_id: existing.order_id,
                product,
                quantity: existing.quantity,
                status: existing.status,
            }));
        }

        // 2. Atomically reserve the stock.
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

        let Some(product) = product else {
            tx.rollback().await?;
            return Ok(None);
        };

        let reservation_id = Uuid::new_v4();

        // 3. Record the successful reservation.
        sqlx::query(
            r#"
        INSERT INTO stock_reservations (
            id,
            idempotency_key,
            product_id,
            order_id,
            quantity,
            status
        )
        VALUES ($1, $2, $3, $4, $5, $6)
        "#,
        )
        .bind(reservation_id)
        .bind(idempotency_key)
        .bind(product_id)
        .bind(order_id)
        .bind(quantity)
        .bind("reserved")
        .execute(&mut *tx)
        .await?;

        //4. Make both operations permanent
        tx.commit().await?;

        Ok(Some(StockReservationResult {
            reservation_id,
            order_id,
            product,
            quantity,
            status: "reserved".to_string(),
        }))
    }

    pub async fn release_stock(
        &self,
        reservation_id: Uuid,
    ) -> Result<StockReservation, RepositoryError> {
        let mut tx = self.db.begin().await?;
        let reservation = sqlx::query_as::<_, ReservationRow>(
            r#"
            SELECT
                       id,
                       order_id,
                       product_id,
                       quantity,
                       status
                   FROM stock_reservations
                   WHERE id = $1
                   FOR UPDATE
        "#,
        )
        .bind(reservation_id)
        .fetch_optional(&self.db)
        .await?;

        let Some(reservation) = reservation else {
            return Err(RepositoryError::ReservationNotFound);
        };

        // Already released → idempotent success.
        if reservation.status == "released" {
            tx.commit().await?;
            return reservation.try_into();
        }

        //update release stock
        sqlx::query(
            r#"UPDATE products
        SET
        stock = stock+$2,
        updated_at= NOW()
        WHERE id = $1"#,
        )
        .bind(reservation.product_id)
        .bind(reservation.quantity)
        .execute(&mut *tx)
        .await?;

        // Update status to released
        let reservation = sqlx::query_as::<_, ReservationRow>(
            r#"
        UPDATE stock_reservations
        SET
            status = 'released',
            updated_at = NOW()
        WHERE id = $1
        RETURNING
            id,
            order_id,
            product_id,
            quantity,
            status
        "#,
        )
        .bind(reservation_id)
        .fetch_one(&mut *tx)
        .await?;

        tx.commit().await?;
        reservation.try_into()
    }
}
