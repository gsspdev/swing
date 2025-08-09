# Jazz Standards Database

A command-line tool for searching, analyzing, and displaying jazz standards with detailed chord progression data. Built in Rust for performance and reliability.

## Features

- 🎵 Comprehensive database of jazz standards with chord progressions
- 🔍 Powerful search by title, composer, key, and rhythm
- 📊 Statistical analysis of songs, composers, and musical elements
- 🎼 Detailed chord progression display
- ⚡ Self-contained binary with embedded database
- 🚀 Fast performance with zero runtime dependencies

## Installation

### Quick Install (Recommended)

```bash
# Clone and build the project
git clone <repository-url>
cd swing
cargo build --release

# Install the binary to your system
sudo cp target/release/swing /usr/local/bin/

# Or install to user directory (add ~/.local/bin to your PATH)
mkdir -p ~/.local/bin
cp target/release/swing ~/.local/bin/
```

### Alternative Installation

If you have `just` installed:
```bash
just build-release
sudo just install
```

### Verify Installation

```bash
# Check that the tool is installed correctly
swing --version
swing --help
```

## Usage

Once installed, you can run `swing` from anywhere in your terminal.

### Basic Commands

```bash
# Show all available commands
swing --help

# Display database statistics
swing stats

# List all songs
swing list

# Search for songs by title
swing search "Body and Soul"

# Display detailed song information with chord progressions
swing show "All The Things You Are"
```

### Search Examples

```bash
# Search by composer
swing search --composer "Duke Ellington"

# Search by musical key
swing search --key "Bb"

# Search by rhythm/style
swing search --rhythm "Medium Swing"

# Search by time signature
swing search --time-signature "4/4"

# Combine multiple search criteria
swing search --composer "Jerome Kern" --key "Bb"

# Partial matching (case-insensitive)
swing search "blue"  # Finds "Blue Moon", "Blue Skies", etc.
```

### View Song Details

```bash
# Show complete song information including chord progressions
swing show "Autumn Leaves"
swing show "Giant Steps"
swing show "Fly Me to the Moon"
```

### Statistical Analysis

```bash
# View comprehensive database statistics
swing stats

# Example output includes:
# - Total number of songs
# - Most prolific composers
# - Key distribution
# - Rhythm patterns
# - Time signatures
```

## Understanding the Output

### Song Information
Each song displays:
- **Title**: Song name
- **Composer**: Primary composer
- **Key**: Musical key
- **Rhythm**: Style and tempo (e.g., "Medium Swing", "Ballad")
- **Time Signature**: Musical meter (e.g., "4/4", "3/4")
- **Sections**: Song structure (A, B, etc.) with chord progressions

### Chord Progressions
Chords are displayed as measures separated by pipes:
```
Fmaj7 Ab7 | Dbmaj7 E7 | Amaj7
```
This represents:
- Measure 1: Fmaj7, Ab7
- Measure 2: Dbmaj7, E7
- Measure 3: Amaj7

## Common Use Cases

### For Musicians
```bash
# Find songs to practice in a specific key
swing search --key "F"

# Discover songs by your favorite composer
swing search --composer "Bill Evans"

# Find ballads for slow practice
swing search --rhythm "Ballad"
```

### For Music Students
```bash
# Study chord progressions of standards
swing show "Giant Steps"
swing show "All The Things You Are"

# Analyze key distribution in jazz
swing stats
```

### For Educators
```bash
# Find songs appropriate for different skill levels
swing search --rhythm "Medium Swing"

# Get comprehensive song information for lesson planning
swing show "Autumn Leaves"
```

## Getting Help

```bash
# Show all available commands and options
swing --help

# Get help for specific commands
swing search --help
swing show --help
```

## System Requirements

- Compatible with macOS, Linux, and Windows
- No additional dependencies required (self-contained binary)
- Minimal system resources needed

## Troubleshooting

**Command not found**: Ensure the binary is in your PATH or use the full path to the executable.

**Permission denied**: You may need to make the binary executable:
```bash
chmod +x /path/to/swing
```

## Support

For issues and feature requests, please [create an issue](link-to-issues) in the repository.