use uuid::Uuid;

#[derive(Debug)]
pub struct ReserveStockCommand<'a> {
    pub product_id: Uuid,
    pub quantity: i32,
    pub idempotency_key: &'a str,
}

impl<'a> ReserveStockCommand<'a> {
    pub fn new(product_id: Uuid, quantity: i32, idempotency_key: &'a str) -> Self {
        Self {
            product_id,
            quantity,
            idempotency_key,
        }
    }
}
