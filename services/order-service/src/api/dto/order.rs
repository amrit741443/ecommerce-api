use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::domain::order::{Order, OrderStatus};

#[derive(Debug, Deserialize)]
pub struct CreateOrderRequest {
    pub user_id: Uuid,
    pub items: Vec<CreateOrderItemRequest>,
}

#[derive(Debug, Deserialize)]
pub struct CreateOrderItemRequest {
    pub product_id: Uuid,
    pub quantity: i32,
}

#[derive(Debug, Serialize)]
pub struct OrderResponse {
    pub id: Uuid,
    pub user_id: Uuid,
    pub status: String,
    pub total: Decimal,
    pub created_at: String,
    pub updated_at: String,
}

impl From<Order> for OrderResponse {
    fn from(order: Order) -> Self {
        let status = match order.status {
            OrderStatus::Pending => "pending",
            OrderStatus::Confirmed => "confirmed",
            OrderStatus::Cancelled => "cancelled",
            OrderStatus::Paid => "paid",
        };

        Self {
            id: order.id,
            user_id: order.user_id,
            status: status.to_string(),
            total: order.total,
            created_at: order.created_at.to_rfc3339(),
            updated_at: order.updated_at.to_rfc3339(),
        }
    }
}
