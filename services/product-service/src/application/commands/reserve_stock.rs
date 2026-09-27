use uuid::Uuid;

#[derive(Debug)]
pub struct ReserveStockCommand<'a> {
    pub order_id: Uuid,
    pub product_id: Uuid,
    pub quantity: i32,
    pub idempotency_key: &'a str,
}

impl<'a> ReserveStockCommand<'a> {
    pub fn new(order_id: Uuid, product_id: Uuid, quantity: i32, idempotency_key: &'a str) -> Self {
        Self {
            order_id,
            product_id,
            quantity,
            idempotency_key,
        }
    }
}
