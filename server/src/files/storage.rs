use std::io;
use std::path::PathBuf;

use bytes::Bytes;

const UPLOADS_FOLDER: &str = "uploads";

fn get_uploads_folder() -> io::Result<PathBuf> {
    std::env::current_dir().map(|x| x.join(UPLOADS_FOLDER))
}

async fn create_folder(folder: PathBuf) -> io::Result<()> {
    if !folder.exists() {
        tokio::fs::create_dir_all(&folder).await
    } else {
        Err(io::Error::new(
            io::ErrorKind::AlreadyExists,
            "Uploads folder already exists",
        ))
    }
}

pub async fn create_uploads_folder() -> io::Result<()> {
    match get_uploads_folder() {
        Ok(uploads) => create_folder(uploads).await,
        Err(_) => Err(io::Error::new(
            io::ErrorKind::NotFound,
            "Couldn't resolve uploads folder",
        )),
    }
}

pub async fn write_to_uploads(file_name: String, image_data: Bytes) -> io::Result<()> {
    tokio::fs::write(get_uploads_folder()?.join(&file_name), &image_data).await
}
