use super::{errors::FileUploadError, storage};

use axum::{
    body,
    extract::{multipart::Field, Json, Multipart, Path},
    http::StatusCode,
};
use image::ImageFormat;
use serde::Serialize;
use std::io::Cursor;

const MAX_THUMBNAILS_SIZE: u32 = 150;

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
    setup_folders().await?;

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

        storage::write_image(&sanitized_name, &image_data, storage::ImageType::Image)
            .await
            .map_err(|err| FileUploadError::WriteFileError(err.to_string()))?;

        let mut thumbnail = Cursor::new(Vec::new());
        let _ = image::load_from_memory(&image_data)
            .map_err(|err| FileUploadError::ProcessingError(err.to_string()))?
            .thumbnail(MAX_THUMBNAILS_SIZE, MAX_THUMBNAILS_SIZE)
            .write_to(&mut thumbnail, ImageFormat::Jpeg);

        storage::write_image(
            &sanitized_name,
            &body::Bytes::from(thumbnail.into_inner()),
            storage::ImageType::Thumbnail,
        )
        .await
        .map_err(|err| FileUploadError::WriteFileError(err.to_string()))?;

        dbg!("File and thumbnail uploaded: {}", &sanitized_name);
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
        read_image_from_storage(&name, storage::ImageType::Image).await?,
    ))
}

pub async fn get_thumbnail(
    Path(name): Path<String>,
) -> Result<(StatusCode, body::Bytes), FileUploadError> {
    Ok((
        StatusCode::OK,
        read_image_from_storage(&name, storage::ImageType::Thumbnail).await?,
    ))
}

async fn read_image_from_storage(
    name: &str,
    image_type: storage::ImageType,
) -> Result<body::Bytes, FileUploadError> {
    storage::read_image(&name.to_string(), image_type)
        .await
        .map_err(|err| FileUploadError::ReadFileError(err.to_string()))
}

pub async fn get_image_list() -> Result<Json<ImageList>, FileUploadError> {
    Ok(Json(ImageList {
        images: storage::get_all_image_names()
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

async fn setup_folders() -> Result<(), FileUploadError> {
    let image_folder = storage::get_folder(storage::ImageType::Image)
        .map_err(|err| FileUploadError::CreateFolderError(err.to_string()))?;
    let thumbnail_folder = storage::get_folder(storage::ImageType::Thumbnail)
        .map_err(|err| FileUploadError::CreateFolderError(err.to_string()))?;

    storage::ensure_folder_exists(&image_folder)
        .await
        .map_err(|err| FileUploadError::CreateFolderError(err.to_string()))?;
    storage::ensure_folder_exists(&thumbnail_folder)
        .await
        .map_err(|err| FileUploadError::CreateFolderError(err.to_string()))
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
