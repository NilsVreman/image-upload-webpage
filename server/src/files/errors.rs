use std::io;

use axum::extract::multipart::MultipartError;
use axum::http::StatusCode;
use axum::{response, Json};
use image::ImageError;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum FileError {
    #[error("Failed to read multipart field: {0}")]
    Multipart(#[from] MultipartError),

    #[error("Invalid content type: {0}")]
    InvalidContentType(&'static str),

    #[error("Failed to write file to disk: {0}")]
    WriteFile(#[from] ImageError),

    #[error("Failed to read file data: {0}")]
    ReadFile(#[from] io::Error),

    #[error("Failed to join: {0}")]
    Join(#[from] tokio::task::JoinError),
}

#[derive(Error, Debug)]
pub enum ParseError {
    #[error("failed to parse RFC‑3339 date: {0}")]
    Date(#[from] chrono::ParseError), // keep the real chrono error

    #[error("failed to parse UUID: {0}")]
    Uuid(#[from] uuid::Error), // keep the real uuid error

    #[error("bad file‑name layout: {0}")]
    FileName(&'static str), // just a static msg is enough here
}

impl response::IntoResponse for FileError {
    fn into_response(self) -> response::Response {
        let code = match &self {
            Self::Multipart(_) | Self::InvalidContentType(_) => StatusCode::BAD_REQUEST,
            Self::ReadFile(_) | Self::WriteFile(_) | Self::Join(_) => {
                StatusCode::INTERNAL_SERVER_ERROR
            }
        };

        (code, Json(serde_json::json!({"error": self.to_string()}))).into_response()
    }
}
