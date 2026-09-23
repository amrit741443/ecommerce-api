use axum::{
    Json,
    extract::{Path, Query, State},
    http::{HeaderMap, StatusCode},
};

use uuid::Uuid;

use crate::{
    api::{
        dto::product::{
            CreateProductRequest, ProductListResponse, ProductResponse, ReleaseStockResponse,
            ReserveStockRequest, ReserveStockResponse, UpdateProductRequest,
        },
        state::AppState,
    },
    application::{
        commands::{
            create_product::CreateProductCommand, release_stock::ReleaseStockCommand,
            reserve_stock::ReserveStockCommand, update_prouct::UpdateProductCommand,
        },
        queries::{
            delete_product::DeleteProductQuery, get_product::GetProductQuery,
            list_products::ListProductsQuery,
        },
    },
    error::{ApiError, ApplicationError},
};

pub async fn create_product(
    State(state): State<AppState>,
    Json(request): Json<CreateProductRequest>,
) -> Result<(StatusCode, Json<ProductResponse>), ApiError> {
    let command = CreateProductCommand::new(
        request.name,
        request.description,
        request.price,
        request.stock,
    );

    let prouct = state.product_service.create_product(command).await?;

    Ok((StatusCode::CREATED, Json(ProductResponse::from(prouct))))
}

pub async fn get_product(
    State(state): State<AppState>,
    Path(product_id): Path<Uuid>,
) -> Result<(StatusCode, Json<ProductResponse>), ApiError> {
    let query = GetProductQuery { product_id };

    let product = state.product_service.get_product(query).await?;

    Ok((StatusCode::OK, Json(ProductResponse::from(product))))
}

pub async fn list_products(
    State(state): State<AppState>,
    Query(query): Query<ListProductsQuery>,
) -> Result<(StatusCode, Json<ProductListResponse>), ApiError> {
    let products = state.product_service.list_products(query).await?;

    let product_responses: Vec<ProductResponse> =
        products.into_iter().map(ProductResponse::from).collect();

    Ok((
        StatusCode::OK,
        Json(ProductListResponse {
            total: product_responses.len(),
            products: product_responses,
        }),
    ))
}

pub async fn update_product(
    State(state): State<AppState>,
    Path(product_id): Path<Uuid>,
    Json(request): Json<UpdateProductRequest>,
) -> Result<(StatusCode, Json<ProductResponse>), ApiError> {
    let command = UpdateProductCommand {
        id: product_id,
        name: request.name,
        description: request.description,
        price: request.price,
        stock: request.stock,
    };

    let product = state.product_service.update_product(command).await?;

    Ok((StatusCode::OK, Json(ProductResponse::from(product))))
}

pub async fn delete_product(
    State(state): State<AppState>,
    Path(product_id): Path<Uuid>,
) -> Result<(StatusCode, Json<String>), ApiError> {
    let query = DeleteProductQuery { id: product_id };

    state.product_service.delete_product(query).await?;

    Ok((
        StatusCode::OK,
        Json("Product deleted successfully".to_string()),
    ))
}

//reserve_stock
pub async fn reserve_stock(
    State(state): State<AppState>,
    Path(product_id): Path<Uuid>,
    headers: HeaderMap,
    Json(request): Json<ReserveStockRequest>,
) -> Result<Json<ReserveStockResponse>, ApiError> {
    let idempotency_key = headers
        .get("Idempotency-Key")
        .and_then(|value| value.to_str().ok())
        .ok_or(ApplicationError::MissingIdempotencyKey)?;

    let reservation = state
        .product_service
        .reserve_stock(ReserveStockCommand::new(
            request.order_id,
            product_id,
            request.quantity,
            idempotency_key,
        ))
        .await?;

    Ok(Json(ReserveStockResponse {
        reservation_id: reservation.reservation_id,
        order_id: reservation.order_id,
        product: ProductResponse::from(reservation.product),
        quantity: reservation.quantity,
        status: reservation.status,
    }))
}

//releases stock

pub async fn release_stock(
    State(state): State<AppState>,
    Path(product_id): Path<Uuid>,
    Json(request): Json<ReserveStockRequest>,
) -> Result<Json<ReleaseStockResponse>, ApiError> {
    let product = state
        .product_service
        .release_stock(ReleaseStockCommand {
            product_id,
            quantity: request.quantity,
        })
        .await?;

    Ok(Json(ReleaseStockResponse {
        product_id: product.id,
        reserved_quantity: request.quantity,
        remaining_stock: product.stock,
    }))
}
