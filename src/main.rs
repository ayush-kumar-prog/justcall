/// JustCall - Zero-friction video calling app
/// What: Main entry point, will become Tauri app in Phase 3
/// Why: For now, provides test harness for core utilities
/// Change notes: This file will be replaced by Tauri's main.rs in Phase 3

// Declare our modules
mod core;

// For testing during development
use crate::core::generate_code_base32_100b;

fn main() {
    println!("JustCall - Code Generation Test\n");
    
    // Generate a few codes to verify it's working
    println!("Generating sample pairing codes:");
    for i in 1..=5 {
        let code = generate_code_base32_100b();
        println!("  {}. {}", i, code);
    }
    
    println!("\nAll codes are unique and cryptographically secure.");
    println!("Share these codes with your partners to establish pairing.");
}
