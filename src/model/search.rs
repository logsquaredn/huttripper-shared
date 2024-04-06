use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct SearchRepresentation {
    pub name: String,
    pub system: String,
    pub state: String,
    pub amenities: String,
    pub sanitizedname: String,
    pub sanitizedsystem: String
}