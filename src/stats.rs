//! Database statistics

use std::collections::HashMap;
use crate::models::Song;

pub fn show_statistics(songs: &[Song], detailed: bool) {
    println!("\n📊 Jazz Standards Database Statistics");
    println!("════════════════════════════════════════");
    println!("Total songs: {}", songs.len());

    let songs_with_composers = songs.iter().filter(|s| s.composer.is_some()).count();
    let songs_with_keys = songs.iter().filter(|s| s.key.is_some()).count();
    let songs_with_rhythms = songs.iter().filter(|s| s.rhythm.is_some()).count();
    let songs_with_time_sigs = songs.iter().filter(|s| s.time_signature.is_some()).count();
    let songs_with_sections = songs.iter().filter(|s| s.sections.is_some()).count();

    println!("Songs with composers: {}/{} ({:.1}%)", 
        songs_with_composers, songs.len(), 
        songs_with_composers as f64 / songs.len() as f64 * 100.0);
    println!("Songs with keys: {}/{} ({:.1}%)", 
        songs_with_keys, songs.len(),
        songs_with_keys as f64 / songs.len() as f64 * 100.0);
    println!("Songs with rhythms: {}/{} ({:.1}%)", 
        songs_with_rhythms, songs.len(),
        songs_with_rhythms as f64 / songs.len() as f64 * 100.0);
    println!("Songs with time signatures: {}/{} ({:.1}%)", 
        songs_with_time_sigs, songs.len(),
        songs_with_time_sigs as f64 / songs.len() as f64 * 100.0);
    println!("Songs with sections: {}/{} ({:.1}%)", 
        songs_with_sections, songs.len(),
        songs_with_sections as f64 / songs.len() as f64 * 100.0);

    if detailed {
        show_detailed_statistics(songs);
    }
}

pub fn show_detailed_statistics(songs: &[Song]) {
    println!("\n🎹 Key Distribution:");
    println!("──────────────────");
    let mut key_counts = HashMap::new();
    for song in songs {
        if let Some(key) = &song.key {
            *key_counts.entry(key.clone()).or_insert(0) += 1;
        }
    }
    let mut key_vec: Vec<_> = key_counts.iter().collect();
    key_vec.sort_by(|a, b| b.1.cmp(a.1));
    for (key, count) in key_vec.iter().take(10) {
        println!("  {:<6}: {:>4} songs", key, count);
    }

    println!("\n🎤 Rhythm Distribution:");
    println!("─────────────────────");
    let mut rhythm_counts = HashMap::new();
    for song in songs {
        if let Some(rhythm) = &song.rhythm {
            *rhythm_counts.entry(rhythm.clone()).or_insert(0) += 1;
        }
    }
    let mut rhythm_vec: Vec<_> = rhythm_counts.iter().collect();
    rhythm_vec.sort_by(|a, b| b.1.cmp(a.1));
    for (rhythm, count) in rhythm_vec.iter().take(10) {
        println!("  {:<20}: {:>4} songs", rhythm, count);
    }

    println!("\n🎵 Top Composers:");
    println!("───────────────");
    let mut composer_counts = HashMap::new();
    for song in songs {
        if let Some(composer) = &song.composer {
            *composer_counts.entry(composer.clone()).or_insert(0) += 1;
        }
    }
    let mut composer_vec: Vec<_> = composer_counts.iter().collect();
    composer_vec.sort_by(|a, b| b.1.cmp(a.1));
    for (composer, count) in composer_vec.iter().take(10) {
        println!("  {:<25}: {:>4} songs", composer, count);
    }
}

pub fn list_field_values(songs: &[Song], field: &str) {
    match field.to_lowercase().as_str() {
        "keys" | "key" => {
            let mut keys: Vec<_> = songs
                .iter()
                .filter_map(|s| s.key.as_ref())
                .collect::<std::collections::HashSet<_>>()
                .into_iter()
                .collect();
            keys.sort();
            println!("🎹 Available Keys ({}):", keys.len());
            for key in keys {
                println!("  {}", key);
            }
        }
        "rhythms" | "rhythm" => {
            let mut rhythms: Vec<_> = songs
                .iter()
                .filter_map(|s| s.rhythm.as_ref())
                .collect::<std::collections::HashSet<_>>()
                .into_iter()
                .collect();
            rhythms.sort();
            println!("🎤 Available Rhythms ({}):", rhythms.len());
            for rhythm in rhythms {
                println!("  {}", rhythm);
            }
        }
        "composers" | "composer" => {
            let mut composers: Vec<_> = songs
                .iter()
                .filter_map(|s| s.composer.as_ref())
                .collect::<std::collections::HashSet<_>>()
                .into_iter()
                .collect();
            composers.sort();
            println!("🎵 Available Composers ({}):", composers.len());
            for composer in composers {
                println!("  {}", composer);
            }
        }
        "time-signatures" | "time" => {
            let mut time_sigs: Vec<_> = songs
                .iter()
                .filter_map(|s| s.time_signature.as_ref())
                .collect::<std::collections::HashSet<_>>()
                .into_iter()
                .collect();
            time_sigs.sort();
            println!("⏱️  Available Time Signatures ({}):", time_sigs.len());
            for time_sig in time_sigs {
                println!("  {}", time_sig);
            }
        }
        _ => {
            println!("❌ Unknown field '{}'. Available fields: keys, rhythms, composers, time-signatures", field);
        }
    }
}