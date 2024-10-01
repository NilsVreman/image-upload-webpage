use super::{errors::FileUploadError, storage};
use axum::extract::{multipart::Field, Json, Multipart};
use serde::Serialize;

pub trait Sanitize {
    fn sanitize(&self) -> Option<String>;
}

impl Sanitize for Option<&str> {
    fn sanitize(&self) -> Option<String> {
        self.map(|s| s.replace(&['/', '\\', ':', '*', '?', '"', '<', '>', '|'][..], ""))
    }
}

#[derive(Serialize)]
pub struct FileUploadResponse {
    msg: String,
    name: String,
}

pub async fn image_upload_handler(
    mut files: Multipart,
) -> Result<Json<FileUploadResponse>, FileUploadError> {
    storage::ensure_uploads_folder_exists()
        .await
        .map_err(|err| FileUploadError::CreateFolderError(err.to_string()))?;

    let mut sanitized_name = String::new();

    while let Some(file) = files
        .next_field()
        .await
        .map_err(|err| FileUploadError::MultipartError(err.to_string()))?
    {
        assert_image_validity(&file)?;

        sanitized_name = file
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

        dbg!("File uploaded: {}", &sanitized_name);
    }

    Ok(Json(FileUploadResponse {
        msg: "Files uploaded successfully".to_string(),
        name: sanitized_name,
    }))
}

fn assert_image_validity(field: &Field) -> Result<(), FileUploadError> {
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
