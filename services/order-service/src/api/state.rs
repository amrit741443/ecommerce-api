use crate::application::order_service::OrderService;

#[derive(Clone)]
pub struct AppState {
    pub order_service: OrderService,
}
