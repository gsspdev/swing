#!/usr/bin/env python3
"""
Jazz Standards Database Sync Script

This script:
1. Compares individual JSON files in JazzStandards/ with the consolidated JazzStandards.json
2. Adds new songs from JazzStandards/ to JazzStandards.json in alphabetical order
3. Verifies that songs are stored in alphabetical order
4. Ensures both sources match after synchronization

Usage: python3 sync_database.py
"""

import json
import os
import sys
from pathlib import Path
from typing import Dict, List, Set


def load_json_file(file_path: Path) -> Dict:
    """Load and parse a JSON file."""
    try:
        with open(file_path, 'r', encoding='utf-8') as f:
            return json.load(f)
    except Exception as e:
        print(f"❌ Error loading {file_path}: {e}")
        return {}


def save_json_file(file_path: Path, data: List[Dict]) -> None:
    """Save data to a JSON file with proper formatting."""
    try:
        with open(file_path, 'w', encoding='utf-8') as f:
            json.dump(data, f, indent=2, ensure_ascii=False)
        print(f"✅ Saved {len(data)} songs to {file_path}")
    except Exception as e:
        print(f"❌ Error saving {file_path}: {e}")
        sys.exit(1)


def get_individual_songs() -> Dict[str, Dict]:
    """Load all individual song files from JazzStandards/ directory."""
    individual_dir = Path("JazzStandards")
    if not individual_dir.exists():
        print(f"❌ Directory {individual_dir} not found")
        sys.exit(1)
    
    songs = {}
    json_files = list(individual_dir.glob("*.json"))
    
    print(f"📁 Loading {len(json_files)} individual song files...")
    
    for json_file in json_files:
        song_data = load_json_file(json_file)
        if song_data and 'Title' in song_data:
            title = song_data['Title']
            songs[title] = song_data
        else:
            print(f"⚠️  Warning: Invalid or missing Title in {json_file}")
    
    return songs


def get_consolidated_songs() -> List[Dict]:
    """Load songs from the consolidated JazzStandards.json file."""
    consolidated_file = Path("JazzStandards.json")
    if not consolidated_file.exists():
        print(f"❌ Consolidated file {consolidated_file} not found")
        sys.exit(1)
    
    songs = load_json_file(consolidated_file)
    if not isinstance(songs, list):
        print(f"❌ Expected array in {consolidated_file}, got {type(songs)}")
        sys.exit(1)
    
    print(f"📄 Loaded {len(songs)} songs from consolidated file")
    return songs


def check_alphabetical_order(songs: List[Dict]) -> bool:
    """Check if songs are in alphabetical order by Title."""
    print("\n🔤 Checking alphabetical order...")
    
    titles = [song.get('Title', '') for song in songs]
    sorted_titles = sorted(titles, key=lambda x: x.lower())
    
    is_sorted = titles == sorted_titles
    
    if is_sorted:
        print("✅ Songs are in alphabetical order")
    else:
        print("❌ Songs are NOT in alphabetical order")
        
        # Find first mismatch
        for i, (actual, expected) in enumerate(zip(titles, sorted_titles)):
            if actual != expected:
                print(f"   First mismatch at position {i}: '{actual}' should be '{expected}'")
                break
    
    return is_sorted


def find_differences(individual_songs: Dict[str, Dict], consolidated_songs: List[Dict]) -> tuple:
    """Find songs that are in individual files but not in consolidated file, and vice versa."""
    
    # Create set of titles from consolidated file
    consolidated_titles = {song.get('Title', '') for song in consolidated_songs}
    individual_titles = set(individual_songs.keys())
    
    # Find differences
    missing_from_consolidated = individual_titles - consolidated_titles
    missing_from_individual = consolidated_titles - individual_titles
    
    return missing_from_consolidated, missing_from_individual


def sync_databases() -> bool:
    """Main synchronization function."""
    print("🎵 Jazz Standards Database Sync")
    print("=" * 40)
    
    # Load individual songs
    individual_songs = get_individual_songs()
    print(f"📁 Found {len(individual_songs)} individual songs")
    
    # Load consolidated songs
    consolidated_songs = get_consolidated_songs()
    print(f"📄 Found {len(consolidated_songs)} consolidated songs")
    
    # Find differences
    missing_from_consolidated, missing_from_individual = find_differences(individual_songs, consolidated_songs)
    
    print(f"\n🔍 Analysis:")
    print(f"   Songs only in JazzStandards/: {len(missing_from_consolidated)}")
    print(f"   Songs only in JazzStandards.json: {len(missing_from_individual)}")
    
    # Handle missing from individual files
    if missing_from_individual:
        print(f"\n⚠️  Songs in consolidated but missing individual files:")
        for title in sorted(missing_from_individual):
            print(f"     - {title}")
        
        print(f"\n🗑️  Removing {len(missing_from_individual)} songs from consolidated file:")
        # Filter out songs that don't have individual files
        consolidated_songs = [song for song in consolidated_songs 
                            if song.get('Title') not in missing_from_individual]
        for title in sorted(missing_from_individual):
            print(f"     - {title}")
    
    # Check if we need to sort (either new songs, removals, or not alphabetical)
    is_alphabetical = check_alphabetical_order(consolidated_songs)
    needs_update = len(missing_from_consolidated) > 0 or len(missing_from_individual) > 0 or not is_alphabetical
    
    if missing_from_consolidated:
        print(f"\n➕ Adding {len(missing_from_consolidated)} new songs to consolidated file:")
        for title in sorted(missing_from_consolidated):
            print(f"     + {title}")
            consolidated_songs.append(individual_songs[title])
    
    if needs_update:
        # Sort all songs alphabetically by title (case-insensitive)
        print("\n🔤 Sorting all songs alphabetically...")
        consolidated_songs.sort(key=lambda x: x.get('Title', '').lower())
        
        # Save updated consolidated file
        save_json_file(Path("JazzStandards.json"), consolidated_songs)
        
        if missing_from_consolidated:
            print(f"✅ Successfully added {len(missing_from_consolidated)} new songs and sorted alphabetically")
        else:
            print("✅ Successfully sorted existing songs alphabetically")
    else:
        print("\n✅ No changes needed - databases are in sync and alphabetical")
    
    return needs_update


def verify_sync() -> bool:
    """Verify that both databases are now in sync."""
    print("\n🔍 Final verification...")
    
    # Reload both databases
    individual_songs = get_individual_songs()
    consolidated_songs = get_consolidated_songs()
    
    # Check alphabetical order
    is_alphabetical = check_alphabetical_order(consolidated_songs)
    
    # Check for differences
    missing_from_consolidated, missing_from_individual = find_differences(individual_songs, consolidated_songs)
    
    # Report results
    if missing_from_individual:
        print(f"⚠️  Warning: {len(missing_from_individual)} songs still missing individual files")
        for title in sorted(missing_from_individual):
            print(f"     - {title}")
    
    is_synced = len(missing_from_consolidated) == 0
    
    if is_synced and is_alphabetical:
        print("✅ Databases are fully synchronized and in alphabetical order")
    elif is_synced:
        print("⚠️  Databases are synchronized but not in alphabetical order")
    else:
        print("❌ Databases are not synchronized")
    
    print(f"\n📊 Final counts:")
    print(f"   Individual songs: {len(individual_songs)}")
    print(f"   Consolidated songs: {len(consolidated_songs)}")
    
    return is_synced and is_alphabetical


def main():
    """Main function."""
    try:
        # Change to script directory
        script_dir = Path(__file__).parent
        os.chdir(script_dir)
        
        # Sync databases
        changes_made = sync_databases()
        
        # Verify final state
        success = verify_sync()
        
        # Exit with appropriate code
        if success:
            if changes_made:
                print("\n🎉 Sync completed successfully with changes!")
            else:
                print("\n🎉 Databases verified - already in sync!")
            sys.exit(0)
        else:
            print("\n💥 Sync completed but issues remain")
            sys.exit(1)
            
    except KeyboardInterrupt:
        print("\n\n⏹️  Sync interrupted by user")
        sys.exit(1)
    except Exception as e:
        print(f"\n💥 Unexpected error: {e}")
        sys.exit(1)


if __name__ == "__main__":
    main()