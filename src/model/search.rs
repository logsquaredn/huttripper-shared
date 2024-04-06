use serde::{Deserialize, Serialize};
use sqlx::postgres::PgRow;
use sqlx::Row;

#[derive(Debug, Deserialize, Serialize)]
pub struct HutSearchRepresentation {
    pub name: String,
    pub sanitized_name: String,
    pub system: String,
    pub sanitized_system: String,
    pub state: String,
    pub amenities: String
}

impl HutSearchRepresentation {

    pub fn map_from(row: PgRow) -> Result<Self, sqlx::Error> {
        let amenities: Vec<String> = row.try_get("amenities")?;
        amenities.join(" ");
        Ok(Self {
            name: row.try_get("name")?,
            sanitized_name: row.try_get("sanitizedname")?,
            system: row.try_get("system")?,
            sanitized_system: row.try_get("sanitizedsystem")?,
            state: row.try_get("state")?,
            amenities: amenities.join(" ")
        })
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct TripReportSearchRepresentation {
    pub sanitized_hut_name: String,
    pub hut_conditions: String,
    pub weather_conditions: String,
    pub riding_conditions: String
}

impl TripReportSearchRepresentation {

    pub fn map_from(row: PgRow) -> Result<Self, sqlx::Error> {
        Ok(Self {
            sanitized_hut_name: row.try_get("sanitizedhutname")?,
            hut_conditions: row.try_get("hutconditions")?,
            weather_conditions: row.try_get("weatherconditions")?,
            riding_conditions: row.try_get("ridingconditions")?
        })
    }
}