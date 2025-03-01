use crate::app;

use super::{
    errors::FileUploadError,
    storage::{self, Sanitize},
};

use axum::{
    body,
    extract::{multipart::Field, Json, Multipart, Path},
    response::{IntoResponse, Response},
    Extension,
};
use hyper::StatusCode;
use serde::Serialize;

pub const MAX_UPLOAD_SIZE: usize = 50 * 1024 * 1024; // 50 MB

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

pub struct ImageResponse {
    status: StatusCode,
    bytes: body::Bytes,
}

impl IntoResponse for ImageResponse {
    fn into_response(self) -> Response {
        (self.status, self.bytes).into_response()
    }
}

pub async fn post_image_list(
    mut images: Multipart,
) -> Result<Json<serde_json::Value>, FileUploadError> {
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

        tokio::spawn(async move { storage::write_thumbnail(&sanitized_name, &image_data) })
            .await
            .unwrap()
            .map_err(|err| FileUploadError::WriteFileError(err.to_string()))?;
    }

    Ok(Json(serde_json::json!(
        { "msg": "Files uploaded successfully".to_string() }
    )))
}

pub async fn get_image(Path(name): Path<String>) -> Result<ImageResponse, FileUploadError> {
    Ok(ImageResponse {
        status: StatusCode::OK,
        bytes: storage::get_image(&name)
            .await
            .map_err(|err| FileUploadError::ReadFileError(err.to_string()))?,
    })
}

pub async fn get_thumbnail(Path(name): Path<String>) -> Result<ImageResponse, FileUploadError> {
    Ok(ImageResponse {
        status: StatusCode::OK,
        bytes: storage::get_thumbnail(&name)
            .await
            .map_err(|err| FileUploadError::ReadFileError(err.to_string()))?,
    })
}

pub async fn get_all_thumbnails(
    Extension(general_config): Extension<app::GeneralConfig>,
) -> Result<Json<ImageList>, FileUploadError> {
    Ok(Json(ImageList {
        images: storage::get_all_thumbnail_names()
            .await
            .map_err(|err| FileUploadError::ReadFileError(err.to_string()))?
            .iter()
            .map(|name| ImageMetaData {
                name: name.clone(),
                image_url: format!(
                    "{}:{}/images/{}",
                    general_config.host, general_config.port, name
                ),
                thumbnail_url: format!(
                    "{}:{}/images/{}/thumbnail",
                    general_config.host, general_config.port, name
                ),
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
