use super::storage::store_file;
use axum::{extract::Multipart, http::StatusCode, response::IntoResponse};

pub async fn upload_handler(mut multipart: Multipart) -> impl IntoResponse {
    while let Some(field) = multipart.next_field().await.unwrap() {
        let file_name = field.file_name().unwrap_or("file").to_string();
        let data = field.bytes().await.unwrap();

        // Store the file
        match store_file(file_name, data).await {
            Ok(_) => println!("File stored successfully"),
            Err(e) => {
                eprintln!("Error storing file: {}", e);
                return (StatusCode::INTERNAL_SERVER_ERROR, "Upload failed");
            }
        }
    }

    (StatusCode::OK, "Files uploaded successfully")
}
