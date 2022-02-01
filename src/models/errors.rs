use serde::Serialize;
use actix_web::{ResponseError, HttpResponse, http::StatusCode};
use std::fmt::{self};

#[derive(Debug)]
pub enum CustomErrorType {
    DatabaseError,
    NotFoundError
}

#[derive(Debug)]
pub struct CustomError {
    pub message: Option<String>,
    pub cause: Option<String>,
    pub error_type: CustomErrorType,
}

impl CustomError {
    fn message(&self) -> String {
        match &*self {
            CustomError {
                message: Some(message),
                cause: _,
                error_type: _ } => message.clone(),
            CustomError {
                message: None,
                cause: _,
                error_type: CustomErrorType::NotFoundError } => "The requested item was not found".to_string(),
            _ => "An unexpected error has occurred".to_string()
        }
    }
}

impl fmt::Display for CustomError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(Serialize)]
pub struct CustomErrorResponse {
    pub error: String,
}

impl ResponseError for CustomError {

    fn status_code(&self) -> StatusCode {
        match self.error_type {
            CustomErrorType::DatabaseError => StatusCode::INTERNAL_SERVER_ERROR,
            CustomErrorType::NotFoundError => StatusCode::NOT_FOUND,
        }
    }

    fn error_response(&self) -> HttpResponse {
        HttpResponse::build(self.status_code())
            .json(CustomErrorResponse {error: self.message()})
    }
}