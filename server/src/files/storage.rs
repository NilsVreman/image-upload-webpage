use axum::body;
use core::fmt;
use std::{io, path::PathBuf};

pub enum ImageType {
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

pub fn get_folder(image_type: ImageType) -> io::Result<PathBuf> {
    std::env::current_dir().map(|x| x.join(image_type.to_string()))
}

pub async fn ensure_folder_exists(folder: &PathBuf) -> io::Result<()> {
    if !tokio::fs::try_exists(&folder).await? {
        tokio::fs::create_dir_all(&folder).await?;
    }
    Ok(())
}

pub async fn write_image(file_name: &String, image_data: &body::Bytes) -> io::Result<()> {
    tokio::fs::write(get_folder(ImageType::Image)?.join(&file_name), &image_data).await
}

pub async fn write_thumbnail(file_name: &String, image_data: &body::Bytes) -> io::Result<()> {
    tokio::fs::write(
        get_folder(ImageType::Thumbnail)?.join(&file_name),
        &image_data,
    )
    .await
}

pub async fn read_image(file_name: &String) -> io::Result<body::Bytes> {
    tokio::fs::read(get_folder(ImageType::Image)?.join(&file_name))
        .await
        .map(|data| axum::body::Bytes::from(data))
}

pub async fn get_all_image_names() -> io::Result<Vec<String>> {
    let mut entries = tokio::fs::read_dir(get_folder(ImageType::Image)?).await?;
    let mut image_names: Vec<String> = Vec::new();
    while let Some(entry) = entries.next_entry().await? {
        image_names.push(entry.file_name().to_string_lossy().to_string());
    }
    Ok(image_names)
}
