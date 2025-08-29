/// JustCall - Zero-friction video calling app
/// What: Main entry point, will become Tauri app in Phase 3
/// Why: For now, provides test harness for core utilities
/// Change notes: This file will be replaced by Tauri's main.rs in Phase 3

// Declare our modules
mod core;
mod models;
mod storage;

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
    
    // Demo settings serialization
    println!("\nSettings Schema Demo:");
    let mut settings = crate::models::Settings::default();
    
    // Add a sample target
    settings.targets.push(crate::models::Target {
        id: "tg_demo".to_string(),
        label: "Demo Partner".to_string(),
        code,
        target_type: crate::models::TargetType::Person,
        is_primary: true,
        call_defaults: crate::models::CallDefaults::default(),
        created_at: "2024-01-01T00:00:00Z".to_string(),
        notes: None,
    });
    
    // Show JSON format
    match serde_json::to_string_pretty(&settings) {
        Ok(json) => {
            println!("  Settings would be saved as:");
            println!("  {}", json.lines().take(10).collect::<Vec<_>>().join("\n  "));
            println!("  ... (truncated)");
        }
        Err(e) => println!("  Error serializing: {}", e),
    }
    
    // Demo settings persistence
    println!("\nSettings Store Demo:");
    match crate::storage::SettingsStore::load() {
        Ok(store) => {
            println!("  ✅ Settings loaded from: ~/Library/Application Support/justcall/");
            println!("  Current targets: {}", store.get_targets().len());
            if let Some(primary) = store.get_primary_target() {
                println!("  Primary target: {}", primary.label);
            }
        }
        Err(e) => {
            println!("  ℹ️  No settings found (first run): {}", e);
            println!("  Settings will be created on first save.");
        }
    }
    
    // Demo state machine
    println!("\nCall State Machine Demo:");
    use crate::core::CallState;
    let mut state = CallState::default();
    println!("  Initial state: {} (busy: {})", state, state.is_busy());
    
    // Show valid transitions
    println!("  Valid transitions from {}:", state);
    for next in [CallState::Idle, CallState::Connecting, CallState::InCall, CallState::Disconnecting] {
        if state.can_transition_to(next) {
            println!("    → {}", next);
        }
    }
    
    // Demo a call flow
    println!("\n  Simulating call flow:");
    println!("    {} → Connecting", state);
    state = CallState::Connecting;
    println!("    {} → InCall", state);
    state = CallState::InCall;
    println!("    {} → Disconnecting", state);
    state = CallState::Disconnecting;
    println!("    {} → Idle", state);
}
