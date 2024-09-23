use bytes::Bytes;
use std::fs;
use std::io::Write;
use std::path::PathBuf;
use uuid::Uuid;

pub async fn store_file(file_name: String, data: Bytes) -> Result<(), String> {
    // Resolve the upload directory
    let mut upload_dir = std::env::current_dir().unwrap();
    upload_dir.push("uploads");

    // Create the directory if it doesn't exist
    if !upload_dir.exists() {
        fs::create_dir_all(&upload_dir)
            .map_err(|e| format!("Failed to create upload directory: {}", e))?;
    }

    // Generate a unique file name
    let unique_file_name = format!("{}_{}", Uuid::new_v4(), file_name);

    // Full path to the file
    let mut file_path = PathBuf::from(&upload_dir);
    file_path.push(unique_file_name);

    // Write the file
    let mut file =
        fs::File::create(&file_path).map_err(|e| format!("Failed to create file: {}", e))?;
    file.write_all(&data)
        .map_err(|e| format!("Failed to write file: {}", e))?;

    Ok(())
}
