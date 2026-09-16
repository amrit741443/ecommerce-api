use rust_decimal::Decimal;

pub struct CreateProductCommand {
    pub name: String,
    pub description: Option<String>,
    pub price: Decimal,
    pub stock: i32,
}
