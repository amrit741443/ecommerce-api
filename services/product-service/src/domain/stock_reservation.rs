use uuid::Uuid;

use crate::domain::product::Product;

#[derive(Debug)]
pub struct StockReservation {
    pub id: Uuid,
    pub order_id: Uuid,
    pub product_id: Uuid,
    pub quantity: i32,
    pub status: ReservationStatus,
}

#[derive(Debug, sqlx::FromRow)]
pub struct ReservationRow {
    pub id: Uuid,
    pub order_id: Uuid,
    pub product_id: Uuid,
    pub quantity: i32,
    pub status: String,
}

#[derive(Debug, Clone)]
pub enum ReservationStatus {
    Reserved,
    Released,
}

pub struct StockReservationResult {
    pub reservation_id: Uuid,
    pub order_id: Uuid,
    pub product: Product,
    pub quantity: i32,
    pub status: String,
}
