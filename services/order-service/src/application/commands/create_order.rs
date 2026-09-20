use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct CreateOrderItemCommand {
    pub product_id: Uuid,
    pub quantity: i32,
}

#[derive(Debug, Clone)]
pub struct CreateOrderCommand {
    pub user_id: Uuid,
    pub items: Vec<CreateOrderItemCommand>,
}
