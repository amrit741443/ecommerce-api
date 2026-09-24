use uuid::Uuid;

#[derive(Debug)]
pub struct ReleaseStockCommand {
    pub reservation_id: Uuid,
}
