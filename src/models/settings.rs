/// Settings data structures with serialization support
/// What: Complete schema for app configuration
/// Why: Structured data enables type-safe settings management
/// Used by:
///   - SettingsStore::save/load (Phase 2.2)
///   - Settings UI for display/edit (Phase 3.2)
///   - CallController for runtime config (Phase 5.1)

use serde::{Deserialize, Serialize};

/// Root settings object containing all configuration
/// What: Top-level container for all app settings
/// Why: Single source of truth for configuration
/// Used by: SettingsStore, main app initialization
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Settings {
    /// Schema version for migrations
    pub version: u32,
    
    /// Global app settings
    pub app_settings: AppSettings,
    
    /// Keyboard shortcuts configuration
    pub keybinds: Keybinds,
    
    /// List of call targets (people/groups)
    pub targets: Vec<Target>,
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            version: 1,
            app_settings: AppSettings::default(),
            keybinds: Keybinds::default(),
            targets: Vec::new(),
        }
    }
}

/// Global application settings
/// What: App-wide preferences and behavior settings
/// Why: Users need to customize app behavior
/// Used by: App initialization, tray menu, window manager
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct AppSettings {
    /// Start app when OS boots
    #[serde(default)]
    pub autostart: bool,
    
    /// Keep call window on top of others
    #[serde(default = "default_true")]
    pub always_on_top: bool,
    
    /// Play sound when someone joins
    #[serde(default = "default_true")]
    pub play_join_sound: bool,
    
    /// Show notifications for call events
    #[serde(default = "default_true")]
    pub show_notifications: bool,
    
    /// Theme preference (for future use)
    #[serde(default)]
    pub theme: Theme,
}

impl Default for AppSettings {
    fn default() -> Self {
        Self {
            autostart: false,
            always_on_top: true,
            play_join_sound: true,
            show_notifications: true,
            theme: Theme::System,
        }
    }
}

/// Theme options
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum Theme {
    Light,
    Dark,
    System,
}

impl Default for Theme {
    fn default() -> Self {
        Theme::System
    }
}

/// Keyboard shortcut configuration
/// What: All hotkey bindings for the app
/// Why: Users need customizable shortcuts that don't conflict
/// Used by: GlobalShortcutService, Settings UI, first-run setup
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Keybinds {
    /// Hotkey to join primary target
    pub join_primary: String,
    
    /// Hotkey to end any active call
    pub hangup: String,
    
    /// Per-target hotkeys (target_id -> hotkey)
    #[serde(default)]
    pub target_hotkeys: std::collections::HashMap<String, String>,
    
    /// In-call shortcuts (future use)
    #[serde(default)]
    pub toggle_mute: Option<String>,
    #[serde(default)]
    pub toggle_video: Option<String>,
}

impl Default for Keybinds {
    fn default() -> Self {
        // Get platform-specific defaults
        let platform_defaults = crate::core::get_default_keybinds();
        
        Self {
            join_primary: platform_defaults.join_primary,
            hangup: platform_defaults.hangup,
            target_hotkeys: std::collections::HashMap::new(),
            toggle_mute: None,
            toggle_video: None,
        }
    }
}

/// A call target (person or group)
/// What: Represents someone you can call
/// Why: Need to store pairing info and preferences per target
/// Used by: Target list UI, CallController, invite system
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Target {
    /// Unique identifier (generated)
    pub id: String,
    
    /// Display name chosen by user
    pub label: String,
    
    /// Pairing code (high-entropy, shared secret)
    pub code: String,
    
    /// Target type for UI/behavior differences
    #[serde(rename = "type")]
    pub target_type: TargetType,
    
    /// Is this the primary (default) target?
    #[serde(default)]
    pub is_primary: bool,
    
    /// Per-target call preferences
    pub call_defaults: CallDefaults,
    
    /// When this target was added (ISO 8601)
    pub created_at: String,
    
    /// Custom notes (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notes: Option<String>,
}

/// Type of target
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum TargetType {
    Person,
    Group,
}

/// Per-target call settings
/// What: Default behavior when calling this target
/// Why: Different people/groups may need different settings
/// Used by: CallController when initiating calls
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct CallDefaults {
    /// Start with microphone on/off
    #[serde(default = "default_true")]
    pub start_with_audio: bool,
    
    /// Start with camera on/off
    #[serde(default = "default_true")]
    pub start_with_video: bool,
    
    /// Use specific display name (overrides global)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
}

impl Default for CallDefaults {
    fn default() -> Self {
        Self {
            start_with_audio: true,
            start_with_video: true,
            display_name: None,
        }
    }
}

// Helper for serde defaults
fn default_true() -> bool {
    true
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_settings_default() {
        let settings = Settings::default();
        assert_eq!(settings.version, 1);
        assert!(settings.targets.is_empty());
        assert!(!settings.app_settings.autostart);
    }
    
    #[test]
    fn test_settings_serialization() {
        let mut settings = Settings::default();
        
        // Add a target
        settings.targets.push(Target {
            id: "tg_123".to_string(),
            label: "Alice".to_string(),
            code: "test-code-1234".to_string(),
            target_type: TargetType::Person,
            is_primary: true,
            call_defaults: CallDefaults::default(),
            created_at: "2024-01-01T00:00:00Z".to_string(),
            notes: Some("Best friend".to_string()),
        });
        
        // Serialize
        let json = serde_json::to_string_pretty(&settings).unwrap();
        assert!(json.contains("\"version\": 1"));
        assert!(json.contains("\"label\": \"Alice\""));
        
        // Deserialize
        let parsed: Settings = serde_json::from_str(&json).unwrap();
        assert_eq!(settings, parsed);
    }
    
    #[test]
    fn test_backwards_compatibility() {
        // Old format missing new fields
        let old_json = r#"{
            "version": 1,
            "app_settings": {
                "autostart": true,
                "always_on_top": false
            },
            "keybinds": {
                "join_primary": "Cmd+Opt+J",
                "hangup": "Cmd+Opt+H"
            },
            "targets": []
        }"#;
        
        let settings: Settings = serde_json::from_str(old_json).unwrap();
        assert_eq!(settings.version, 1);
        assert!(settings.app_settings.autostart);
        // New fields should have defaults
        assert!(settings.app_settings.play_join_sound);
        assert!(settings.keybinds.toggle_mute.is_none());
    }
    
    #[test]
    fn test_minimal_json() {
        // Absolute minimum valid JSON
        let minimal = r#"{"version": 1}"#;
        let result = serde_json::from_str::<Settings>(minimal);
        assert!(result.is_err(), "Should require required fields");
        
        // Minimum with required fields
        let minimal_valid = r#"{
            "version": 1,
            "app_settings": {},
            "keybinds": {
                "join_primary": "Ctrl+J",
                "hangup": "Ctrl+H"
            },
            "targets": []
        }"#;
        
        let settings: Settings = serde_json::from_str(minimal_valid).unwrap();
        assert_eq!(settings.targets.len(), 0);
    }
    
    // Edge case tests
    
    #[test]
    fn test_empty_strings() {
        let target = Target {
            id: "".to_string(), // Empty ID
            label: "".to_string(), // Empty label
            code: "".to_string(), // Empty code
            target_type: TargetType::Person,
            is_primary: true,
            call_defaults: CallDefaults::default(),
            created_at: "2024-01-01T00:00:00Z".to_string(),
            notes: Some("".to_string()), // Empty note
        };
        
        let json = serde_json::to_string(&target).unwrap();
        let parsed: Target = serde_json::from_str(&json).unwrap();
        assert_eq!(target, parsed);
    }
    
    #[test]
    fn test_unicode_in_labels() {
        let mut settings = Settings::default();
        settings.targets.push(Target {
            id: "tg_unicode".to_string(),
            label: "张三 & फ्रेंड्स 🎉".to_string(),
            code: "test-code".to_string(),
            target_type: TargetType::Group,
            is_primary: false,
            call_defaults: CallDefaults::default(),
            created_at: "2024-01-01T00:00:00Z".to_string(),
            notes: Some("多语言测试 🌍".to_string()),
        });
        
        let json = serde_json::to_string(&settings).unwrap();
        let parsed: Settings = serde_json::from_str(&json).unwrap();
        assert_eq!(settings.targets[0].label, parsed.targets[0].label);
    }
    
    #[test]
    fn test_large_settings() {
        let mut settings = Settings::default();
        
        // Add many targets
        for i in 0..100 {
            settings.targets.push(Target {
                id: format!("tg_{}", i),
                label: format!("Target {}", i),
                code: format!("code-{}", i),
                target_type: if i % 2 == 0 { TargetType::Person } else { TargetType::Group },
                is_primary: i == 0,
                call_defaults: CallDefaults::default(),
                created_at: "2024-01-01T00:00:00Z".to_string(),
                notes: if i % 3 == 0 { Some(format!("Note {}", i)) } else { None },
            });
            
            // Add custom hotkey for first 10
            if i < 10 {
                settings.keybinds.target_hotkeys.insert(
                    format!("tg_{}", i),
                    format!("Cmd+Opt+{}", i)
                );
            }
        }
        
        // Should serialize/deserialize without issues
        let json = serde_json::to_string(&settings).unwrap();
        let parsed: Settings = serde_json::from_str(&json).unwrap();
        assert_eq!(settings.targets.len(), parsed.targets.len());
        assert_eq!(settings.keybinds.target_hotkeys.len(), 10);
    }
    
    #[test]
    fn test_duplicate_target_ids() {
        let mut settings = Settings::default();
        
        // Add targets with same ID (shouldn't happen but test behavior)
        settings.targets.push(Target {
            id: "duplicate".to_string(),
            label: "First".to_string(),
            code: "code1".to_string(),
            target_type: TargetType::Person,
            is_primary: true,
            call_defaults: CallDefaults::default(),
            created_at: "2024-01-01T00:00:00Z".to_string(),
            notes: None,
        });
        
        settings.targets.push(Target {
            id: "duplicate".to_string(),
            label: "Second".to_string(),
            code: "code2".to_string(),
            target_type: TargetType::Person,
            is_primary: false,
            call_defaults: CallDefaults::default(),
            created_at: "2024-01-01T00:00:00Z".to_string(),
            notes: None,
        });
        
        // Should serialize both (validation happens elsewhere)
        let json = serde_json::to_string(&settings).unwrap();
        assert!(json.contains("First"));
        assert!(json.contains("Second"));
    }
    
    #[test]
    fn test_malformed_json_handling() {
        let test_cases = vec![
            ("", "Empty string"),
            ("{", "Unclosed brace"),
            (r#"{"version": "not a number"}"#, "Wrong type for version"),
            (r#"{"version": 999999}"#, "Future version"),
            (r#"null"#, "Null value"),
            (r#"[]"#, "Array instead of object"),
        ];
        
        for (json, description) in test_cases {
            let result = serde_json::from_str::<Settings>(json);
            assert!(result.is_err(), "Should fail for: {}", description);
        }
    }
    
    #[test]
    fn test_optional_fields() {
        let target_json = r#"{
            "id": "test",
            "label": "Test",
            "code": "test-code",
            "type": "person",
            "call_defaults": {},
            "created_at": "2024-01-01T00:00:00Z"
        }"#;
        
        let target: Target = serde_json::from_str(target_json).unwrap();
        assert!(!target.is_primary); // Should default to false
        assert!(target.notes.is_none()); // Should be None
    }
    
    #[test]
    fn test_settings_equality() {
        let s1 = Settings::default();
        let s2 = Settings::default();
        assert_eq!(s1, s2);
        
        let mut s3 = Settings::default();
        s3.version = 2;
        assert_ne!(s1, s3);
    }
}
