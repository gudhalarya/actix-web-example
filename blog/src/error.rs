use actix_web::{HttpResponse, ResponseError, http::StatusCode};
use serde::Serialize;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum AppError {
    #[error("Internal server error")]
    InternalServerError(#[from] anyhow::Error),

    #[error("Not found")]
    NotFound,

    #[error("Unauthorized")]
    Unauthorized,

    #[error("Already exist")]
    AlreadyExist,

    #[error("Does not exist ")]
    NotExist
}

#[derive(Serialize)]
pub struct ErrorBody {
    error: String,
}

impl ResponseError for AppError {
    fn status_code(&self) -> StatusCode {
        match self {
            AppError::InternalServerError(_) => StatusCode::INTERNAL_SERVER_ERROR,
            AppError::NotFound => StatusCode::NOT_FOUND,
            AppError::Unauthorized => StatusCode::UNAUTHORIZED,
            AppError::AlreadyExist=>StatusCode::CONFLICT,
            AppError::NotExist=>StatusCode::NOT_FOUND
        }
    }

    fn error_response(&self) -> HttpResponse {
        if let AppError::InternalServerError(err) = self {
        tracing::error!(
            error = %err,
            debug = ?err,
            "Internal server error"
        );
        }

        HttpResponse::build(self.status_code())
            .json(ErrorBody {
                error: self.to_string(),
            })
    }
}

pub type AppResponse<T> = Result<T, AppError>;