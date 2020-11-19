use actix_web::{error, http::StatusCode, HttpResponse};
use serde::{Deserialize, Serialize};
use thiserror::Error;
use tracks_core::tracks;

#[derive(Debug, Error)]
pub enum Error {
    #[error("resource not found")]
    NotFound,

    #[error("{0}")]
    ServiceError(tracks_core::tracks::Error),
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ErrorResponse {
    pub code: u16,
    pub message: String,
    pub error: String,
}

impl Error {
    pub fn name(&self) -> String {
        match self {
            Self::NotFound => String::from("NotFound"),
            Self::ServiceError(err) => match err {
                tracks::Error::NotFound => String::from("NotFound"),
                _ => String::from("InternalServerError"),
            },
        }
    }
}

impl error::ResponseError for Error {
    fn status_code(&self) -> StatusCode {
        match self {
            Self::NotFound => StatusCode::NOT_FOUND,
            Self::ServiceError(err) => match err {
                tracks::Error::NotFound => StatusCode::NOT_FOUND,
                _ => StatusCode::INTERNAL_SERVER_ERROR,
            },
        }
    }

    fn error_response(&self) -> HttpResponse {
        let status_code = self.status_code();
        let error_response = ErrorResponse {
            code: status_code.as_u16(),
            message: self.to_string(),
            error: self.name(),
        };
        HttpResponse::build(status_code).json(error_response)
    }
}
