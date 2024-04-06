use serde::{Deserialize, Serialize};
use sqlx::postgres::PgRow;
use sqlx::Row;

#[derive(Debug, Deserialize, Serialize)]
pub struct SearchRepresentation {
    pub name: String,
    pub sanitizedname: String,
    pub system: String,
    pub sanitizedsystem: String,
    pub state: String,
    pub amenities: String
}

impl SearchRepresentation {

    pub fn map_from(row: PgRow) -> Result<Self, sqlx::Error> {
        let amenities: Vec<String> = row.try_get("amenities")?;
        amenities.join(" ");
        Ok(Self {
            name: row.try_get("name")?,
            sanitizedname: row.try_get("sanitizedname")?,
            system: row.try_get("system")?,
            sanitizedsystem: row.try_get("sanitizedsystem")?,
            state: row.try_get("state")?,
            amenities: amenities.join(" ")
        })
    }
}