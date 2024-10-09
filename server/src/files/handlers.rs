use super::{errors::FileUploadError, storage};

use axum::{
    body,
    extract::{multipart::Field, Json, Multipart, Path},
    http::StatusCode,
};
use serde::Serialize;

pub const MAX_UPLOAD_SIZE: usize = 50 * 1024 * 1024; // 50 MB

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
        Some(&self[..]).sanitize()
    }
}

#[derive(Serialize)]
pub struct ImageMetaData {
    name: String,
    image_url: String,
    thumbnail_url: String,
}

#[derive(Serialize)]
pub struct ImageList {
    images: Vec<ImageMetaData>,
}

pub async fn post_image_list(
    mut images: Multipart,
) -> Result<Json<serde_json::Value>, FileUploadError> {
    storage::setup()
        .await
        .map_err(|err| FileUploadError::StorageError(err.to_string()))?;

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

        let x = sanitized_name.clone();

        tokio::spawn(async move {
            storage::write_thumbnail(&sanitized_name, &image_data)
                .map_err(|err| FileUploadError::WriteFileError(err.to_string()))
        });

        dbg!("Uploaded file: {}", x);
    }

    Ok(Json(serde_json::json!(
        "Files uploaded successfully".to_string()
    )))
}

pub async fn get_image(
    Path(name): Path<String>,
) -> Result<(StatusCode, body::Bytes), FileUploadError> {
    Ok((
        StatusCode::OK,
        storage::get_image(&name)
            .await
            .map_err(|err| FileUploadError::ReadFileError(err.to_string()))?,
    ))
}

pub async fn get_thumbnail(
    Path(name): Path<String>,
) -> Result<(StatusCode, body::Bytes), FileUploadError> {
    Ok((
        StatusCode::OK,
        storage::get_thumbnail(&name)
            .await
            .map_err(|err| FileUploadError::ReadFileError(err.to_string()))?,
    ))
}

pub async fn get_all_thumbnail_meta_data() -> Result<Json<ImageList>, FileUploadError> {
    Ok(Json(ImageList {
        images: storage::get_all_thumbnail_names()
            .await
            .map_err(|err| FileUploadError::ReadFileError(err.to_string()))?
            .iter()
            .map(|name| ImageMetaData {
                name: name.clone(),
                image_url: format!("/api/images/{}", name), // NOTE: these have to match routes
                thumbnail_url: format!("/api/images/{}/thumbnail", name),
            })
            .collect(),
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
