use actix_web::{ResponseError, http::StatusCode};
//This is the error.rs
use thiserror::Error;
#[derive(Debug,Error)]
pub enum AppError {
    #[error("Internal server error")]
    InternalError(#[from]anyhow::Error),

    #[error("Not found")]
    NotFound,

    #[error("Unauthorized")]
    Unauthorized
}

use serde::Serialize;
pub struct ErrorBody{
    error:String
}

impl ResponseError for AppError{
    fn status_code(&self) -> actix_web::http::StatusCode {
        match self{
            AppError::InternalError(_)=>StatusCode::INTERNAL_SERVER_ERROR,
            AppError::NotFound=>StatusCode::NOT_FOUND,
            AppError::Unauthorized=>StatusCode::UNAUTHORIZED
            }
    }
}