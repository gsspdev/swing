//! Search and filtering

use crate::models::Song;

pub fn search_songs<'a>(songs: &'a [Song], term: &str) -> Vec<&'a Song> {
    let term_lower = term.to_lowercase();
    songs
        .iter()
        .filter(|song| {
            song.title.to_lowercase().contains(&term_lower)
                || song.composer
                    .as_ref()
                    .map_or(false, |c| c.to_lowercase().contains(&term_lower))
        })
        .collect()
}

pub fn filter_songs<'a>(
    songs: &'a [Song], 
    key: Option<&str>, 
    rhythm: Option<&str>, 
    time: Option<&str>, 
    composer: Option<&str>
) -> Vec<&'a Song> {
    songs
        .iter()
        .filter(|song| {
            if let Some(k) = key {
                if !song.key.as_ref().map_or(false, |sk| sk.eq_ignore_ascii_case(k)) {
                    return false;
                }
            }
            if let Some(r) = rhythm {
                if !song.rhythm.as_ref().map_or(false, |sr| sr.to_lowercase().contains(&r.to_lowercase())) {
                    return false;
                }
            }
            if let Some(t) = time {
                if !song.time_signature.as_ref().map_or(false, |st| st == t) {
                    return false;
                }
            }
            if let Some(c) = composer {
                if !song.composer.as_ref().map_or(false, |sc| sc.to_lowercase().contains(&c.to_lowercase())) {
                    return false;
                }
            }
            
            true
        })
        .collect()
}