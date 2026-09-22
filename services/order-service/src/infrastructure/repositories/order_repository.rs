use rust_decimal::Decimal;
use uuid::Uuid;

use crate::{
    domain::{
        order::{Order, OrderStatus},
        order_item::OrderItem,
    },
    error::RepositoryError,
};

#[derive(Debug, Clone, Default)]
pub struct OrderRepository;

impl OrderRepository {
    pub fn new() -> Self {
        Self
    }

    pub async fn create_order(
        &self,
        tx: &mut sqlx::Transaction<'_, sqlx::Postgres>,
        user_id: Uuid,
        total: Decimal,
        id: Uuid,
    ) -> Result<Order, RepositoryError> {
        let order = sqlx::query_as::<_, OrderRow>(
            r#"
                    INSERT INTO orders (
                        id,
                        user_id,
                        status,
                        total
                    )
                    VALUES ($1, $2, $3, $4)
                    RETURNING
                        id,
                        user_id,
                        status,
                        total,
                        created_at,
                        updated_at
                    "#,
        )
        .bind(id)
        .bind(user_id)
        .bind("pending")
        .bind(total)
        .fetch_one(&mut **tx)
        .await?;

        order.into_domain()
    }

    pub async fn create_order_item(
        &self,
        tx: &mut sqlx::Transaction<'_, sqlx::Postgres>,
        order_id: Uuid,
        product_id: Uuid,
        quantity: i32,
        unit_price: Decimal,
    ) -> Result<OrderItem, RepositoryError> {
        let id = Uuid::new_v4();

        let item = sqlx::query_as::<_, OrderItem>(
            r#"
                    INSERT INTO order_items (
                        id,
                        order_id,
                        product_id,
                        quantity,
                        unit_price
                    )
                    VALUES ($1, $2, $3, $4, $5)
                    RETURNING
                        id,
                        order_id,
                        product_id,
                        quantity,
                        unit_price,
                        created_at
                    "#,
        )
        .bind(id)
        .bind(order_id)
        .bind(product_id)
        .bind(quantity)
        .bind(unit_price)
        .fetch_one(&mut **tx)
        .await?;

        Ok(item)
    }
}

#[derive(sqlx::FromRow)]
struct OrderRow {
    id: Uuid,
    user_id: Uuid,
    status: String,
    total: Decimal,
    created_at: chrono::DateTime<chrono::Utc>,
    updated_at: chrono::DateTime<chrono::Utc>,
}

impl OrderRow {
    fn into_domain(self) -> Result<Order, RepositoryError> {
        let status = match self.status.as_str() {
            "pending" => OrderStatus::Pending,
            "confirmed" => OrderStatus::Confirmed,
            "cancelled" => OrderStatus::Cancelled,
            "paid" => OrderStatus::Paid,
            _ => {
                return Err(RepositoryError::InvalidOrderStatus);
            }
        };

        Ok(Order {
            id: self.id,
            user_id: self.user_id,
            status,
            total: self.total,
            created_at: self.created_at,
            updated_at: self.updated_at,
        })
    }
}
