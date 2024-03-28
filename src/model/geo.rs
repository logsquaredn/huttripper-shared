use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct GeoJSON {
    #[serde(rename = "type")]
    pub type_field: String,
    pub features: Vec<Feature>
}

#[derive(Debug, Deserialize, Serialize)]
pub struct HutsJSON {
    pub features: Vec<Feature>
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Feature {
    pub id: Option<String>,
    #[serde(rename = "type")]
    pub type_field: String,
    pub geometry: serde_json::Value,
    pub properties: Properties
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Properties {
    pub class: String,
    pub title: String,
    pub description: Option<String>,
    #[serde(rename = "folderId")]
    pub folder_id: Option<String>,
    #[serde(rename = "marker-color")]
    pub marker_color: Option<String>,
    #[serde(rename = "marker-symbol")]
    pub marker_symbol: Option<String>,
    pub stroke: Option<String>,
    pub huttripper_type: Option<String>
}
