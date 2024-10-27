use axum::{http::StatusCode, response, Json};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum FileUploadError {
    #[error("Failed to read multipart field: {0}")]
    MultipartError(String),

    #[error("Invalid content type: {0}")]
    InvalidContentType(String),

    #[error("Failed to read file data: {0}")]
    ReadFileError(String),

    #[error("Failed to write file to disk: {0}")]
    WriteFileError(String),
}

impl response::IntoResponse for FileUploadError {
    fn into_response(self) -> response::Response {
        let code = match &self {
            FileUploadError::MultipartError(_) | FileUploadError::InvalidContentType(_) => {
                StatusCode::BAD_REQUEST
            }
            FileUploadError::ReadFileError(_) | FileUploadError::WriteFileError(_) => {
                StatusCode::INTERNAL_SERVER_ERROR
            }
        };

        (code, Json(serde_json::json!({"error": self.to_string()}))).into_response()
    }
}
