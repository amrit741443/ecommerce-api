use uuid::Uuid;

#[derive(Debug)]
pub struct ReserveStockCommand {
    pub product_id: Uuid,
    pub quantity: i32,
}
