use chrono::Utc;
use sea_orm::{DatabaseConnection,EntityTrait, Set};
use crate::models::user::{self, Entity as User, Model as UserModel, UserCreate};

pub async fn create_user(
    db: &DatabaseConnection,
    new_user: UserCreate
) -> Result<UserModel, sea_orm::DbErr> {
    let active_model = user::ActiveModel {
        username: Set(new_user.username),
        password: Set(new_user.password),
        email: Set(new_user.email),
        created_at: Set(Utc::now()),
        ..Default::default()
    };

    let result = User::insert(active_model).exec(db).await?;
    let user = User::find_by_id(result.last_insert_id).one(db).await?;
    Ok(user.unwrap())  // panic ร้ายแรงสุด พังไปเลย
}

pub async fn get_user(
    db: &DatabaseConnection,
    user_id: i32,
) -> Result<Option<UserModel>, sea_orm::DbErr> { 
    User::find_by_id(user_id).one(db).await 
}