use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct PostgresSearchRepresentation {
    pub name: String,
    pub system: String,
    pub state: String,
    pub amenities: Vec<String>,
    pub sanitizedstate: String,
    pub sanitizedsystem: String
}