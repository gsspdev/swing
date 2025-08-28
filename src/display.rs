//! Display formatting

use crate::models::Song;

pub fn print_song_summary(song: &Song) {
    println!("📄 {}", song.title);
    
    if let Some(composer) = &song.composer {
        println!("   🎵 Composer: {}", composer);
    }
    
    if let Some(key) = &song.key {
        println!("   🎹 Key: {}", key);
    }
    
    if let Some(rhythm) = &song.rhythm {
        println!("   🎤 Rhythm: {}", rhythm);
    }
    
    if let Some(time_sig) = &song.time_signature {
        println!("   ⏱️  Time: {}", time_sig);
    }
    
    if let Some(sections) = &song.sections {
        println!("   📋 Sections: {}", sections.len());
    }
}

pub fn print_song_detailed(song: &Song) {
    println!("\n═══════════════════════════════════════");
    println!("📄 {}", song.title);
    println!("═══════════════════════════════════════");
    
    if let Some(composer) = &song.composer {
        println!("🎵 Composer: {}", composer);
    }
    
    if let Some(key) = &song.key {
        println!("🎹 Key: {}", key);
    }
    
    if let Some(rhythm) = &song.rhythm {
        println!("🎤 Rhythm: {}", rhythm);
    }
    
    if let Some(time_sig) = &song.time_signature {
        println!("⏱️  Time Signature: {}", time_sig);
    }

    if let Some(sections) = &song.sections {
        println!("\n📋 Song Structure ({} sections):", sections.len());
        println!("───────────────────────────────────────");
        
        for (i, section) in sections.iter().enumerate() {
            if let Some(label) = &section.label {
                print!("  Section {}", label);
                if let Some(repeats) = section.repeats {
                    print!(" (repeats: {})", repeats);
                }
                println!();
            } else {
                println!("  Section {}", i + 1);
            }
            if let Some(main_seg) = &section.main_segment {
                if let Some(chords) = &main_seg.chords {
                    println!("    🎼 Main: {}", chords);
                }
            }
            if let Some(endings) = &section.endings {
                for (j, ending) in endings.iter().enumerate() {
                    if let Some(chords) = &ending.chords {
                        println!("    🔚 Ending {}: {}", j + 1, chords);
                    }
                }
            }
            
            println!();
        }
    }
}