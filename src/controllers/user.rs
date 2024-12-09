use actix_web::{web, HttpResponse};
use sea_orm::DatabaseConnection;
use crate::models::user::UserCreate;
use crate::services::user_service;

pub async fn create_user(
    db: web::Data<DatabaseConnection>,
    payload: web::Json<UserCreate>,
    ) -> HttpResponse {
            match user_service::create_user(db.get_ref(), payload.into_inner()).await {
                Ok(user) => HttpResponse::Created().json(user),
                Err(err) => {
                    HttpResponse::InternalServerError().body(err.to_string())
            }
        }
    }
    pub async fn get_user(
        db: web::Data<DatabaseConnection>,
        user_id: web::Path<i32>,
    ) -> HttpResponse {
        match user_service::get_user(db.get_ref(), user_id.into_inner()).await {
            Ok(Some(user)) => HttpResponse::Ok().json(user),
            Ok(None) => HttpResponse::NotFound().body("User not found"),
            Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
        }
    }