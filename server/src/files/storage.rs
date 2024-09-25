use hyper::body;
use std::{io, path::PathBuf};

const UPLOADS_FOLDER: &str = "uploads";

fn get_uploads_folder() -> io::Result<PathBuf> {
    std::env::current_dir().map(|x| x.join(UPLOADS_FOLDER))
}

async fn create_folder(folder: &PathBuf) -> io::Result<()> {
    if tokio::fs::try_exists(&folder).await? {
        tokio::fs::create_dir_all(&folder).await?;
    }
    Ok(())
}

pub async fn ensure_folder_exists() -> io::Result<()> {
    let folder = get_uploads_folder()?;
    create_folder(&folder).await
}

pub async fn write_to_uploads(file_name: &String, image_data: &body::Bytes) -> io::Result<()> {
    tokio::fs::write(get_uploads_folder()?.join(&file_name), &image_data).await
}
