use serde::{Deserialize, Serialize};
use sqlx::{postgres::PgRow, Row};

#[derive(Debug, Deserialize, Serialize)]
pub struct S3PathAttributes {
    pub sanitized_state: String,
    pub sanitized_system: String,
    pub sanitized_name: String
}

impl S3PathAttributes {

    pub fn map_from(row: PgRow) -> Result<Self, sqlx::Error> {
        Ok(Self { 
            sanitized_state: row.try_get("sanitizedstate")?,
            sanitized_system: row.try_get("sanitizedsystem")?,
            sanitized_name: row.try_get("sanitizedname")?
        })
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ImgLinkUpdates {
    pub thumbnail_image: Option<String>,
    pub images: Vec<String>
}