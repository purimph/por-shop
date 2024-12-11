use crate::entity::products;
use rust_decimal::Decimal;
use sea_orm::{ActiveModelTrait, DatabaseConnection, DbErr, DeleteResult, EntityTrait, Set};

pub async fn get_all_products(db: &DatabaseConnection) -> Result<Vec<products::Model>, DbErr> {
    products::Entity::find().all(db).await
}

pub async fn get_product_by_id(
    db: &DatabaseConnection,
    product_id: uuid::Uuid,
) -> Result<Option<products::Model>, DbErr> {
    products::Entity::find_by_id(product_id).one(db).await
}

pub async fn create_product(
    db: &DatabaseConnection,
    name: String,
    description: Option<String>,
    price: Decimal,
) -> Result<products::Model, DbErr> {
    let new_product = products::ActiveModel {
        id: Set(uuid::Uuid::new_v4()),
        name: Set(name),
        description: Set(description),
        price: Set(price),
        created_at: Set(chrono::Utc::now()),
    };
    new_product.insert(db).await
}

pub async fn delete_product(
    db: &DatabaseConnection,
    product_id: uuid::Uuid,
) -> Result<DeleteResult, DbErr> {
    products::Entity::delete_by_id(product_id).exec(db).await
}
