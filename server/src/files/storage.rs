use axum::body;
use std::{io, path::PathBuf};

const UPLOADS_FOLDER: &str = "uploads";

pub async fn create_folder(folder: &PathBuf) -> io::Result<()> {
    if tokio::fs::try_exists(&folder).await? {
        tokio::fs::create_dir_all(&folder).await?;
    }
    Ok(())
}

pub async fn ensure_uploads_folder_exists() -> io::Result<()> {
    let folder = get_uploads_folder()?;
    create_folder(&folder).await
}

pub async fn write_image(file_name: &String, image_data: &body::Bytes) -> io::Result<()> {
    tokio::fs::write(get_uploads_folder()?.join(&file_name), &image_data).await
}

pub async fn read_image(file_name: &String) -> io::Result<body::Bytes> {
    tokio::fs::read(get_uploads_folder()?.join(&file_name))
        .await
        .map(|data| axum::body::Bytes::from(data))
}

pub async fn get_all_image_names() -> io::Result<Vec<String>> {
    let mut entries = tokio::fs::read_dir(get_uploads_folder()?).await?;
    let mut image_names: Vec<String> = Vec::new();
    while let Some(entry) = entries.next_entry().await? {
        image_names.push(entry.file_name().to_string_lossy().to_string());
    }
    Ok(image_names)
}

fn get_uploads_folder() -> io::Result<PathBuf> {
    std::env::current_dir().map(|x| x.join(UPLOADS_FOLDER))
}
