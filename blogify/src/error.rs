use actix_web::{HttpResponse, ResponseError, http::StatusCode};
//This is the custom error file we will write now 
use thiserror::Error;
use serde::Serialize;

#[derive(Debug,Error)]
pub enum AppError {
    #[error("Internal error")]
    InternalError(#[from]anyhow::Error),

    #[error("Not found ")]
    NotFound,

    #[error("Bad request")]
    BadRequest
}


#[derive(Debug,Serialize)]
pub struct ErrorBody{
    error:String
}

impl ResponseError for AppError{
    fn status_code(&self) -> StatusCode {
        match self {
            AppError::InternalError(_)=>StatusCode::INTERNAL_SERVER_ERROR,
            AppError::BadRequest=>StatusCode::BAD_REQUEST,
            AppError::NotFound=>StatusCode::NOT_FOUND
        }
    }

    fn error_response(&self) -> HttpResponse<actix_web::body::BoxBody> {
        if let  AppError::InternalError(err)=self{
            tracing::error!("Internal error occured : {:#}",err);
        }
        HttpResponse::build(self.status_code()).json(ErrorBody{
            error:self.to_string(),
        })
    }
}

pub type AppResponse<T>= Result<T,AppError>;