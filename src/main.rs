/// JustCall - Zero-friction video calling app
/// What: Main entry point, will become Tauri app in Phase 3
/// Why: For now, provides test harness for core utilities
/// Change notes: This file will be replaced by Tauri's main.rs in Phase 3

// Declare our modules
mod core;

// For testing during development
use crate::core::{generate_code_base32_100b, room_id_from_code, get_default_keybinds, get_platform_name, get_platform_capabilities};

fn main() {
    println!("JustCall - Phase 1 Demo\n");
    
    // Show platform info
    println!("Platform Detection:");
    println!("  Running on: {}", get_platform_name());
    let keybinds = get_default_keybinds();
    println!("  Default hotkeys:");
    println!("    Join: {}", keybinds.join_primary);
    println!("    Hangup: {}", keybinds.hangup);
    println!("    Target prefix: {}[1-9]\n", keybinds.join_target_prefix);
    
    // Show capabilities
    let caps = get_platform_capabilities();
    println!("  Platform capabilities:");
    println!("    System tray: {}", if caps.has_native_tray { "✅" } else { "❌" });
    println!("    Always on top: {}", if caps.supports_always_on_top { "✅" } else { "❌" });
    println!("    Global shortcuts: {}", if caps.supports_global_shortcuts { "✅" } else { "❌" });
    if caps.needs_accessibility_permission {
        println!("    ⚠️  Requires accessibility permission for global shortcuts");
    }
    println!();
    
    // Generate a pairing code and room
    println!("Example pairing:");
    let code = generate_code_base32_100b();
    let room = room_id_from_code(&code);
    println!("  Your code: {}", code);
    println!("  Your room: {}", room);
    println!("  Share the code with your partner to connect!");
}
