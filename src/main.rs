/// JustCall - Zero-friction video calling app
/// What: Main entry point, will become Tauri app in Phase 3
/// Why: For now, provides test harness for core utilities
/// Change notes: This file will be replaced by Tauri's main.rs in Phase 3

// Declare our modules
mod core;

// For testing during development
use crate::core::{generate_code_base32_100b, room_id_from_code};

fn main() {
    println!("JustCall - Phase 1 Demo\n");
    
    // Generate codes and derive room IDs
    println!("Partner pairing example:");
    for i in 1..=3 {
        let code = generate_code_base32_100b();
        let room = room_id_from_code(&code);
        println!("  Partner {}:", i);
        println!("    Code: {}", code);
        println!("    Room: {}\n", room);
    }
    
    // Demonstrate deterministic rooms
    println!("Same code = same room:");
    let shared_code = "test-code-1234-5678-abcd";
    println!("  Alice uses: {}", shared_code);
    println!("  Alice joins: {}", room_id_from_code(shared_code));
    println!("  Bob uses:   {}", shared_code);
    println!("  Bob joins:   {}", room_id_from_code(shared_code));
    println!("  ✅ They're in the same room!");
}
