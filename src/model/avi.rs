use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize)]
pub struct AviReport {
    pub zone_name: String,
    pub avi_center: String,
    pub avi_center_link: String,
    pub off_season: bool,
    pub travel_advice: String,
    pub danger: String,
    pub danger_level: i32,
    pub zone_link: String,
    pub start_time: Option<chrono::NaiveDateTime>,
    pub end_time: Option<chrono::NaiveDateTime>
}

impl AviReport {

    pub fn from_zone_properties(zps: &ZoneProperties) -> Result<Self, chrono::ParseError> {
        let start_time: Option<chrono::NaiveDateTime> = if let Some(start_date) = &zps.start_date {
            Some(NaiveDateTime::parse_from_str(&start_date, "%Y-%m-%dT%H:%M:%S")?)
        } else { None };
        let end_time: Option<chrono::NaiveDateTime> = if let Some(end_date) = &zps.end_date {
            Some(NaiveDateTime::parse_from_str(&end_date, "%Y-%m-%dT%H:%M:%S")?)
        } else { None };

        Ok(AviReport { 
            zone_name: zps.name.clone(), 
            avi_center: zps.center.clone(), 
            avi_center_link: zps.center_link.clone(), 
            off_season: zps.off_season, 
            travel_advice: zps.travel_advice.clone(), 
            danger: zps.danger.clone(), 
            danger_level: zps.danger_level, 
            zone_link: zps.link.clone(), 
            start_time: start_time, 
            end_time: end_time
        })
    }
}

#[derive(Debug, Deserialize)]
pub struct ZoneProperties {
    pub name: String,
    pub center: String,
    pub center_link: String,
    pub off_season: bool,
    pub travel_advice: String,
    pub danger: String,
    pub danger_level: i32,
    pub link: String,
    pub start_date: Option<String>,
    pub end_date: Option<String>
}