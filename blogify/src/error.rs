use actix_web::{HttpResponse, ResponseError, http::StatusCode};
use serde::Serialize;
//This is for the custom errors
use thiserror::Error;
#[derive(Debug,Error)]
pub enum AppError {
    #[error("Internal server error ")]
    InternalError(#[from]anyhow::Error),

    #[error("Not found ")]
    NotFound,

    #[error("Inavalid input")]
    BadRequest,
}

#[derive(Serialize)]
struct ErrorBody{
    error:String
}

impl ResponseError for AppError{
    fn status_code(&self) -> StatusCode {
        match self {
            AppError::InternalError(_)=>StatusCode::INTERNAL_SERVER_ERROR,
            AppError::NotFound=>StatusCode::NOT_FOUND,
            AppError::BadRequest=>StatusCode::BAD_REQUEST
        }
    }

    fn error_response(&self) -> HttpResponse<actix_web::body::BoxBody> {
        if let AppError::InternalError(err)=self{
            tracing::error!(system_crash = ?err,"An unhandled errror occured");
        }
        HttpResponse::build(self.status_code()).json(ErrorBody{error:self.to_string(),
        })
    }
}

pub type AppResponse<T> = Result<T,AppError>;