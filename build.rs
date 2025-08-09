fn main() {
    // Tell Cargo to rebuild if JazzStandards.json changes
    println!("cargo:rerun-if-changed=JazzStandards.json");
    
    // Also rebuild if any individual song file changes
    println!("cargo:rerun-if-changed=JazzStandards/");
}