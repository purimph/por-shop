use crate::services::product_service;
use actix_web::{web, HttpResponse};
use rust_decimal::Decimal;
use sea_orm::DatabaseConnection;
use serde::Deserialize;
use serde_with::{serde_as, FromInto};
// use uuid::Uuid;

pub async fn get_products(db: web::Data<DatabaseConnection>) -> HttpResponse {
    match product_service::get_all_products(&db).await {
        Ok(products) => HttpResponse::Ok().json(products),
        Err(_) => HttpResponse::InternalServerError().body("Error"),
    }
}

pub async fn get_product(
    db: web::Data<DatabaseConnection>,
    product_id: web::Path<uuid::Uuid>,
) -> HttpResponse {
    match product_service::get_product_by_id(&db, product_id.into_inner()).await {
        Ok(Some(product)) => HttpResponse::Ok().json(product),
        Ok(None) => HttpResponse::NotFound().body("product not found"),
        Err(_) => HttpResponse::InternalServerError().body("Error"),
    }
}

#[serde_as]
#[derive(Deserialize)]

pub struct CreateProductRequest {
    name: String,
    description: Option<String>,
    #[serde_as(as = "FromInto<Decimal>")]
    price: Decimal,
}

pub async fn create_product(
    data: web::Json<CreateProductRequest>,
    db: web::Data<DatabaseConnection>,
) -> HttpResponse {
    match product_service::create_product(
        &db,
        data.name.clone(),
        data.description.clone(),
        data.price,
    )
    .await
    {
        Ok(product) => HttpResponse::Ok().json(product),
        Err(_) => HttpResponse::InternalServerError().body("Error najaa"),
    }
}

pub async fn delete_product(
    db: web::Data<DatabaseConnection>,
    product_id: web::Path<uuid::Uuid>,
) -> HttpResponse {
    println!("{}", product_id);
    match product_service::delete_product(&db, product_id.into_inner()).await {
        Ok(response) if response.rows_affected > 0 => HttpResponse::Ok().body("Deleted"),
        Ok(_) => HttpResponse::NotFound().body("product not found"),
        Err(_) => HttpResponse::InternalServerError().body("Error"),
    }
}
