use crate::{controllers::product::ProductRequest, entity::products, repository};
// use rust_decimal::Decimal;
use sea_orm::{DatabaseConnection, DbErr, DeleteResult};

use super::error::ServiceError;

pub async fn get_all_products(
    db: &DatabaseConnection,
) -> Result<Vec<products::Model>, ServiceError> {
    // if true {
    //     return Err(ServiceError::NotWork);
    // }
    repository::product::get_all_products(db)
        .await
        .map_err(|e| ServiceError::Database(e))
    // products::Entity::find().all(db).await
}

pub async fn get_product_by_id(
    db: &DatabaseConnection,
    product_id: uuid::Uuid,
) -> Result<Option<products::Model>, DbErr> {
    repository::product::get_product_by_id(db, product_id).await
    // products::Entity::find_by_id(product_id).one(db).await
}

pub async fn create_product(
    db: &DatabaseConnection,
    data: ProductRequest,
) -> Result<products::Model, DbErr> {
    repository::product::create_product(db, data).await
}

pub async fn delete_product(
    db: &DatabaseConnection,
    product_id: uuid::Uuid,
) -> Result<DeleteResult, DbErr> {
    repository::product::delete_product(db, product_id).await
}

pub async fn update_product(
    db: &DatabaseConnection,
    product_id: uuid::Uuid,
    data_product: ProductRequest,
) -> Result<products::Model, DbErr> {
    repository::product::update_product(db, product_id, data_product).await
}
