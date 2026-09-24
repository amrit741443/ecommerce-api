use serde::Serialize;
use uuid::Uuid;

use crate::domain::stock_reservation::{ReservationStatus, StockReservation};

#[derive(Debug, Serialize)]
pub struct ReleaseReservationResponse {
    pub reservation_id: Uuid,
    pub order_id: Uuid,
    pub product_id: Uuid,
    pub quantity: i32,
    pub status: String,
}

impl From<StockReservation> for ReleaseReservationResponse {
    fn from(reservation: StockReservation) -> Self {
        Self {
            reservation_id: reservation.id,
            order_id: reservation.order_id,
            product_id: reservation.product_id,
            quantity: reservation.quantity,
            status: match reservation.status {
                ReservationStatus::Reserved => "reserved".to_string(),
                ReservationStatus::Released => "released".to_string(),
            },
        }
    }
}
