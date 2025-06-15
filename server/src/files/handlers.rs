use std::borrow::Borrow;
use std::str::FromStr;

use super::errors::FileError;
use super::storage::{self, FileName, Sanitize};

use axum::{
    body,
    extract::{multipart::Field, Json, Multipart, Path},
    response::{IntoResponse, Response},
};
use chrono::{DateTime, Utc};
use hyper::StatusCode;
use serde::Serialize;

pub const MAX_UPLOAD_SIZE: usize = 100 * 1024 * 1024; // 100 MB

#[derive(Serialize)]
struct ImageMetaData {
    name: String,
    uploaded_at: DateTime<Utc>,
    image_url: String,
    thumbnail_url: String,
}

impl<T> From<T> for ImageMetaData
where
    T: Borrow<FileName>,
{
    fn from(value: T) -> Self {
        let file_name: &FileName = value.borrow();

        ImageMetaData {
            name: file_name.to_string(),
            uploaded_at: file_name.uploaded_at,
            image_url: format!("/images/{}", file_name),
            thumbnail_url: format!("/images/{}/thumbnail", file_name),
        }
    }
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
    let mut upload_tasks = Vec::new();

    while let Some(file) = images.next_field().await.map_err(FileError::Multipart)? {
        assert_image_validity(&file)?;

        let sanitized_name = file
            .file_name()
            .sanitize()
            .ok_or(FileError::InvalidContentType("file_name"))?;

        uploaded_images.push(sanitized_name.clone());

        let image_data = file.bytes().await.map_err(FileError::Multipart)?;

        upload_tasks.push(tokio::spawn({
            let name = sanitized_name.clone();
            let bytes = image_data.clone();
            async move {
                storage::write_image(&name, &bytes)
                    .await
                    .map_err(FileError::WriteFile)
            }
        }));
        upload_tasks.push(tokio::task::spawn_blocking({
            let name = sanitized_name.clone();
            let bytes = image_data.clone();
            move || storage::write_thumbnail(&name, &bytes).map_err(FileError::WriteFile)
        }));
    }

    // Await all upload tasks
    for handle in upload_tasks {
        handle.await??
    }

    Ok(Json(ImageList {
        images: uploaded_images.iter().map(ImageMetaData::from).collect(),
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
            .map(|name| {
                FileName::from_str(name)
                    .map(ImageMetaData::from)
                    .expect("could not get data from file_name")
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
