use crate::entity::user::{self, ActiveModel};
use crate::error::ApiError;
use crate::services::auth::{generate_jwt, hash_password, verify_password};
use actix_web::{web, HttpResponse};
use sea_orm::{ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter, Set};
use serde::Deserialize;
use validator::Validate;

#[derive(Deserialize)]
pub struct RegisterData {
    pub username: String,
    pub email: String,
    pub password: String,
}

pub async fn register(
    data: web::Json<RegisterData>,
    db: web::Data<DatabaseConnection>,
) -> HttpResponse {
    let hashed_password = hash_password(&data.password);
    let new_user = ActiveModel {
        id: Set(uuid::Uuid::new_v4()),
        username: Set(data.username.clone()),
        email: Set(data.email.clone()),
        hashed_password: Set(hashed_password),
        ..Default::default()
    };

    if let Err(err) = new_user.insert(db.as_ref()).await {
        println!("Error inserting user: {}", err); // Debug print
        return HttpResponse::BadRequest().body("Error inserting user");
    }
    HttpResponse::Ok().body("User registered")
}

#[derive(Debug, Deserialize, Validate)]
pub struct LoginData {
    pub username: String,
    #[validate(length(min = 8))]
    pub password: String,
}

pub async fn login(
    data: web::Json<LoginData>,
    db: web::Data<DatabaseConnection>,
) -> Result<impl actix_web::Responder, ApiError> {
    data.validate()?;
    let user = user::Entity::find()
        .filter(user::Column::Username.eq(data.username.clone()))
        .one(db.as_ref())
        .await?
        .ok_or(ApiError::AuthenticationError(
            "Invalid username or password".into(),
        ))?;

    // หากพบผู้ใช้ ตรวจสอบรหัสผ่าน
    if verify_password(&data.password, &user.hashed_password) {
        let token = generate_jwt(&user.id.to_string());
        return Ok(HttpResponse::Ok().json(token));
    } else {
        return Err(ApiError::AuthenticationError(
            "Invalid username or password".into(),
        ));
    }
}
