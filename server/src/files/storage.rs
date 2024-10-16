use axum::body;
use core::fmt;
use image::ImageError;
use std::{io, path::PathBuf};

const MAX_THUMBNAILS_SIZE: u32 = 100;

enum ImageType {
    Image,
    Thumbnail,
}

impl fmt::Display for ImageType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ImageType::Image => write!(f, "images"),
            ImageType::Thumbnail => write!(f, "thumbnails"),
        }
    }
}

pub async fn setup() -> io::Result<()> {
    let image_folder = get_folder(ImageType::Image)?;
    let thumbnail_folder = get_folder(ImageType::Thumbnail)?;

    ensure_folder_exists(&image_folder).await?;
    ensure_folder_exists(&thumbnail_folder).await
}

pub async fn write_image(file_name: &String, image_data: &body::Bytes) -> Result<(), ImageError> {
    tokio::fs::write(get_folder(ImageType::Image)?.join(&file_name), &image_data)
        .await
        .map_err(|err| ImageError::IoError(err))
}

pub fn write_thumbnail(file_name: &String, image_data: &body::Bytes) -> Result<(), ImageError> {
    image::load_from_memory(&image_data)?
        .thumbnail(MAX_THUMBNAILS_SIZE, MAX_THUMBNAILS_SIZE)
        .save(get_folder(ImageType::Thumbnail)?.join(&file_name))
}

pub async fn get_all_thumbnail_names() -> io::Result<Vec<String>> {
    let mut entries = tokio::fs::read_dir(get_folder(ImageType::Thumbnail)?).await?;
    let mut image_names: Vec<String> = Vec::new();
    while let Some(entry) = entries.next_entry().await? {
        image_names.push(entry.file_name().to_string_lossy().to_string());
    }
    Ok(image_names)
}

pub async fn get_image(name: &String) -> io::Result<body::Bytes> {
    get_image_type(name, ImageType::Image).await
}

pub async fn get_thumbnail(name: &String) -> io::Result<body::Bytes> {
    get_image_type(name, ImageType::Thumbnail).await
}

async fn get_image_type(name: &String, image_type: ImageType) -> io::Result<body::Bytes> {
    tokio::fs::read(get_folder(image_type)?.join(name))
        .await
        .map(|bytes| body::Bytes::from(bytes))
}

fn get_folder(image_type: ImageType) -> io::Result<PathBuf> {
    std::env::current_dir().map(|x| x.join(image_type.to_string()))
}

async fn ensure_folder_exists(folder: &PathBuf) -> io::Result<()> {
    if !tokio::fs::try_exists(&folder).await? {
        tokio::fs::create_dir_all(&folder).await?;
    }
    Ok(())
}
