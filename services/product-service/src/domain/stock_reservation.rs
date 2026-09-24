use uuid::Uuid;

use crate::{domain::product::Product, error::RepositoryError};

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

impl TryFrom<ReservationRow> for StockReservation {
    type Error = RepositoryError;

    fn try_from(row: ReservationRow) -> Result<Self, Self::Error> {
        let status = match row.status.as_str() {
            "reserved" => ReservationStatus::Reserved,
            "released" => ReservationStatus::Released,
            _ => return Err(RepositoryError::InvalidReservationStatus),
        };

        Ok(Self {
            id: row.id,
            order_id: row.order_id,
            product_id: row.product_id,
            quantity: row.quantity,
            status,
        })
    }
}
