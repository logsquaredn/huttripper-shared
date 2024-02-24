use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
pub struct S3PathAttributes {
    pub sanitized_state: String,
    pub sanitized_system: String,
    pub sanitized_name: String
}

#[derive(Debug, Serialize)]
pub struct ImgLinkUpdates {
    pub thumbnail_image: Option<String>,
    pub images: Vec<String>
}