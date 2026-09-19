use rust_decimal::Decimal;
use uuid::Uuid;

pub struct UpdateProductCommand {
    pub id: Uuid,
    pub name: Option<String>,
    pub price: Option<Decimal>,
    pub stock: Option<i32>,
    pub description: Option<String>,
}
