use sqlx::Row;

#[derive(Debug, serde::Serialize, serde::Deserialize, Clone)]
pub struct TripReport {
    pub id: String,
    pub first_name: String,
    pub last_name: String,
    pub trip_start_date: chrono::NaiveDate,
    pub trip_end_date: chrono::NaiveDate,
    pub weather_conditions: Option<String>,
    pub hut_conditions: String,
    pub riding_conditions: Option<String>,
    pub approved: bool,
    pub image_links: Vec<String>,
}

impl TripReport {

    pub fn map_from(row: sqlx::postgres::PgRow) -> Result<Self, sqlx::Error> {
        let maybe_image_links: Option<Vec<String>> = row.try_get("images")?;
        Ok(Self {
            id: row.try_get("id")?,
            first_name: row.try_get("firstname")?,
            last_name: row.try_get("lastname")?,
            trip_start_date: row.try_get("tripstartdate")?,
            trip_end_date: row.try_get("tripenddate")?,
            weather_conditions: row.try_get("weatherconditions")?,
            hut_conditions: row.try_get("hutconditions")?,
            riding_conditions: row.try_get("ridingconditions")?,
            approved: row.try_get("approved")?,
            image_links: maybe_image_links.unwrap_or(vec![])
        })
    }
}