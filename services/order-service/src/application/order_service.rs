use rust_decimal::Decimal;
use sqlx::PgPool;
use uuid::Uuid;

use crate::{
    application::commands::create_order::CreateOrderCommand,
    domain::order::Order,
    error::ApplicationError,
    infrastructure::{
        clients::product_client::ProductClient, repositories::order_repository::OrderRepository,
    },
};

struct ReservedItem {
    product_id: Uuid,
    quantity: i32,
    unit_price: Decimal,
}

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

        let mut reserved_items = Vec::new();

        let order_id = Uuid::new_v4();

        for item in &command.items {
            if item.quantity <= 0 {
                return Err(ApplicationError::InvalidQuantity);
            }

            let product = self.product_client.get_product(item.product_id).await?;

            let reservation_key = format!("order:{}:product:{}", order_id, item.product_id);

            match self
                .product_client
                .reserve_stock(item.product_id, item.quantity, &reservation_key)
                .await
            {
                Ok(_) => {
                    reserved_items.push(ReservedItem {
                        product_id: item.product_id,
                        quantity: item.quantity,
                        unit_price: product.price,
                    });
                }
                Err(error) => {
                    self.release_reserved_item(&reserved_items).await;

                    return Err(error.into());
                }
            }
        }

        let total = reserved_items
            .iter()
            .map(|item| item.unit_price * Decimal::from(item.quantity))
            .sum();

        let mut tx = self.db.begin().await?;

        let order = self
            .repository
            .create_order(&mut tx, command.user_id, total, order_id)
            .await?;

        for item in reserved_items {
            self.repository
                .create_order_item(
                    &mut tx,
                    order.id,
                    item.product_id,
                    item.quantity,
                    item.unit_price,
                )
                .await?;
        }

        tx.commit().await?;
        Ok(order)
    }

    async fn release_reserved_item(&self, items: &[ReservedItem]) {
        for item in items {
            if let Err(error) = self
                .product_client
                .release_stock(item.product_id, item.quantity)
                .await
            {
                eprintln!(
                    "failed to release stock for {}: {:?}",
                    item.product_id, error
                );
            }
        }
    }
}
