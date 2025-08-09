# Developer Tools Guide

This document outlines the test and development tools available for the Jazz Standards Database CLI project.

## Build System

The project uses [Just](https://github.com/casey/just) as the build system with a comprehensive justfile providing various development commands.

### Core Build Commands

#### Development Build
```bash
just build
```
- Builds debug version with development optimizations
- Faster compilation, includes debug symbols
- Binary location: `target/debug/swing`

#### Production Build
```bash
just build-release
```
- Builds optimized release version
- Slower compilation but optimized performance
- Binary location: `target/release/swing`
- Shows file size after successful build

#### Database Synchronization
```bash
just sync
```
- Synchronizes individual JSON files in `JazzStandards/` with consolidated `JazzStandards.json`
- Adds new songs from individual files to the consolidated database
- Maintains alphabetical ordering by song title
- Removes songs from consolidated database that lack individual files
- Provides detailed output of changes made

#### Project Information
```bash
just info
```
- Displays comprehensive binary information
- Shows file type, size, and version
- Includes database statistics
- Requires release build to be available

### Installation Commands

#### System Installation
```bash
just install
```
- Installs the release binary to `/usr/local/bin/swing`
- Requires `sudo` privileges
- Makes the tool globally available as `swing`

#### System Removal
```bash
just uninstall
```
- Removes the binary from `/usr/local/bin`
- Requires `sudo` privileges

### Packaging Commands

#### Debian Package
```bash
just package-deb
```
- Creates a `.deb` package for Debian/Ubuntu systems
- Output: `dist/swing_1.0.0_amd64.deb`
- Includes proper package metadata and copyright information

#### RPM Package
```bash
just package-rpm
```
- Creates RPM spec file for Red Hat-based systems
- Requires `rpmbuild` to be installed
- Manual step required: `rpmbuild -bb dist/rpm/SPECS/swing.spec`

### Maintenance Commands

#### Clean Build Artifacts
```bash
just clean
```
- Removes all Cargo build artifacts (`cargo clean`)
- Removes packaging directories (`dist/`)

## Rust Development Tools

### Standard Cargo Commands

#### Compilation Check
```bash
cargo check
```
- Fast syntax and type checking without code generation
- Recommended for development workflow

#### Testing
```bash
cargo test
```
- Runs the test suite (currently no tests are defined)
- Framework ready for future test implementation

#### Direct Execution
```bash
cargo run -- [ARGS]
```
- Compiles and runs the application in one step
- Pass command-line arguments after `--`
- Example: `cargo run -- search "autumn leaves"`

### Code Quality Tools

The project is set up to work with standard Rust tooling:

#### Formatting
```bash
cargo fmt
```
- Formats code according to Rust style guidelines

#### Linting
```bash
cargo clippy
```
- Provides additional lints beyond the compiler

## Project Structure

### Source Organization
```
src/
├── main.rs      # Entry point and CLI handling
├── lib.rs       # Library exports and documentation  
├── models.rs    # Data structures (Song, Section, etc.)
├── database.rs  # Database loading functionality
├── search.rs    # Search and filtering logic
├── display.rs   # Output formatting
├── stats.rs     # Statistical analysis
└── cli.rs       # Command-line interface definitions
```

### Data Files
```
JazzStandards.json    # Main consolidated database (embedded)
JazzStandards/        # Individual song JSON files
```

## Dependencies

The project uses minimal, well-maintained dependencies:

- **serde**: Serialization framework with derive macros
- **serde_json**: JSON support for serde
- **clap**: Command-line argument parsing with derive macros

## Development Workflow

### Recommended Development Cycle

1. **Code Changes**: Edit source files in `src/`
2. **Quick Check**: Run `cargo check` for fast feedback
3. **Database Sync**: Use `just sync` if adding new songs or modifying database structure
4. **Test Build**: Use `just build` for development testing
5. **Local Testing**: Run `cargo run -- [command]` to test functionality
6. **Release Build**: Use `just build-release` before final testing
7. **Information Check**: Run `just info` to verify build and database stats

### Binary Information

The built binary includes:
- Embedded JSON database (no external file dependencies)
- Complete chord progression data for 1,382+ jazz standards
- Self-contained executable with no runtime dependencies

### Performance Considerations

- **Debug builds**: Faster compilation, slower execution
- **Release builds**: Slower compilation, optimized execution
- **Database**: Embedded at compile time for zero-latency access
- **Memory usage**: Efficient JSON parsing with serde

## Database Management

### Synchronization Script

The project includes a Python script (`sync_database.py`) for managing the database:

#### Features
- **Bidirectional sync**: Compares individual files with consolidated database
- **Automatic ordering**: Maintains alphabetical sorting by song title
- **Change detection**: Reports additions, removals, and ordering issues
- **Verification**: Confirms successful synchronization
- **Error handling**: Provides detailed feedback on issues

#### Workflow
1. Loads individual JSON files from `JazzStandards/` directory
2. Loads consolidated database from `JazzStandards.json`
3. Identifies songs missing from either source
4. Adds new songs from individual files to consolidated database
5. Removes songs from consolidated database without individual files
6. Sorts all songs alphabetically by title
7. Saves updated consolidated database
8. Verifies final synchronization state

#### Output Example
```
🎵 Jazz Standards Database Sync
========================================
📁 Found 1382 individual songs
📄 Found 1382 consolidated songs
🔍 Analysis:
   Songs only in JazzStandards/: 0
   Songs only in JazzStandards.json: 0
✅ No changes needed - databases are in sync and alphabetical
```

## Testing Strategy

Currently, the project has no automated tests but is structured to support:
- Unit tests for individual modules
- Integration tests for CLI functionality
- Property-based testing for search algorithms
- Performance benchmarks for database operations
- Database integrity tests using the sync script

To add tests, create files in:
- `src/` with `#[cfg(test)]` modules for unit tests
- `tests/` directory for integration tests