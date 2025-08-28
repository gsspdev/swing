//! Database loading

use std::error::Error;
use crate::models::Song;

pub fn load_jazz_standards() -> Result<Vec<Song>, Box<dyn Error>> {
    let json_content = include_str!("../JazzStandards.json");
    let songs: Vec<Song> = serde_json::from_str(json_content)?;
    Ok(songs)
}