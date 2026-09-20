use axum::{Json, extract::State};
use reqwest::StatusCode;

use crate::{
    api::{
        dto::order::{CreateOrderRequest, OrderResponse},
        state::AppState,
    },
    application::commands::create_order::{CreateOrderCommand, CreateOrderItemCommand},
    error::ApiError,
};

pub async fn create_order(
    State(state): State<AppState>,
    Json(request): Json<CreateOrderRequest>,
) -> Result<(StatusCode, Json<OrderResponse>), ApiError> {
    let command = CreateOrderCommand {
        user_id: request.user_id,
        items: request
            .items
            .into_iter()
            .map(|item| CreateOrderItemCommand {
                product_id: item.product_id,
                quantity: item.quantity,
            })
            .collect(),
    };

    let order = state.order_service.create_order(command).await?;

    Ok((StatusCode::CREATED, Json(OrderResponse::from(order))))
}
