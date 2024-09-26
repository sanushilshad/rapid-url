use actix_http::StatusCode;
use actix_web::{HttpResponse, ResponseError};

use crate::{schemas::GenericResponse, utils::error_chain_fmt};

#[derive(thiserror::Error)]
pub enum CustomJWTTokenError {
    #[error("Token expired")]
    Expired,
    #[error("{0}")]
    Invalid(String),
}

impl std::fmt::Debug for CustomJWTTokenError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        error_chain_fmt(self, f)
    }
}

#[derive(thiserror::Error)]
pub enum GenericError {
    #[error("{0}")]
    ValidationError(String),
    #[error(transparent)]
    UnexpectedError(#[from] anyhow::Error),
    #[error("{0}")]
    InvalidJWT(String),

}

impl std::fmt::Debug for GenericError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        error_chain_fmt(self, f)
    }
}

impl ResponseError for GenericError {
    fn status_code(&self) -> StatusCode {
        match self {
            GenericError::ValidationError(_) => StatusCode::BAD_REQUEST,
            GenericError::UnexpectedError(_) => StatusCode::INTERNAL_SERVER_ERROR,

            GenericError::InvalidJWT(_) => StatusCode::UNAUTHORIZED,
        }
    }

    fn error_response(&self) -> HttpResponse {
        let status_code = self.status_code();
        let status_code_str = status_code.as_str();
        let inner_error_msg = match self {
            GenericError::ValidationError(message) => message.to_string(),
            GenericError::UnexpectedError(error_msg) => error_msg.to_string(),
            GenericError::InvalidJWT(error_msg) => error_msg.to_string(),
        };

        HttpResponse::build(status_code).json(GenericResponse::error(
            &inner_error_msg,
            status_code_str,
            Some(()),
        ))
    }
}