use actix_web::{HttpResponse, ResponseError, http::StatusCode};
//This is the file for the custom error 
//---------------ONE OF THE MOST IMPORTANT FILE IS HERE ---------------
use thiserror::Error;

#[derive(Debug,Error)]
pub enum AppError {
    #[error("Internal services failed unexpectedly")]
    InternalServerError(#[from]anyhow::Error),

    #[error("Not found")]
    NotFound,

    #[error("Unauthorized")]
    Unauthroized,

    #[error("Bad Request")]
    BadRequest
}

use serde::Serialize;
#[derive(Serialize)]
pub struct ErrorBody{
    error:String
}

impl ResponseError for AppError{
    fn status_code(&self) -> StatusCode {
        match self{
            AppError::InternalServerError(_)=>StatusCode::INTERNAL_SERVER_ERROR,
            AppError::Unauthroized=>StatusCode::UNAUTHORIZED,
            AppError::BadRequest=>StatusCode::BAD_REQUEST,
            AppError::NotFound=>StatusCode::NOT_FOUND
        }
    }

    fn error_response(&self) -> HttpResponse<actix_web::body::BoxBody> {
        if let AppError::InternalServerError(err)=self{
            tracing::error!(error = ?err,"Internal error");
        }
        HttpResponse::build(self.status_code()).json(ErrorBody{error:self.to_string()})
    }
}


pub type AppResponse<T> = Result<T,AppError>;