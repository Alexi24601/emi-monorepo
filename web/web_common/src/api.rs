use std::collections::HashSet;

use serde::{Deserialize, Serialize};
pub mod auth;
pub mod oauth;
pub mod ws;
use validator::ValidationError;
use validator::ValidationErrors;

use crate::custom_validators::validation_errors::ValidationErrorToString;

pub const ENDPOINT: &str = "/api";
pub const FULL_ENDPOINT: &str = ENDPOINT;

#[derive(Debug, Deserialize, Serialize, Clone)]
pub enum ApiError {
    Unauthorized,
    ExpiredAuthorization,
    BadGateway,
    BadRequest(Vec<String>),
    InternalServerError,
}

impl ApiError {
    pub fn unauthorized() -> Self {
        Self::Unauthorized
    }

    pub fn internal_server_error() -> Self {
        Self::InternalServerError
    }

    pub fn bad_gateway() -> Self {
        Self::BadGateway
    }
}

impl From<serde_json::Error> for ApiError {
    fn from(e: serde_json::Error) -> Self {
        log::error!("Failed to serialize response: {}", e);
        Self::InternalServerError
    }
}

impl From<ValidationErrors> for ApiError {
    fn from(e: ValidationErrors) -> Self {
        log::error!("Validation error: {:?}", e);
        Self::BadRequest(e.convert_to_string())
    }
}

impl From<ValidationError> for ApiError {
    fn from(e: ValidationError) -> Self {
        log::error!("Validation error: {:?}", e);
        Self::BadRequest(e.convert_to_string())
    }
}

impl From<bincode::ErrorKind> for ApiError {
    fn from(e: bincode::ErrorKind) -> Self {
        Self::BadRequest(vec![format!("Serialization failure: {}", e)])
    }
}

impl From<Box<bincode::ErrorKind>> for ApiError {
    fn from(e: Box<bincode::ErrorKind>) -> Self {
        Self::BadRequest(vec![format!("Serialization failure: {}", e)])
    }
}

impl Into<Vec<String>> for ApiError {
    fn into(self) -> Vec<String> {
        match self {
            ApiError::BadRequest(errors) => errors,
            ApiError::Unauthorized => vec!["Unauthorized".to_string()],
            ApiError::ExpiredAuthorization => vec!["Expired Authorization".to_string()],
            ApiError::BadGateway => vec!["Bad Gateway".to_string()],
            ApiError::InternalServerError => vec!["Internal Server Error".to_string()],
        }
    }
}

impl Into<HashSet<String>> for ApiError {
    fn into(self) -> HashSet<String> {
        match self {
            ApiError::BadRequest(errors) => errors.into_iter().collect(),
            ApiError::Unauthorized => vec!["Unauthorized".to_string()].into_iter().collect(),
            ApiError::ExpiredAuthorization => vec!["Expired Authorization".to_string()].into_iter().collect(),
            ApiError::BadGateway => vec!["Bad Gateway".to_string()].into_iter().collect(),
            ApiError::InternalServerError => vec!["Internal Server Error".to_string()].into_iter().collect(),
        }
    }
}