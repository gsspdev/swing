//! CLI definitions

use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "swing")]
#[command(about = "A CLI tool for searching and analyzing the Jazz Standards database")]
#[command(long_about = "
A comprehensive CLI tool for exploring the Jazz Standards database containing > 1,000 songs.
Search by title/composer, filter by musical criteria, view detailed chord progressions, 
and analyze database statistics.

Examples:
  swing search \"miles davis\"
  swing filter --key C --rhythm swing
  swing show \"All Blues\"
  swing stats --detailed
")]
#[command(version)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Search songs by title or composer
    Search {
        term: String,
        #[arg(short, long)]
        detailed: bool,
    },
    /// Filter songs by musical criteria
    Filter {
        #[arg(short, long)]
        key: Option<String>,
        #[arg(short, long)]
        rhythm: Option<String>,
        #[arg(short, long)]
        time: Option<String>,
        #[arg(short, long)]
        composer: Option<String>,
        #[arg(short, long)]
        detailed: bool,
    },
    /// Show database statistics
    Stats {
        #[arg(short, long)]
        detailed: bool,
    },
    /// List unique field values
    List {
        #[arg(value_parser = ["keys", "rhythms", "composers", "time-signatures", "time"])]
        field: String,
    },
    /// Show detailed song information
    Show {
        #[arg()]
        title: String,
    },
}