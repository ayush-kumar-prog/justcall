/// Settings store for persistent configuration
/// What: Loads/saves settings from/to disk in JSON format
/// Why: App needs to remember user configuration between sessions
/// Used by:
///   - Main app initialization to load settings
///   - Settings UI to save changes
///   - Target management to persist targets
///   - Tests for integration testing

use anyhow::{Context, Result};
use std::fs;
use std::path::{Path, PathBuf};

use crate::models::{Settings, Target};

/// Settings store that manages persistence
/// What: Handles all settings I/O operations
/// Why: Centralizes settings management with proper error handling
/// Used by: App initialization, settings UI, hotkey registration
pub struct SettingsStore {
    /// Current settings in memory
    settings: Settings,
    /// Path to settings file
    file_path: PathBuf,
}

impl SettingsStore {
    /// Load settings from default location
    /// What: Loads from platform-specific app data directory
    /// Why: Standard location for app settings on each OS
    /// Used by: App startup, first-run detection
    /// Calls: dirs::config_dir(), Self::load_from_path()
    /// Change notes: If dirs crate changes API, update path resolution
    pub fn load() -> Result<Self> {
        let config_dir = dirs::config_dir()
            .context("Failed to determine config directory")?;
        
        let app_dir = config_dir.join("blink");
        let file_path = app_dir.join("settings.json");
        
        Self::load_from_path(file_path)
    }
    
    /// Load settings from specific path
    /// What: Loads settings or creates defaults if missing
    /// Why: Allows testing with temp directories
    /// Used by: load(), tests
    /// Calls: fs::read_to_string, serde_json::from_str
    pub fn load_from_path<P: AsRef<Path>>(path: P) -> Result<Self> {
        let file_path = path.as_ref().to_path_buf();
        
        let settings = if file_path.exists() {
            let contents = fs::read_to_string(&file_path)
                .with_context(|| format!("Failed to read settings from {:?}", file_path))?;
            
            serde_json::from_str(&contents)
                .with_context(|| format!("Failed to parse settings from {:?}", file_path))?
        } else {
            // File doesn't exist, use defaults
            Settings::default()
        };
        
        Ok(Self {
            settings,
            file_path,
        })
    }
    
    /// Create new store with specific path
    /// What: Creates store with defaults at given path
    /// Why: Testing needs custom paths
    /// Used by: Tests, first-run setup
    pub fn new_with_path<P: AsRef<Path>>(path: P) -> Self {
        Self {
            settings: Settings::default(),
            file_path: path.as_ref().to_path_buf(),
        }
    }
    
    /// Save current settings to disk
    /// What: Persists settings to JSON file
    /// Why: User changes need to be saved
    /// Used by: Settings UI save button, add/remove target
    /// Calls: fs::create_dir_all, serde_json::to_string_pretty, fs::write
    /// Change notes: If changing format, ensure backwards compatibility
    pub fn save(&self) -> Result<()> {
        // Ensure directory exists
        if let Some(parent) = self.file_path.parent() {
            fs::create_dir_all(parent)
                .with_context(|| format!("Failed to create directory {:?}", parent))?;
        }
        
        // Serialize to pretty JSON
        let json = serde_json::to_string_pretty(&self.settings)
            .context("Failed to serialize settings")?;
        
        // Write atomically (write to temp, then rename)
        let temp_path = self.file_path.with_extension("json.tmp");
        fs::write(&temp_path, json)
            .with_context(|| format!("Failed to write settings to {:?}", temp_path))?;
        
        fs::rename(&temp_path, &self.file_path)
            .with_context(|| format!("Failed to save settings to {:?}", self.file_path))?;
        
        Ok(())
    }
    
    /// Get target by ID
    /// What: Finds a specific target
    /// Why: Need to look up targets for hotkey actions
    /// Used by: CallController::join(), hotkey handlers
    pub fn get_target(&self, id: &str) -> Option<&Target> {
        self.settings.targets.iter().find(|t| t.id == id)
    }
    
    /// Get primary target
    /// What: Returns the default target
    /// Why: Primary hotkey needs quick access
    /// Used by: CallController::join_primary()
    pub fn get_primary_target(&self) -> Option<&Target> {
        self.settings.targets.iter().find(|t| t.is_primary)
    }
    
    /// Add a new target
    /// What: Adds target and saves to disk
    /// Why: User adds new call partners
    /// Used by: Settings UI, invite acceptance
    /// Calls: save()
    pub fn add_target(&mut self, target: Target) -> Result<()> {
        // If this is the first target, make it primary
        let mut target = target;
        if self.settings.targets.is_empty() {
            target.is_primary = true;
        }
        
        self.settings.targets.push(target);
        self.save()
    }
    
    /// Remove target by ID
    /// What: Removes target and saves
    /// Why: User removes partners
    /// Used by: Settings UI remove button
    /// Calls: save()
    pub fn remove_target(&mut self, id: &str) -> Result<bool> {
        let initial_len = self.settings.targets.len();
        self.settings.targets.retain(|t| t.id != id);
        
        if self.settings.targets.len() < initial_len {
            // If we removed the primary, make the first one primary
            if !self.settings.targets.is_empty() && 
               !self.settings.targets.iter().any(|t| t.is_primary) {
                self.settings.targets[0].is_primary = true;
            }
            
            self.save()?;
            Ok(true)
        } else {
            Ok(false)
        }
    }
    
    /// Update existing target
    /// What: Updates target properties
    /// Why: User edits target settings
    /// Used by: Settings UI edit
    pub fn update_target(&mut self, target: Target) -> Result<bool> {
        if let Some(existing) = self.settings.targets.iter_mut().find(|t| t.id == target.id) {
            *existing = target;
            self.save()?;
            Ok(true)
        } else {
            Ok(false)
        }
    }
    
    /// Get all targets
    /// What: Returns all configured targets
    /// Why: Settings UI needs to display list
    /// Used by: Settings UI, target list
    pub fn get_targets(&self) -> &[Target] {
        &self.settings.targets
    }
    
    /// Get mutable reference to settings
    /// What: Direct access to settings
    /// Why: Bulk updates, migrations
    /// Used by: Migration code, tests
    pub fn settings_mut(&mut self) -> &mut Settings {
        &mut self.settings
    }
    
    /// Get reference to settings
    /// What: Read-only access to settings
    /// Why: Reading configuration
    /// Used by: App initialization, keybind setup
    pub fn settings(&self) -> &Settings {
        &self.settings
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;
    use crate::models::{TargetType, CallDefaults};
    
    fn create_test_target(id: &str) -> Target {
        Target {
            id: id.to_string(),
            label: format!("Test {}", id),
            code: format!("test-code-{}", id),
            target_type: TargetType::Person,
            is_primary: false,
            call_defaults: CallDefaults::default(),
            created_at: "2024-01-01T00:00:00Z".to_string(),
            notes: None,
        }
    }
    
    #[test]
    fn test_save_and_load() {
        let temp_dir = TempDir::new().unwrap();
        let file_path = temp_dir.path().join("settings.json");
        
        // Create and save
        let mut store = SettingsStore::new_with_path(&file_path);
        store.add_target(create_test_target("1")).unwrap();
        store.add_target(create_test_target("2")).unwrap();
        
        // Load from disk
        let loaded = SettingsStore::load_from_path(&file_path).unwrap();
        assert_eq!(loaded.get_targets().len(), 2);
        assert_eq!(loaded.get_target("1").unwrap().label, "Test 1");
        assert_eq!(loaded.get_target("2").unwrap().label, "Test 2");
    }
    
    #[test]
    fn test_missing_file_creates_defaults() {
        let temp_dir = TempDir::new().unwrap();
        let file_path = temp_dir.path().join("nonexistent.json");
        
        let store = SettingsStore::load_from_path(&file_path).unwrap();
        assert_eq!(store.settings().version, 1);
        assert!(store.get_targets().is_empty());
    }
    
    #[test]
    fn test_first_target_becomes_primary() {
        let temp_dir = TempDir::new().unwrap();
        let file_path = temp_dir.path().join("settings.json");
        
        let mut store = SettingsStore::new_with_path(&file_path);
        let target = create_test_target("first");
        store.add_target(target).unwrap();
        
        let primary = store.get_primary_target().unwrap();
        assert_eq!(primary.id, "first");
        assert!(primary.is_primary);
    }
    
    #[test]
    fn test_remove_target() {
        let temp_dir = TempDir::new().unwrap();
        let file_path = temp_dir.path().join("settings.json");
        
        let mut store = SettingsStore::new_with_path(&file_path);
        store.add_target(create_test_target("1")).unwrap();
        store.add_target(create_test_target("2")).unwrap();
        
        assert!(store.remove_target("1").unwrap());
        assert_eq!(store.get_targets().len(), 1);
        assert!(store.get_target("1").is_none());
        assert!(store.get_target("2").is_some());
        
        // Try removing non-existent
        assert!(!store.remove_target("999").unwrap());
    }
    
    #[test]
    fn test_remove_primary_reassigns() {
        let temp_dir = TempDir::new().unwrap();
        let file_path = temp_dir.path().join("settings.json");
        
        let mut store = SettingsStore::new_with_path(&file_path);
        store.add_target(create_test_target("1")).unwrap(); // Will be primary
        store.add_target(create_test_target("2")).unwrap();
        
        assert!(store.get_target("1").unwrap().is_primary);
        assert!(!store.get_target("2").unwrap().is_primary);
        
        // Remove primary
        store.remove_target("1").unwrap();
        
        // Second should now be primary
        assert!(store.get_target("2").unwrap().is_primary);
    }
    
    #[test]
    fn test_update_target() {
        let temp_dir = TempDir::new().unwrap();
        let file_path = temp_dir.path().join("settings.json");
        
        let mut store = SettingsStore::new_with_path(&file_path);
        store.add_target(create_test_target("1")).unwrap();
        
        // Update
        let mut updated = create_test_target("1");
        updated.label = "Updated Label".to_string();
        assert!(store.update_target(updated).unwrap());
        
        assert_eq!(store.get_target("1").unwrap().label, "Updated Label");
        
        // Try updating non-existent
        assert!(!store.update_target(create_test_target("999")).unwrap());
    }
    
    // Edge case tests
    
    #[test]
    fn test_empty_file_path() {
        // Empty path should create defaults since it's treated as non-existent file
        let result = SettingsStore::load_from_path("");
        assert!(result.is_ok());
        let store = result.unwrap();
        assert_eq!(store.settings().version, 1);
        assert!(store.get_targets().is_empty());
    }
    
    #[test]
    fn test_corrupt_json() {
        let temp_dir = TempDir::new().unwrap();
        let file_path = temp_dir.path().join("corrupt.json");
        
        // Write invalid JSON
        fs::write(&file_path, "{ invalid json }").unwrap();
        
        let result = SettingsStore::load_from_path(&file_path);
        assert!(result.is_err());
        let err_msg = result.err().unwrap().to_string();
        assert!(err_msg.contains("Failed to parse"), "Error was: {}", err_msg);
    }
    
    #[test]
    fn test_invalid_directory() {
        // Try to save to a path that can't exist
        let store = SettingsStore::new_with_path("/nonexistent/deep/path/settings.json");
        let result = store.save();
        assert!(result.is_err());
        let err_msg = result.err().unwrap().to_string();
        assert!(err_msg.contains("Failed to create directory") || err_msg.contains("Failed to write"));
    }
    
    #[test]
    fn test_concurrent_access() {
        use std::thread;
        use std::sync::Arc;
        
        let temp_dir = Arc::new(TempDir::new().unwrap());
        let file_path = temp_dir.path().join("concurrent.json");
        
        // Create initial file
        let store = SettingsStore::new_with_path(&file_path);
        store.save().unwrap();
        
        // Multiple threads trying to read
        let handles: Vec<_> = (0..10)
            .map(|_| {
                let path = file_path.clone();
                thread::spawn(move || {
                    SettingsStore::load_from_path(path).unwrap()
                })
            })
            .collect();
        
        for handle in handles {
            let loaded = handle.join().unwrap();
            assert_eq!(loaded.settings().version, 1);
        }
    }
    
    #[test]
    fn test_unicode_in_path() {
        let temp_dir = TempDir::new().unwrap();
        let file_path = temp_dir.path().join("设置-🦀.json");
        
        let mut store = SettingsStore::new_with_path(&file_path);
        store.add_target(create_test_target("unicode")).unwrap();
        
        let loaded = SettingsStore::load_from_path(&file_path).unwrap();
        assert_eq!(loaded.get_targets().len(), 1);
    }
    
    #[test]
    fn test_very_large_settings() {
        let temp_dir = TempDir::new().unwrap();
        let file_path = temp_dir.path().join("large.json");
        
        let mut store = SettingsStore::new_with_path(&file_path);
        
        // Add many targets
        for i in 0..1000 {
            let mut target = create_test_target(&i.to_string());
            target.notes = Some("x".repeat(1000)); // Large notes
            store.settings_mut().targets.push(target);
        }
        
        // Save and load
        store.save().unwrap();
        let loaded = SettingsStore::load_from_path(&file_path).unwrap();
        assert_eq!(loaded.get_targets().len(), 1000);
    }
    
    #[test]
    fn test_atomic_save() {
        let temp_dir = TempDir::new().unwrap();
        let file_path = temp_dir.path().join("atomic.json");
        
        // Create initial file
        let mut store = SettingsStore::new_with_path(&file_path);
        store.add_target(create_test_target("1")).unwrap();
        
        // Verify temp file is cleaned up
        let temp_path = file_path.with_extension("json.tmp");
        assert!(!temp_path.exists());
        assert!(file_path.exists());
    }
    
    #[test]
    fn test_settings_migration_compatibility() {
        let temp_dir = TempDir::new().unwrap();
        let file_path = temp_dir.path().join("old_version.json");
        
        // Simulate old version with missing fields
        let old_json = r#"{
            "version": 1,
            "app_settings": {
                "autostart": true
            },
            "keybinds": {
                "join_primary": "Cmd+J",
                "hangup": "Cmd+H"
            },
            "targets": []
        }"#;
        
        fs::write(&file_path, old_json).unwrap();
        
        // Should load with defaults for missing fields
        let store = SettingsStore::load_from_path(&file_path).unwrap();
        assert!(store.settings().app_settings.always_on_top); // Should have default
    }
}
