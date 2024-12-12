use actix_web::{
    error,
    http::{header::ContentType, StatusCode},
    HttpResponse
};
use derive_more::derive::{Display, From};
use sea_orm::DbErr;

#[derive(Debug, Display, From)]
pub enum APIError {
    #[display("Error on Internal server error")]
    DatabaseError(DbErr),
}

impl error::ResponseError for APIError {
    fn error_response(&self) -> HttpResponse {
        HttpResponse::build(self.status_code())
            .insert_header(ContentType::html())
            .body(self.to_string())
    }
    fn status_code(&self) -> StatusCode {
        match *self {
            APIError::DatabaseError(_) => StatusCode::BAD_REQUEST,
        }
    }
}