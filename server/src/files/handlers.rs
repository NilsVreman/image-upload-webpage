use super::storage;

use axum::extract::multipart::Field;
use axum::{
    extract::Multipart,
    http::StatusCode,
    response::{IntoResponse, Response},
};

pub struct FileUploadError(String);

impl IntoResponse for FileUploadError {
    fn into_response(self) -> Response {
        (StatusCode::INTERNAL_SERVER_ERROR, self.0).into_response()
    }
}

pub trait OptionSanitize {
    fn sanitize(&self) -> Option<String>;
}

impl OptionSanitize for Option<&str> {
    fn sanitize(&self) -> Option<String> {
        self.map(|s| s.replace(&['/', '\\', ':', '*', '?', '"', '<', '>', '|'][..], ""))
    }
}

pub async fn file_upload_handler(mut files: Multipart) -> Result<Response, FileUploadError> {
    storage::create_uploads_folder()
        .await
        .map_err(|err| FileUploadError(format!("Failed to create uploads folder: {}", err)))?;

    while let Some(file) = files
        .next_field()
        .await
        .map_err(|_| FileUploadError("Failed to read field".to_string()))?
    {
        assert_valid_file(&file)?;

        let sanitized_name = file
            .file_name()
            .sanitize()
            .ok_or(FileUploadError("Missing Filename".to_string()))?;

        let image_data = file
            .bytes()
            .await
            .map_err(|err| FileUploadError(format!("Failed to read file data: {}", err)))?;

        storage::write_to_uploads(sanitized_name, image_data)
            .await
            .map_err(|err| FileUploadError(format!("Failed to write file to disk: {}", err)))?;
    }

    Ok((
        StatusCode::CREATED,
        "Files uploaded successfully".to_owned(),
    )
        .into_response())
}

fn assert_valid_file(field: &Field) -> Result<(), FileUploadError> {
    field
        .file_name()
        .sanitize()
        .ok_or(FileUploadError("Missing Filename".to_string()))?;

    if !field
        .content_type()
        .ok_or(FileUploadError("Missing Content Type".to_string()))?
        .starts_with("image/")
    {
        return Err(FileUploadError("Invalid Content Type".to_string()));
    }

    Ok(())
}
