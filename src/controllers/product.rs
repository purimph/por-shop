use crate::{error::ApiError, services::product_service};
use actix_web::{web, HttpResponse};
use rust_decimal::Decimal;
use sea_orm::DatabaseConnection;
use serde::Deserialize;
use serde_with::{serde_as, FromInto};

pub async fn get_products(
    db: web::Data<DatabaseConnection>,
) -> Result<impl actix_web::Responder, ApiError> {
    let products = product_service::get_all_products(&db).await?;
    Ok(HttpResponse::Ok().json(products))
    // match product_service::get_all_products(&db).await {
    //     Ok(products) => HttpResponse::Ok().json(products),
    //     Err(_) => HttpResponse::InternalServerError().body("Error"),
    // }
}

pub async fn get_product(
    db: web::Data<DatabaseConnection>,
    product_id: web::Path<uuid::Uuid>,
) -> Result<impl actix_web::Responder, ApiError> {
    let products = product_service::get_product_by_id(&db, product_id.into_inner()).await?;
    Ok(HttpResponse::Ok().json(products))
    // match product_service::get_product_by_id(&db, product_id.into_inner()).await {
    //     Ok(Some(product)) => HttpResponse::Ok().json(product),
    //     Ok(None) => HttpResponse::NotFound().body("product not found"),
    //     Err(_) => HttpResponse::InternalServerError().body("Error"),
    // }
}

#[serde_as]
#[derive(Deserialize)]

pub struct ProductRequest {
    pub name: String,
    pub description: Option<String>,
    #[serde_as(as = "FromInto<Decimal>")]
    pub price: Decimal,
}

pub async fn create_product(
    data: web::Json<ProductRequest>,
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
    // let product_id = product_id.into_inner();
    match product_service::delete_product(&db, product_id.into_inner()).await {
        Ok(response) if response.rows_affected > 0 => HttpResponse::Ok().body("Deleted"),
        Ok(_) => HttpResponse::NotFound().body("Product not found"),
        Err(_) => HttpResponse::InternalServerError().body("Error"),
    }
}

pub async fn update_product(
    data: web::Json<ProductRequest>,
    db: web::Data<DatabaseConnection>,
    product_id: web::Path<uuid::Uuid>,
) -> Result<impl actix_web::Responder, ApiError> {
    let response =
        product_service::update_product(&db, product_id.into_inner(), data.into_inner()).await?;
    Ok(HttpResponse::Ok().json(response))
}
