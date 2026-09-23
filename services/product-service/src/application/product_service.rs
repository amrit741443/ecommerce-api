use rust_decimal::Decimal;

use crate::{
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
    domain::{product::Product, stock_reservation::StockReservationResult},
    error::ApplicationError,
    infrastructure::repositories::product_repository::ProductRepository,
};

#[derive(Clone)]
pub struct ProductService {
    repository: ProductRepository,
}

impl ProductService {
    pub fn new(repository: ProductRepository) -> Self {
        ProductService { repository }
    }

    pub async fn create_product(
        &self,
        mut command: CreateProductCommand,
    ) -> Result<Product, ApplicationError> {
        command.name = command.name.trim().to_string();

        if command.name.is_empty() {
            return Err(ApplicationError::InvalidProductName);
        }
        if command.price < Decimal::ZERO {
            return Err(ApplicationError::InvalidProductPrice);
        }
        if command.stock.is_negative() {
            return Err(ApplicationError::InvalidProductStock);
        }
        let product = self
            .repository
            .create(
                &command.name,
                command.description.as_deref(),
                command.price,
                command.stock,
            )
            .await?;

        Ok(product)
    }

    pub async fn get_product(&self, query: GetProductQuery) -> Result<Product, ApplicationError> {
        let product = self.repository.find_by_id(query.product_id).await?;

        product.ok_or(ApplicationError::ProductNotFound)
    }

    pub async fn list_products(
        &self,
        query: ListProductsQuery,
    ) -> Result<Vec<Product>, ApplicationError> {
        let products = self.repository.find_all(query.limit, query.offset).await?;

        Ok(products)
    }

    pub async fn update_product(
        &self,
        mut command: UpdateProductCommand,
    ) -> Result<Product, ApplicationError> {
        let existing_product = self.repository.find_by_id(command.id).await?;

        if existing_product.is_none() {
            return Err(ApplicationError::ProductNotFound);
        }

        if let Some(ref mut name) = command.name {
            *name = name.trim().to_string();

            if name.is_empty() {
                return Err(ApplicationError::InvalidProductName);
            }
        }

        if let Some(price) = command.price
            && price <= Decimal::ZERO
        {
            return Err(ApplicationError::InvalidProductPrice);
        }

        if let Some(stock) = command.stock
            && stock.is_negative()
        {
            return Err(ApplicationError::InvalidProductStock);
        }

        let product = self
            .repository
            .update(
                command.id,
                command.name.as_deref(),
                command.description.as_deref(),
                command.price,
                command.stock,
            )
            .await?;

        Ok(product)
    }

    pub async fn delete_product(&self, query: DeleteProductQuery) -> Result<(), ApplicationError> {
        let existing_product = self.repository.find_by_id(query.id).await?;

        if existing_product.is_none() {
            return Err(ApplicationError::ProductNotFound);
        }

        self.repository.delete(query.id).await?;

        Ok(())
    }

    //Reserve stock
    pub async fn reserve_stock<'a>(
        &self,
        command: ReserveStockCommand<'a>,
    ) -> Result<StockReservationResult, ApplicationError> {
        if command.quantity <= 0 {
            return Err(ApplicationError::InvalidProductStock);
        }

        let product = self
            .repository
            .reserve_stock(
                command.product_id,
                command.quantity,
                command.order_id,
                command.idempotency_key,
            )
            .await?;

        product.ok_or(ApplicationError::InsufficientStock)
    }

    //release stock

    pub async fn release_stock(
        &self,
        command: ReleaseStockCommand,
    ) -> Result<Product, ApplicationError> {
        if command.quantity <= 0 {
            return Err(ApplicationError::InvalidProductStock);
        }

        let product = self
            .repository
            .release_stock(command.product_id, command.quantity)
            .await?;

        product.ok_or(ApplicationError::ProductNotFound)
    }
}
