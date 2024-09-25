use super::{errors::FileUploadError, storage};
use axum::{
    extract::{multipart::Field, Multipart},
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};

pub trait OptionSanitize {
    fn sanitize(&self) -> Option<String>;
}

impl OptionSanitize for Option<&str> {
    fn sanitize(&self) -> Option<String> {
        self.map(|s| s.replace(&['/', '\\', ':', '*', '?', '"', '<', '>', '|'][..], ""))
    }
}

pub async fn file_upload_handler(mut files: Multipart) -> Result<Response, FileUploadError> {
    storage::ensure_folder_exists()
        .await
        .map_err(|err| FileUploadError::CreateFolderError(err.to_string()))?;

    while let Some(file) = files
        .next_field()
        .await
        .map_err(|err| FileUploadError::MultipartError(err.to_string()))?
    {
        assert_file_validity(&file)?;

        let sanitized_name = file
            .file_name()
            .sanitize()
            .ok_or(FileUploadError::InvalidContentType("file_name".to_string()))?;

        let image_data = file
            .bytes()
            .await
            .map_err(|err| FileUploadError::ReadFileError(err.to_string()))?;

        storage::write_to_uploads(&sanitized_name, &image_data)
            .await
            .map_err(|err| FileUploadError::WriteFileError(err.to_string()))?;

        tracing::info!("File uploaded: {}", sanitized_name);
    }

    Ok((
        StatusCode::CREATED,
        Json(serde_json::json!({"msg": "Files uploaded successfully".to_owned()})),
    )
        .into_response())
}

fn assert_file_validity(field: &Field) -> Result<(), FileUploadError> {
    field
        .file_name()
        .sanitize()
        .ok_or(FileUploadError::InvalidContentType("file_name".to_string()))?;

    if field
        .content_type()
        .ok_or(FileUploadError::InvalidContentType(
            "content_type".to_string(),
        ))?
        .starts_with("image/")
    {
        return Ok(());
    }

    Err(FileUploadError::InvalidContentType(
        "content_type".to_string(),
    ))
}
