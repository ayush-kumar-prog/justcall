/// Quick verification script for Phase 1.3
/// Run with: cargo run --bin verify-platform

use justcall::core::{get_default_keybinds, get_platform_name, get_platform_capabilities};

fn main() {
    println!("=== Platform Detection Verification ===\n");
    
    // 1. Check what OS is detected
    println!("1. Detected OS: {}", get_platform_name());
    println!("   Expected: macOS (since you're on Mac)\n");
    
    // 2. Check keybinds are Mac-specific
    let keybinds = get_default_keybinds();
    println!("2. Default Keybinds:");
    println!("   Join: {} (should contain 'Cmd')", keybinds.join_primary);
    println!("   Hangup: {} (should contain 'Cmd')", keybinds.hangup);
    println!("   Prefix: {} (should be 'Cmd+Opt+')\n", keybinds.join_target_prefix);
    
    // 3. Check capabilities
    let caps = get_platform_capabilities();
    println!("3. Platform Capabilities:");
    println!("   Tray support: {} (should be ✓)", if caps.has_native_tray { "✓" } else { "✗" });
    println!("   Always on top: {} (should be ✓)", if caps.supports_always_on_top { "✓" } else { "✗" });
    println!("   Global shortcuts: {} (should be ✓)", if caps.supports_global_shortcuts { "✓" } else { "✗" });
    println!("   Needs permission: {} (should be ✓ on macOS)", if caps.needs_accessibility_permission { "✓" } else { "✗" });
    
    // 4. Test consistency
    println!("\n4. Consistency Check:");
    let kb1 = get_default_keybinds();
    let kb2 = get_default_keybinds();
    println!("   Multiple calls return same values: {}", if kb1.join_primary == kb2.join_primary { "✓" } else { "✗" });
    
    // 5. Test target hotkeys
    println!("\n5. Target Hotkeys:");
    for i in 1..=3 {
        println!("   Target {}: {}{}", i, keybinds.join_target_prefix, i);
    }
    
    println!("\n✅ If all above looks correct, Phase 1.3 is working!");
}
