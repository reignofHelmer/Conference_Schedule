// src/models.rs
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Session {
    pub id: Option<u32>,
    pub title: String,
    pub speaker: String,
    pub time: String,
    pub duration: u32,
    pub description: String,
}
