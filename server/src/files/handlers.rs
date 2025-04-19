use std::str::FromStr;

use super::errors::FileError;
use super::storage::{self, FileName, Sanitize};

use axum::{
    body,
    extract::{multipart::Field, Json, Multipart, Path},
    response::{IntoResponse, Response},
};
use hyper::StatusCode;
use serde::Serialize;

pub const MAX_UPLOAD_SIZE: usize = 50 * 1024 * 1024; // 50 MB

#[derive(Serialize)]
struct ImageMetaData {
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

pub async fn post_image_list(mut images: Multipart) -> Result<Json<ImageList>, FileError> {
    let mut uploaded_images: Vec<FileName> = Vec::new();

    while let Some(file) = images.next_field().await.map_err(FileError::Multipart)? {
        assert_image_validity(&file)?;

        let sanitized_name = file
            .file_name()
            .sanitize()
            .ok_or(FileError::InvalidContentType("file_name"))?;

        uploaded_images.push(sanitized_name.clone());

        let image_data = file.bytes().await.map_err(FileError::Multipart)?;

        storage::write_image(&sanitized_name, &image_data)
            .await
            .map_err(FileError::WriteFile)?;

        tokio::spawn(async move { storage::write_thumbnail(&sanitized_name, &image_data) })
            .await
            .unwrap()
            .map_err(FileError::WriteFile)?;
    }

    Ok(Json(ImageList {
        images: uploaded_images
            .iter()
            .map(|file_name| ImageMetaData {
                name: file_name.sanitized_name.clone(),
                uploaded_at: file_name.uploaded_at,
                image_url: format!("/images/{}", file_name),
                thumbnail_url: format!("/images/{}/thumbnail", file_name),
            })
            .collect(),
    }))
}

pub async fn get_image(Path(name): Path<String>) -> Result<ImageResponse, FileError> {
    Ok(ImageResponse {
        status: StatusCode::OK,
        bytes: storage::get_image(&name)
            .await
            .map_err(FileError::ReadFile)?,
    })
}

pub async fn get_thumbnail(Path(name): Path<String>) -> Result<ImageResponse, FileError> {
    Ok(ImageResponse {
        status: StatusCode::OK,
        bytes: storage::get_thumbnail(&name)
            .await
            .map_err(FileError::ReadFile)?,
    })
}

pub async fn get_all_thumbnails() -> Result<Json<ImageList>, FileError> {
    Ok(Json(ImageList {
        images: storage::get_all_thumbnail_names()
            .await
            .map_err(FileError::ReadFile)?
            .iter()
            .map(|name| ImageMetaData {
                name: name.clone(),
                image_url: format!("/images/{}", name),
                thumbnail_url: format!("/images/{}/thumbnail", name),
            })
            .collect(),
    }))
}

fn assert_image_validity(field: &Field) -> Result<(), FileError> {
    field
        .file_name()
        .sanitize()
        .ok_or(FileError::InvalidContentType("file_name"))?;

    if field
        .content_type()
        .ok_or(FileError::InvalidContentType("content_type"))?
        .starts_with("image/")
    {
        return Ok(());
    }

    Err(FileError::InvalidContentType("content_type"))
}
