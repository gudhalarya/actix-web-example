//This is for the custom errors we are making 

use actix_web::{HttpResponse, ResponseError, http::StatusCode};
use thiserror::Error;
use serde::Serialize;


#[derive(Debug,Error)]
pub enum AppError{
    #[error("Database error")]
    DatabaseError(#[from]sqlx::Error),

    #[error("Not found")]
    NotFound
}

#[derive(Serialize)]
struct ErrorBody{
    error:String
}

impl ResponseError for AppError{
    fn status_code(&self) -> actix_web::http::StatusCode {
        match self{
            AppError::DatabaseError(_)=>StatusCode::INTERNAL_SERVER_ERROR,
            AppError::NotFound=>StatusCode::NOT_FOUND
        }
    }


    fn error_response(&self) -> actix_web::HttpResponse<actix_web::body::BoxBody> {
        if matches!(self,AppError::DatabaseError(_)){
            tracing::error!("Internal database error ");
        }
        HttpResponse::build(self.status_code()).json(ErrorBody{error:self.to_string(),
        })
    }
}

pub type AppResponse<T>= Result<T,AppError>;