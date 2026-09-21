use uuid::Uuid;

#[derive(Debug)]
pub struct ReleaseStockCommand {
    pub product_id: Uuid,
    pub quantity: i32,
}
