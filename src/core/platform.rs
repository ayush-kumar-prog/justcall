/// Platform-specific configuration and defaults
/// What: Detect OS and provide appropriate default keybindings
/// Why: Each OS has different modifier key conventions (Cmd vs Ctrl)
/// Used by:
///   - SettingsStore::create_defaults() (Phase 2.2)
///   - OnboardingWizard::suggest_keybinds() (Phase 8.3)
///   - GlobalShortcutService::validate_keybind() (Phase 4.1)

/// Keybind defaults for the platform
/// What: Holds default hotkey combinations for core actions
/// Why: Need consistent defaults that feel native on each OS
/// Used by:
///   - Settings UI to show placeholders
///   - First-run setup to pre-fill keybinds
///   - Tests to verify keybind format
#[derive(Debug, Clone, PartialEq)]
pub struct KeybindDefaults {
    pub join_primary: String,
    pub hangup: String,
    pub join_target_prefix: String, // Base for target hotkeys (+ number)
}

/// get_default_keybinds()
/// What: Returns platform-appropriate default keybindings
/// Why: Users expect native modifier keys (Cmd on Mac, Ctrl on Win/Linux)
/// Contract:
///   - Always returns valid KeybindDefaults
///   - Keybinds follow platform conventions
///   - Format matches what Tauri expects
/// Used by:
///   - SettingsStore::create_defaults() - initializes first-run config
///   - OnboardingWizard::show_defaults() - displays in UI
///   - Tests: settings_integration_test, keybind_validation_test
/// Change notes:
///   - If changing format, update GlobalShortcutService parser
///   - Keep consistent with Tauri's keybind syntax
pub fn get_default_keybinds() -> KeybindDefaults {
                #[cfg(target_os = "macos")]
            {
                KeybindDefaults {
                    join_primary: "Cmd+Shift+J".to_string(),
                    hangup: "Cmd+Shift+H".to_string(),
                    join_target_prefix: "Cmd+Shift+".to_string(),
        }
    }
    
    #[cfg(target_os = "windows")]
    {
        KeybindDefaults {
            join_primary: "Ctrl+Shift+J".to_string(),
            hangup: "Ctrl+Shift+H".to_string(),
            join_target_prefix: "Ctrl+Shift+".to_string(),
        }
    }
    
    #[cfg(target_os = "linux")]
    {
        KeybindDefaults {
            join_primary: "Ctrl+Shift+J".to_string(),
            hangup: "Ctrl+Shift+H".to_string(),
            join_target_prefix: "Ctrl+Shift+".to_string(),
        }
    }
    
    // Catch-all for other platforms (BSDs, etc)
    #[cfg(not(any(target_os = "macos", target_os = "windows", target_os = "linux")))]
    {
        KeybindDefaults {
            join_primary: "Ctrl+Alt+J".to_string(),
            hangup: "Ctrl+Alt+H".to_string(),
            join_target_prefix: "Ctrl+Alt+".to_string(),
        }
    }
}

/// get_platform_name()
/// What: Returns human-readable platform name
/// Why: For display in UI and logs
/// Used by:
///   - Settings UI footer
///   - Error messages
///   - Analytics (if added later)
pub fn get_platform_name() -> &'static str {
    #[cfg(target_os = "macos")]
    { "macOS" }
    
    #[cfg(target_os = "windows")]
    { "Windows" }
    
    #[cfg(target_os = "linux")]
    { "Linux" }
    
    #[cfg(not(any(target_os = "macos", target_os = "windows", target_os = "linux")))]
    { "Unknown" }
}

/// Platform-specific behaviors we might need
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct PlatformCapabilities {
    pub has_native_tray: bool,
    pub supports_always_on_top: bool,
    pub needs_accessibility_permission: bool,
    pub supports_global_shortcuts: bool,
}

/// get_platform_capabilities()
/// What: Returns what this platform can/cannot do
/// Why: Some features need platform-specific handling
/// Used by:
///   - App initialization to show warnings
///   - Feature flags in UI
pub fn get_platform_capabilities() -> PlatformCapabilities {
    #[cfg(target_os = "macos")]
    {
        PlatformCapabilities {
            has_native_tray: true,
            supports_always_on_top: true,
            needs_accessibility_permission: true, // For global shortcuts
            supports_global_shortcuts: true,
        }
    }
    
    #[cfg(target_os = "windows")]
    {
        PlatformCapabilities {
            has_native_tray: true,
            supports_always_on_top: true,
            needs_accessibility_permission: false,
            supports_global_shortcuts: true,
        }
    }
    
    #[cfg(target_os = "linux")]
    {
        PlatformCapabilities {
            has_native_tray: true, // Depends on DE, but assume yes
            supports_always_on_top: true,
            needs_accessibility_permission: false,
            supports_global_shortcuts: true, // X11 yes, Wayland maybe
        }
    }
    
    #[cfg(not(any(target_os = "macos", target_os = "windows", target_os = "linux")))]
    {
        PlatformCapabilities {
            has_native_tray: false,
            supports_always_on_top: false,
            needs_accessibility_permission: false,
            supports_global_shortcuts: false,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_defaults_returned() {
        let defaults = get_default_keybinds();
        
        // Should always return non-empty strings
        assert!(!defaults.join_primary.is_empty());
        assert!(!defaults.hangup.is_empty());
        assert!(!defaults.join_target_prefix.is_empty());
    }
    
    #[test]
    fn test_keybind_format() {
        let defaults = get_default_keybinds();
        
        // All keybinds should contain '+' separator
        assert!(defaults.join_primary.contains('+'));
        assert!(defaults.hangup.contains('+'));
        assert!(defaults.join_target_prefix.contains('+'));
        
        // Should end with a key (J, H, or +)
        assert!(defaults.join_primary.ends_with('J'));
        assert!(defaults.hangup.ends_with('H'));
        assert!(defaults.join_target_prefix.ends_with('+'));
    }
    
    #[test]
    #[cfg(target_os = "macos")]
    fn test_macos_uses_cmd() {
        let defaults = get_default_keybinds();
        assert!(defaults.join_primary.contains("Cmd"));
        assert!(defaults.hangup.contains("Cmd"));
        assert_eq!(defaults.join_primary, "Cmd+Opt+J");
    }
    
    #[test]
    #[cfg(any(target_os = "windows", target_os = "linux"))]
    fn test_non_macos_uses_ctrl() {
        let defaults = get_default_keybinds();
        assert!(defaults.join_primary.contains("Ctrl"));
        assert!(defaults.hangup.contains("Ctrl"));
        assert_eq!(defaults.join_primary, "Ctrl+Alt+J");
    }
    
    #[test]
    fn test_platform_name() {
        let name = get_platform_name();
        assert!(!name.is_empty());
        
        // Should be one of known values
        let valid_names = ["macOS", "Windows", "Linux", "Unknown"];
        assert!(valid_names.contains(&name));
    }
    
    #[test]
    fn test_platform_capabilities() {
        let caps = get_platform_capabilities();
        
        // Global shortcuts should be supported on major platforms
        #[cfg(any(target_os = "macos", target_os = "windows", target_os = "linux"))]
        {
            assert!(caps.supports_global_shortcuts);
            assert!(caps.has_native_tray);
        }
        
        // macOS specific
        #[cfg(target_os = "macos")]
        {
            assert!(caps.needs_accessibility_permission);
        }
        
        // Windows/Linux don't need special permissions
        #[cfg(any(target_os = "windows", target_os = "linux"))]
        {
            assert!(!caps.needs_accessibility_permission);
        }
    }
    
    // Edge case tests
    
    #[test]
    fn test_defaults_are_consistent() {
        // Call multiple times, should always return same values
        let defaults1 = get_default_keybinds();
        let defaults2 = get_default_keybinds();
        assert_eq!(defaults1, defaults2);
    }
    
    #[test]
    fn test_keybind_modifiers_ordered() {
        let defaults = get_default_keybinds();
        
        // Modifiers should appear in consistent order
        #[cfg(target_os = "macos")]
        {
            // Cmd should come before Opt
            let join_parts: Vec<&str> = defaults.join_primary.split('+').collect();
            assert_eq!(join_parts[0], "Cmd");
            assert_eq!(join_parts[1], "Opt");
        }
        
        #[cfg(any(target_os = "windows", target_os = "linux"))]
        {
            // Ctrl should come before Alt
            let join_parts: Vec<&str> = defaults.join_primary.split('+').collect();
            assert_eq!(join_parts[0], "Ctrl");
            assert_eq!(join_parts[1], "Alt");
        }
    }
    
    #[test]
    fn test_target_prefix_ends_correctly() {
        let defaults = get_default_keybinds();
        
        // Prefix should end with + for appending numbers
        assert!(defaults.join_target_prefix.ends_with('+'));
        
        // When combined with number, should form valid keybind
        let target_1 = format!("{}1", defaults.join_target_prefix);
        assert!(target_1.contains('+'));
        assert!(target_1.ends_with('1'));
    }
    
    #[test]
    fn test_no_duplicate_keybinds() {
        let defaults = get_default_keybinds();
        
        // Join and hangup should be different
        assert_ne!(defaults.join_primary, defaults.hangup);
        
        // Neither should conflict with target hotkeys
        for i in 1..=9 {
            let target_key = format!("{}{}", defaults.join_target_prefix, i);
            assert_ne!(defaults.join_primary, target_key);
            assert_ne!(defaults.hangup, target_key);
        }
    }
    
    #[test]
    fn test_keybinds_avoid_common_shortcuts() {
        let defaults = get_default_keybinds();
        
        // Should not conflict with common shortcuts
        let common_shortcuts = [
            "Ctrl+C", "Ctrl+V", "Ctrl+X", "Ctrl+A", "Ctrl+S",
            "Cmd+C", "Cmd+V", "Cmd+X", "Cmd+A", "Cmd+S",
            "Ctrl+Tab", "Cmd+Tab", "Alt+Tab",
            "F1", "F5", "F11",
        ];
        
        assert!(!common_shortcuts.contains(&defaults.join_primary.as_str()));
        assert!(!common_shortcuts.contains(&defaults.hangup.as_str()));
    }
    
    #[test]
    fn test_concurrent_access() {
        use std::thread;
        
        // Multiple threads getting defaults simultaneously
        let handles: Vec<_> = (0..10)
            .map(|_| thread::spawn(|| get_default_keybinds()))
            .collect();
        
        let results: Vec<_> = handles.into_iter()
            .map(|h| h.join().unwrap())
            .collect();
        
        // All should be identical
        let first = &results[0];
        for result in &results {
            assert_eq!(result, first);
        }
    }
    
    #[test]
    fn test_platform_name_matches_cfg() {
        let name = get_platform_name();
        
        #[cfg(target_os = "macos")]
        assert_eq!(name, "macOS");
        
        #[cfg(target_os = "windows")]
        assert_eq!(name, "Windows");
        
        #[cfg(target_os = "linux")]
        assert_eq!(name, "Linux");
    }
}
