use crate::{
    application::commands::create_product::CreateProductCommand, domain::product::Product,
    infrastructure::repositories::product_repository::ProductRepository,
};

pub struct ProductService {
    repository: ProductRepository,
}

impl ProductService {
    pub fn new(repository: ProductRepository) -> Self {
        ProductService { repository }
    }

    pub async fn create_product(
        &self,
        command: CreateProductCommand,
    ) -> Result<Product, sqlx::Error> {
        self.repository
            .create(
                &command.name,
                command.description.as_deref(),
                command.price,
                command.stock,
            )
            .await
    }

    pub async fn get_product(
        &self,
        product_id: uuid::Uuid,
    ) -> Result<Option<Product>, sqlx::Error> {
        self.repository.find_by_id(product_id).await
    }

    pub async fn list_products(
        &self,
        limit: i64,
        offset: i64,
    ) -> Result<Vec<Product>, sqlx::Error> {
        self.repository.find_all(limit, offset).await
    }
}
