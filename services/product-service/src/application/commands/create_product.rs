use rust_decimal::Decimal;

pub struct CreateProductCommand {
    pub name: String,
    pub description: Option<String>,
    pub price: Decimal,
    pub stock: i32,
}

impl CreateProductCommand {
    pub fn new(name: String, description: Option<String>, price: Decimal, stock: i32) -> Self {
        Self {
            name,
            description,
            price,
            stock,
        }
    }
}
