// Thanks to https://mattgathu.github.io/2020/04/16/actix-web-error-handling.html

use actix_web::{error::ResponseError, get, http::StatusCode, web, HttpResponse};
use serde::Serialize;
use std::io::Read;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum CustomError {
    #[error("Requested file was not found")]
    NotFound,
    #[error("You are forbidden to access requested file.")]
    Forbidden,
    #[error("Unknown Internal Error")]
    Unknown,
}

impl CustomError {
    pub fn name(&self) -> String {
        match self {
            Self::NotFound => "NotFound".to_string(),
            Self::Forbidden => "Forbidden".to_string(),
            Self::Unknown => "Unknown".to_string(),
        }
    }
}
impl ResponseError for CustomError {
    fn status_code(&self) -> StatusCode {
        match *self {
            Self::NotFound => StatusCode::NOT_FOUND,
            Self::Forbidden => StatusCode::FORBIDDEN,
            Self::Unknown => StatusCode::INTERNAL_SERVER_ERROR,
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

fn map_io_error(e: std::io::Error) -> CustomError {
    match e.kind() {
        std::io::ErrorKind::NotFound => CustomError::NotFound,
        std::io::ErrorKind::PermissionDenied => CustomError::Forbidden,
        _ => CustomError::Unknown,
    }
}

#[derive(Serialize)]
struct ErrorResponse {
    code: u16,
    error: String,
    message: String,
}

#[get("/file/{file_name}")]
async fn get_file(file_name: web::Path<String>) -> Result<HttpResponse, CustomError> {
    let mut s = String::new();
    std::fs::File::open(file_name.to_string())
        .map_err(map_io_error)?
        .read_to_string(&mut s)
        .map_err(map_io_error)?;
    Ok(HttpResponse::Ok().body(s))
}

pub async fn not_found() -> Result<HttpResponse, CustomError> {
    Ok(HttpResponse::build(StatusCode::OK)
        .content_type("text/html; charset=utf-8")
        .body("<h1>Page not found :(</h1>"))
}
