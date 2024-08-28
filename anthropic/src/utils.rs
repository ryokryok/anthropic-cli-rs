use std::{
    fs::File,
    io::{Error, Read},
};

use base64::{engine::general_purpose, Engine};

pub fn get_image_mimetype_and_base64(file_path: &str) -> Result<(&str, String), Error> {
    let mut buffer = Vec::new();
    File::open(file_path)?.read_to_end(&mut buffer)?;

    let kind = infer::get(&buffer).expect("file type is known");
    let mime_type = kind.mime_type();
    let base64_string = general_purpose::STANDARD.encode(&buffer);

    Ok((mime_type, base64_string))
}

pub const SUPPORT_MEDIA_TYPE: &'static [&'static str] =
    &["image/jpeg", "image/png", "image/gif", "image/webp"];
