# Homebrew Formula for Jazz Standards Database CLI
class Swing < Formula
  desc "CLI tool for searching and analyzing jazz standards with chord progressions"
  homepage "https://github.com/user/swing"  # Update with actual repo
  url "https://github.com/user/swing/archive/v1.0.0.tar.gz"  # Update with actual repo
  sha256 ""  # Will be calculated from the actual release
  license "MIT"  # Update with actual license

  depends_on "rust" => :build

  def install
    system "cargo", "install", *std_cargo_args
  end

  test do
    # Test that the binary runs and shows version
    assert_match "swing", shell_output("#{bin}/swing --version")
    
    # Test basic functionality
    output = shell_output("#{bin}/swing stats")
    assert_match "Total songs: 1383", output
    assert_match "Jazz Standards Database Statistics", output
    
    # Test search functionality
    search_output = shell_output("#{bin}/swing search 'miles davis'")
    assert_match "Found", search_output
    
    # Test help system
    help_output = shell_output("#{bin}/swing --help")
    assert_match "CLI tool for searching and analyzing", help_output
  end
end