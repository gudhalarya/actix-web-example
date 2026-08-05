use actix_web::{HttpResponse, ResponseError, http::StatusCode};
//This is the errors file that will hold the custom errors we will write 
use thiserror::Error;

#[derive(Debug,Error)]
pub enum AppError {
    #[error( "Internal Server error occurred ")]
    InternalServerError(#[from]anyhow::Error),

    #[error("Not Found")]
    NotFound,

    #[error("Unauthorized")]
    Unauthorized
}

use serde::Serialize;
#[derive(Debug,Serialize)]
pub struct ErrorResponse{
    error:String
}

impl ResponseError for AppError{
    fn status_code(&self) -> StatusCode {
        match self{
            AppError::InternalServerError(_)=>StatusCode::INTERNAL_SERVER_ERROR,
            AppError::Unauthorized=>StatusCode::UNAUTHORIZED,
            AppError::NotFound=>StatusCode::NOT_FOUND
        }
    }

    fn error_response(&self) -> HttpResponse<actix_web::body::BoxBody> {
        if let AppError::InternalServerError(err) =self{
            tracing::error!("Internal error : {:?}",err);
        } 
        HttpResponse::build(self.status_code()).json(ErrorResponse{error:self.to_string(),
        })
    }
}

pub type AppResult<T> = Result<T,AppError>;