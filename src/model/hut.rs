use serde::{Deserialize, Serialize};
use sqlx::{postgres::PgRow, Row};

#[derive(Debug, Serialize, Deserialize)]
pub struct HutsFilterResult {
    pub name: String,
    pub sanitized_name: String,
    pub state: String,
    pub sanitized_state: String,
    pub system: String,
    pub max_capacity: i32,
    pub point: Vec<f64>,
    pub image_links: Vec<String>
}

impl HutsFilterResult {

    pub fn map_from(row: PgRow) -> Result<Self, sqlx::Error> {
        let thumbnail_image: Option<String> = row.try_get("thumbnailimage")?;
        let image_links = if thumbnail_image.is_none() || thumbnail_image.clone().unwrap().is_empty() {
            vec![]
        } else {
            vec![thumbnail_image.unwrap()]
        };
        
        Ok(Self { 
            name: row.try_get("name")?, 
            sanitized_name: row.try_get("sanitizedname")?,
            state: row.try_get("state")?, 
            sanitized_state: row.try_get("sanitizedstate")?, 
            system: row.try_get("system")?, 
            max_capacity: row.try_get("maxcapacity")?,
            point: vec![row.try_get("longitude")?, row.try_get("latitude")?],
            image_links: image_links
        })
    }
}