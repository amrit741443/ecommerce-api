use rust_decimal::Decimal;
use sqlx::PgPool;

use crate::{
    application::commands::create_order::CreateOrderCommand,
    domain::order::Order,
    error::ApplicationError,
    infrastructure::{
        clients::product_client::ProductClient, repositories::order_repository::OrderRepository,
    },
};

#[derive(Clone, Debug)]
pub struct OrderService {
    db: PgPool,
    repository: OrderRepository,
    product_client: ProductClient,
}

impl OrderService {
    pub fn new(db: PgPool, repository: OrderRepository, product_client: ProductClient) -> Self {
        Self {
            db,
            repository,
            product_client,
        }
    }

    pub async fn create_order(
        &self,
        command: CreateOrderCommand,
    ) -> Result<Order, ApplicationError> {
        if command.items.is_empty() {
            return Err(ApplicationError::OrderMustContainItems);
        }

        let mut resolved_items = Vec::new();

        for item in &command.items {
            if item.quantity <= 0 {
                return Err(ApplicationError::InvalidQuantity);
            }
            let product = self.product_client.get_product(item.product_id).await?;

            if product.stock < item.quantity {
                return Err(ApplicationError::InsufficientStock);
            }
            resolved_items.push((item.product_id, item.quantity, product.price));
        }

        let total = resolved_items
            .iter()
            .map(|item| item.2 * Decimal::from(item.1))
            .sum();

        let mut tx = self.db.begin().await?;

        let order = self
            .repository
            .create_order(&mut tx, command.user_id, total)
            .await?;

        for (product_id, quantity, unit_price) in resolved_items {
            self.repository
                .create_order_item(&mut tx, order.id, product_id, quantity, unit_price)
                .await?;
        }

        tx.commit().await?;
        Ok(order)
    }
}
