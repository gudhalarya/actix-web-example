//This is where the custom errors will go --------------> 

use actix_web::{HttpResponse, ResponseError, http::StatusCode};
use thiserror::Error;

#[derive(Debug,Error)]
pub enum AppError {
    #[error("Internal server error")]
    InternalServerError(#[from]anyhow::Error),

    #[error("Not found in the scope")]
    NotFound,

    #[error("Invalid credentials")]
    BadRequest,

    #[error("Unauthorized")]
    Unauthorized
}

use serde::Serialize;
#[derive(Debug,Serialize)]
pub struct ErrorBody{
    error:String
}

impl ResponseError for AppError{
    fn status_code(&self) -> StatusCode {
        match self{
            AppError::InternalServerError(_)=>StatusCode::INTERNAL_SERVER_ERROR,
            AppError::BadRequest=>StatusCode::BAD_REQUEST,
            AppError::Unauthorized=>StatusCode::UNAUTHORIZED,
            AppError::NotFound=>StatusCode::NOT_FOUND
        }
    }

    fn error_response(&self) -> HttpResponse<actix_web::body::BoxBody> {
        if let AppError::InternalServerError(err) = self {
            tracing::error!(err = ?err,"Internal error");
        }
        HttpResponse::build(self.status_code()).json(ErrorBody{error:self.to_string()})
    }
}