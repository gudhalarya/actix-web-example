#![allow(unused)]//Not recommended just for ignoring the warnings

use actix_web::{HttpResponse, ResponseError, http::StatusCode};
use thiserror::Error;
use serde::Serialize;

#[derive(Debug,Serialize)]
struct ErrorBody{
    error:String
}

#[derive(Debug,Error)]
pub enum AppError {
    #[error("Internal serever faced some error while working")]
    InternalError(#[from]anyhow::Error),

    #[error("Not found")]
    NotFound,

    #[error("Invalid credentials ")]
    Unauthorized,

    #[error("{0}")]
    Conflict(String)
}


impl ResponseError for AppError{
    fn status_code(&self) -> StatusCode {
        match self{
            AppError::InternalError(_)=>StatusCode::INTERNAL_SERVER_ERROR,
            AppError::NotFound=>StatusCode::NOT_FOUND,
            AppError::Unauthorized=>StatusCode::UNAUTHORIZED,
            AppError::Conflict(_)=>StatusCode::CONFLICT
        }
    }

    fn error_response(&self) -> HttpResponse<actix_web::body::BoxBody> {
        if let AppError::InternalError(err)=self{
            tracing::error!(error = ?err,"Internal error");
        } 
        HttpResponse::build(self.status_code()).json(ErrorBody{error:self.to_string()})
    }
}

pub type AppResponse<T> = Result<T,AppError>;