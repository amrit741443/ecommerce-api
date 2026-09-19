use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct ListProductsQuery {
    #[serde(default = "default_limit")]
    pub limit: i64,
    #[serde(default = "default_offset")]
    pub offset: i64,
}

fn default_limit() -> i64 {
    10
}
fn default_offset() -> i64 {
    0
}
