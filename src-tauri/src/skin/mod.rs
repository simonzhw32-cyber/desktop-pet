use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SkinConfig {
    #[serde(rename = "schemaVersion")]
    pub schema_version: String,
    #[serde(rename = "coreProfile")]
    pub core_profile: String,
    pub id: String,
    pub name: String,
    pub author: String,
    pub canvas: Canvas,
    pub anchor: Anchor,
    pub states: HashMap<String, State>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Canvas {
    pub width: u32,
    pub height: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Anchor {
    pub x: u32,
    pub y: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct State {
    pub frames: Vec<String>,
    pub fps: Option<u8>,
    #[serde(rename = "durationMs")]
    pub duration_ms: Option<u32>,
    #[serde(rename = "loopType")]
    pub loop_type: Option<String>,
}