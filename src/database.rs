//! Database loading functionality
//! 
//! This module handles loading the embedded jazz standards database from
//! the compiled binary.

use std::error::Error;
use crate::models::Song;

/// Load the jazz standards database from embedded JSON data
/// 
/// The JSON database is embedded into the binary at compile time using
/// a build script that rebuilds the embedded data whenever JazzStandards.json changes,
/// making the CLI tool completely self-contained.
/// 
/// # Returns
/// 
/// A `Result` containing either:
/// - `Ok(Vec<Song>)` - Successfully parsed songs from the database
/// - `Err(Box<dyn Error>)` - JSON parsing error
/// 
/// # Examples
/// 
/// ```no_run
/// use jazz_standards_database::load_jazz_standards;
/// 
/// # fn main() -> Result<(), Box<dyn std::error::Error>> {
/// let songs = load_jazz_standards()?;
/// println!("Loaded {} songs", songs.len());
/// # Ok(())
/// # }
/// ```
pub fn load_jazz_standards() -> Result<Vec<Song>, Box<dyn Error>> {
    // Include the JSON data that was embedded by the build script
    // This ensures the data is rebuilt from JazzStandards.json on each compilation
    let json_content = include_str!("../JazzStandards.json");
    let songs: Vec<Song> = serde_json::from_str(json_content)?;
    Ok(songs)
}