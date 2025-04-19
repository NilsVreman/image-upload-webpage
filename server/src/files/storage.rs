use axum::body;
use chrono::{DateTime, SecondsFormat, Utc};
use core::fmt;
use image::ImageError;
use std::{io, path::PathBuf, str::FromStr};
use uuid::Uuid;

use super::errors::ParseError;

const MAX_THUMBNAILS_SIZE: u32 = 100;
static BAD_CHARS: &[char] = &[
    '.', '/', '\\', ' ', '!', '<', '>', '|', ':', '(', ')', '&', ';', '#', '?', '*',
];

trait Pipe: Sized {
    fn pipe<F, R>(self, f: F) -> R
    where
        F: FnOnce(Self) -> R,
    {
        f(self)
    }
}
impl<T> Pipe for T {}

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

#[derive(Debug, Clone)]
pub struct FileName {
    pub sanitized_name: String,
    pub uploaded_at: DateTime<Utc>,
    extension: String,
    id: Uuid,
}

impl fmt::Display for FileName {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}_{}_{}.{}",
            self.uploaded_at.to_rfc3339_opts(SecondsFormat::Secs, true),
            self.sanitized_name,
            self.id,
            self.extension,
        )
    }
}

impl FromStr for FileName {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // split once, then pull each field in order
        let mut parts = s.split('_');

        // let _test = parts.next();
        let uploaded_at = parts
            .next()
            .ok_or(ParseError::FileName("missing timestamp"))?
            .pipe(DateTime::parse_from_rfc3339)
            .map(DateTime::<Utc>::from)?;

        let sanitized_name = parts
            .next()
            .ok_or(ParseError::FileName("missing file stem"))?
            .to_owned();

        let mut tail = parts
            .next()
            .ok_or(ParseError::FileName("missing uuid+extension"))?
            .split('.');

        let id = tail
            .next()
            .ok_or(ParseError::FileName("missing uuid"))?
            .parse::<Uuid>()?;

        let extension = tail
            .next()
            .ok_or(ParseError::FileName("missing extension"))?
            .to_owned();

        Ok(FileName {
            sanitized_name,
            extension,
            uploaded_at,
            id,
        })
    }
}

pub trait Sanitize {
    fn sanitize(&self) -> Option<FileName>;
}

impl Sanitize for Option<&str> {
    fn sanitize(&self) -> Option<FileName> {
        let bad_char = |c| BAD_CHARS.contains(&c) || c.is_control();

        // get the first non-empty substring that doesn't contain bad characters (replace _ with - )
        let file_name = self
            .and_then(|n| n.split(bad_char).find(|&s| !s.is_empty()))?
            .replace('_', "-");
        let file_extension = self.and_then(|n| n.rsplit_once('.')).map(|(_, ext)| ext)?;

        if file_name.is_empty() {
            return None;
        }

        Some(FileName {
            sanitized_name: file_name.to_string(),
            extension: file_extension.to_string(),
            uploaded_at: Utc::now(),
            id: Uuid::new_v4(),
        })
    }
}

pub async fn setup() -> io::Result<()> {
    let image_folder = get_folder(ImageType::Image)?;
    let thumbnail_folder = get_folder(ImageType::Thumbnail)?;

    ensure_folder_exists(&image_folder).await?;
    ensure_folder_exists(&thumbnail_folder).await
}

pub async fn write_image(
    file_name: &FileName,
    image_buffer: &body::Bytes,
) -> Result<(), ImageError> {
    tokio::fs::write(get_file_path(file_name, ImageType::Image)?, &image_buffer)
        .await
        .map_err(ImageError::IoError)
}

pub fn write_thumbnail(file_name: &FileName, image_buffer: &body::Bytes) -> Result<(), ImageError> {
    image::load_from_memory(image_buffer)?
        .thumbnail(MAX_THUMBNAILS_SIZE, MAX_THUMBNAILS_SIZE)
        .save(get_file_path(file_name, ImageType::Thumbnail)?)
}

pub async fn get_all_thumbnail_names() -> io::Result<Vec<String>> {
    get_all_names(ImageType::Thumbnail).await
}

pub async fn get_image(name: &String) -> io::Result<body::Bytes> {
    get_file(name, ImageType::Image).await
}

pub async fn get_thumbnail(name: &String) -> io::Result<body::Bytes> {
    get_file(name, ImageType::Thumbnail).await
}

async fn get_all_names(image_type: ImageType) -> io::Result<Vec<String>> {
    let mut entries = tokio::fs::read_dir(get_folder(image_type)?).await?;
    let mut image_names: Vec<String> = Vec::new();
    while let Some(entry) = entries.next_entry().await? {
        image_names.push(entry.file_name().to_string_lossy().to_string());
    }
    Ok(image_names)
}

async fn get_file(name: &String, image_type: ImageType) -> io::Result<body::Bytes> {
    tokio::fs::read(get_folder(image_type)?.join(name))
        .await
        .map(body::Bytes::from)
}

fn get_folder(image_type: ImageType) -> io::Result<PathBuf> {
    std::env::current_dir().map(|x| x.join(image_type.to_string()))
}

fn get_file_path(file_name: &FileName, image_type: ImageType) -> io::Result<PathBuf> {
    get_folder(image_type).map(|x| x.join(file_name.to_string()))
}

async fn ensure_folder_exists(folder: &PathBuf) -> io::Result<()> {
    if !tokio::fs::try_exists(&folder).await? {
        tokio::fs::create_dir_all(&folder).await?;
    }
    Ok(())
}
