use super::{errors::FileUploadError, storage};
use axum::{
    body,
    extract::{multipart::Field, Json, Multipart, Path},
    http::StatusCode,
};
use serde::Serialize;

pub trait Sanitize {
    fn sanitize(&self) -> Option<String>;
}

impl Sanitize for Option<&str> {
    fn sanitize(&self) -> Option<String> {
        self.map(|s| s.replace(&['/', '\\', ':', '*', '?', '"', '<', '>', '|'][..], ""))
    }
}
impl Sanitize for String {
    fn sanitize(&self) -> Option<String> {
        Some(self.replace(&['/', '\\', ':', '*', '?', '"', '<', '>', '|'][..], ""))
    }
}

#[derive(Serialize)]
pub struct FileUploadResponse {
    msg: String,
}

#[derive(Serialize)]
pub struct FileList {
    files: Vec<String>,
}

pub async fn post_image_list(
    mut images: Multipart,
) -> Result<Json<FileUploadResponse>, FileUploadError> {
    storage::ensure_uploads_folder_exists()
        .await
        .map_err(|err| FileUploadError::CreateFolderError(err.to_string()))?;

    while let Some(file) = images
        .next_field()
        .await
        .map_err(|err| FileUploadError::MultipartError(err.to_string()))?
    {
        assert_image_validity(&file)?;

        let sanitized_name = file
            .file_name()
            .sanitize()
            .ok_or(FileUploadError::InvalidContentType("file_name".to_string()))?;

        let image_data = file
            .bytes()
            .await
            .map_err(|err| FileUploadError::ReadFileError(err.to_string()))?;

        storage::write_image(&sanitized_name, &image_data)
            .await
            .map_err(|err| FileUploadError::WriteFileError(err.to_string()))?;

        dbg!("File uploaded: {}", &sanitized_name);
    }

    Ok(Json(FileUploadResponse {
        msg: "Files uploaded successfully".to_string(),
    }))
}

pub async fn get_image(
    Path(file_name): Path<String>,
) -> Result<(StatusCode, body::Bytes), FileUploadError> {
    let image = storage::read_image(&file_name.sanitize().expect("Invalid file name"))
        .await
        .map_err(|err| FileUploadError::ReadFileError(err.to_string()))?;

    Ok((StatusCode::OK, image))
}

pub async fn get_image_list() -> Result<Json<FileList>, FileUploadError> {
    Ok(Json(FileList {
        files: storage::get_all_image_names()
            .await
            .map_err(|err| FileUploadError::ReadFileError(err.to_string()))?,
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
