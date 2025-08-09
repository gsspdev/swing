# 🎵 Swing - Jazz Standards Database CLI - Installation Guide

Multiple installation methods are available for macOS and Linux systems.

## Quick Install (Recommended)

### One-line install script:
```bash
curl -fsSL https://raw.githubusercontent.com/user/swing/main/install.sh | bash
```

or with wget:
```bash
wget -qO- https://raw.githubusercontent.com/user/swing/main/install.sh | bash
```

## Installation Methods

### 1. 🍺 Homebrew (macOS)

```bash
# Add tap (once available)
brew tap user/swing

# Install
brew install swing

# Verify
swing --version
```

### 2. 📦 Package Managers (Linux)

#### Debian/Ubuntu (.deb):
```bash
# Download package
wget https://github.com/user/swing/releases/download/v1.0.0/swing_1.0.0_amd64.deb

# Install
sudo dpkg -i swing_1.0.0_amd64.deb

# Fix dependencies if needed
sudo apt-get install -f
```

#### Red Hat/CentOS (.rpm):
```bash
# Download package  
wget https://github.com/user/swing/releases/download/v1.0.0/swing-1.0.0-1.x86_64.rpm

# Install
sudo rpm -ivh swing-1.0.0-1.x86_64.rpm
```

### 3. 🔧 Manual Installation

#### Download pre-compiled binary:
```bash
# macOS (Intel)
curl -L -o swing https://github.com/user/swing/releases/download/v1.0.0/swing-macos-x86_64

# macOS (Apple Silicon)
curl -L -o swing https://github.com/user/swing/releases/download/v1.0.0/swing-macos-aarch64

# Linux (x86_64)
curl -L -o swing https://github.com/user/swing/releases/download/v1.0.0/swing-linux-x86_64

# Make executable and install
chmod +x swing
sudo mv swing /usr/local/bin/
```

### 4. 🦀 From Source (Rust required)

```bash
# Clone repository
git clone https://github.com/user/swing.git
cd swing

# Build and install with justfile (recommended)
just build-release
sudo just install

# Or manually with cargo
cargo build --release

# Install to system directory
sudo cp target/release/swing /usr/local/bin/

# Or install to user directory (add ~/.local/bin to your PATH)
mkdir -p ~/.local/bin
cp target/release/swing ~/.local/bin/
```

## Verification

Test your installation:

```bash
# Check version
swing --version

# View help
swing --help

# Test basic functionality
swing stats
swing search "miles davis"
swing show "All Blues"
```

## Troubleshooting

### Command not found
If you get "command not found", ensure `/usr/local/bin` is in your PATH:

```bash
# Add to your shell profile (~/.bashrc, ~/.zshrc, etc.)
export PATH="/usr/local/bin:$PATH"

# Reload your shell
source ~/.bashrc  # or ~/.zshrc
```

### Permission denied
If installation fails with permission errors:

```bash
# Ensure you have write access or use sudo
sudo just install

# Or install to user directory
cargo install --path . --root ~/.local
export PATH="$HOME/.local/bin:$PATH"
```

### Missing dependencies
The binary is self-contained and requires no runtime dependencies. If you're building from source, you need:

- Rust 1.75+ (`curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh`)

### Binary not executable
If you get permission denied when trying to run the binary:

```bash
chmod +x /path/to/swing
```

## Uninstallation

### Remove binary:
```bash
# Using justfile
just uninstall

# Or manually
sudo rm /usr/local/bin/swing
```

### Package managers:
```bash
# Homebrew
brew uninstall swing

# Debian/Ubuntu
sudo apt remove swing

# Red Hat/CentOS
sudo rpm -e swing
```

## System Requirements

- **macOS**: 10.12 Sierra or later
- **Linux**: Any modern distribution (glibc 2.17+)
- **Architecture**: x86_64 (Intel/AMD) or aarch64 (ARM64)
- **Disk Space**: ~2MB for binary (database is embedded)
- **Memory**: Minimal (~10MB runtime)

## Database Information

The CLI tool includes an embedded database containing:
- **1,383 jazz standards** from The Real Book Volume I Sixth Edition
- **Full chord progressions** organized by song sections
- **Metadata**: composers, keys, rhythms, time signatures
- **No external files** required - completely self-contained

## Getting Started

After installation, try these commands:

```bash
# Explore the database
swing stats --detailed

# Search for songs
swing search "blue"
swing search "thelonious monk"

# Filter by criteria  
swing filter --key C
swing filter --rhythm "bossa nova"
swing filter --composer "miles davis" --detailed

# View specific songs
swing show "Giant Steps"
swing show "Body and Soul"

# List available options
swing list keys
swing list rhythms
swing list composers
```

---

## Support

- **Issues**: Report bugs at https://github.com/user/swing/issues
- **Documentation**: Full CLI help available with `swing --help`
- **Updates**: Check for new releases at https://github.com/user/swing/releases

Happy jazz exploring! 🎵