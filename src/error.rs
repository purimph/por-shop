use actix_web::{error, http::StatusCode, HttpResponse};
use derive_more::{derive::From, Display};
// use jsonwebtoken::Validation;
use sea_orm::DbErr;
use serde::Serialize;
use validator::ValidationErrors;

use crate::services::error::ServiceError;

// use crate::services::error::ServiceError;

#[derive(Debug, Display, From)]

pub enum ApiError {
    DatabaseError(String),
    #[from]
    ValidationError(ValidationErrors),

    NotFound(String),

    AuthenticationError(String),
    // #[display("Internal server error:{}", _0)]
    // InternalServerError(String),
}

impl From<DbErr> for ApiError {
    fn from(err: DbErr) -> Self {
        match err {
            DbErr::RecordNotFound(msg) => ApiError::NotFound(msg),
            _ => ApiError::DatabaseError(err.to_string()),
        }
    }
}

impl From<ServiceError> for ApiError {
    fn from(err: ServiceError) -> Self {
        match err {
            ServiceError::Database(e) => ApiError::DatabaseError(e.to_string()),
            _ => ApiError::NotFound("".into()),
        }
    }
}

// impl From<ValidationErrors> for ApiError {
//     fn from(err: ValidationErrors) -> Self {
//         ApiError::ValidationError(err.to_string())
//     }
// }

impl error::ResponseError for ApiError {
    fn error_response(&self) -> HttpResponse {
        let error_response = match self {
            ApiError::DatabaseError(msg) => ErrorResponse {
                error: "Database error".to_string(),
                message: msg.clone(),
            },

            ApiError::ValidationError(msg) => ErrorResponse {
                error: "Validation error".to_string(),
                message: msg.to_string(),
            },

            ApiError::NotFound(msg) => ErrorResponse {
                error: "Not found".to_string(),
                message: msg.clone(),
            },

            ApiError::AuthenticationError(msg) => ErrorResponse {
                error: "Authentication error".to_string(),
                message: msg.clone(),
            },
            // ApiError::InternalServerError(_) => ErrorResponse {
            //     error: "Internal server error".to_string(),
            //     message: "An unexpected error occurred".to_string(),
            // },
        };

        HttpResponse::build(self.status_code()).json(error_response)
    }

    fn status_code(&self) -> StatusCode {
        match self {
            ApiError::DatabaseError(_) => StatusCode::BAD_REQUEST,
            ApiError::ValidationError(_) => StatusCode::BAD_REQUEST,
            ApiError::NotFound(_) => StatusCode::NOT_FOUND,
            ApiError::AuthenticationError(_) => StatusCode::UNAUTHORIZED,
            // ApiError::InternalServerError(_) => StatusCode::INTERNAL_SERVER_ERROR
        }
    }
}

#[derive(Serialize)]
struct ErrorResponse {
    error: String,
    message: String,
}

// use actix_web::{
//     error,
//     http::{header::ContentType, StatusCode},
//     HttpResponse
// };
// use derive_more::derive::{Display, From};
// use sea_orm::DbErr;

// #[derive(Debug, Display, From)]
// pub enum APIError {
//     #[display("Error on Internal server error")]
//     DatabaseError(DbErr),
// }

// impl error::ResponseError for APIError {
//     fn error_response(&self) -> HttpResponse {
//         HttpResponse::build(self.status_code())
//             .insert_header(ContentType::html())
//             .body(self.to_string())
//     }
//     fn status_code(&self) -> StatusCode {
//         match *self {
//             APIError::DatabaseError(_) => StatusCode::BAD_REQUEST,
//         }
//     }
// }
