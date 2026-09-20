use crate::application::product_service::ProductService;

#[derive(Clone)]
pub struct AppState {
    pub product_service: ProductService,
}
