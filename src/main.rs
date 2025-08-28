//! Jazz Standards Database CLI

use clap::Parser;
use jazz_standards_database::{
    Cli, Commands, 
    load_jazz_standards,
    search_songs, filter_songs,
    print_song_summary, print_song_detailed,
    show_statistics, list_field_values
};

fn main() {
    let cli = Cli::parse();

    let songs = match load_jazz_standards() {
        Ok(songs) => songs,
        Err(e) => {
            eprintln!("❌ Error loading jazz standards database: {}", e);
            std::process::exit(1);
        }
    };

    match cli.command {
        Commands::Search { term, detailed } => {
            handle_search_command(&songs, &term, detailed);
        }
        Commands::Filter { key, rhythm, time, composer, detailed } => {
            handle_filter_command(&songs, key.as_deref(), rhythm.as_deref(), 
                                time.as_deref(), composer.as_deref(), detailed);
        }
        Commands::Stats { detailed } => {
            show_statistics(&songs, detailed);
        }
        Commands::List { field } => {
            list_field_values(&songs, &field);
        }
        Commands::Show { title } => {
            handle_show_command(&songs, &title);
        }
    }
}

fn handle_search_command(songs: &[jazz_standards_database::Song], term: &str, detailed: bool) {
    let results = search_songs(songs, term);
    
    if results.is_empty() {
        println!("🔍 No songs found matching '{}'", term);
        return;
    }
    
    println!("🔍 Found {} song(s) matching '{}':", results.len(), term);
    println!("───────────────────────────────────────");
    
    for song in results {
        if detailed {
            print_song_detailed(song);
        } else {
            print_song_summary(song);
            println!();
        }
    }
}

fn handle_filter_command(
    songs: &[jazz_standards_database::Song], 
    key: Option<&str>, 
    rhythm: Option<&str>, 
    time: Option<&str>, 
    composer: Option<&str>, 
    detailed: bool
) {
    let results = filter_songs(songs, key, rhythm, time, composer);
    
    if results.is_empty() {
        println!("🔍 No songs found matching the filter criteria");
        return;
    }
    
    println!("🔍 Found {} song(s) matching filter criteria:", results.len());
    println!("───────────────────────────────────────");
    
    for song in results {
        if detailed {
            print_song_detailed(song);
        } else {
            print_song_summary(song);
            println!();
        }
    }
}

fn handle_show_command(songs: &[jazz_standards_database::Song], title: &str) {
    if let Some(song) = songs.iter().find(|s| s.title.eq_ignore_ascii_case(title)) {
        print_song_detailed(song);
    } else {
        println!("❌ Song '{}' not found", title);
        println!("💡 Try using 'swing search \"{}\"' for partial matches", title);
    }
}