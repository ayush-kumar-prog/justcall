// Global shortcut service
// What: Manages system-wide keyboard shortcuts for the application
// Why: Provides a clean interface for registering and handling hotkeys
// Used by: lib.rs (app setup), commands.rs (settings updates)
// Calls: tauri-plugin-global-shortcut API
// Events: Emits "hotkey-pressed" events
// Change notes: Uses Tauri v2 global shortcut plugin

use std::collections::HashMap;
use tauri::{AppHandle, Emitter};
use tauri_plugin_global_shortcut::{GlobalShortcutExt, Shortcut, ShortcutState};
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ShortcutAction {
    JoinPrimary,
    JoinTarget { id: String },
    Hangup,
}

pub struct GlobalShortcutService {
    // Maps hotkey string to action
    shortcuts: HashMap<String, ShortcutAction>,
    app_handle: AppHandle,
}

impl GlobalShortcutService {
    /// Create new global shortcut service
    /// What: Initializes the service with app handle
    /// Why: Needs app handle to register shortcuts and emit events
    /// Used by: App setup in lib.rs
    pub fn new(app_handle: AppHandle) -> Self {
        Self {
            shortcuts: HashMap::new(),
            app_handle,
        }
    }
    
    /// Register a global hotkey
    /// What: Registers a system-wide keyboard shortcut
    /// Why: Enables users to trigger actions from any application
    /// Contract:
    /// - hotkey: Format like "Cmd+Opt+J" or "Ctrl+Alt+H"
    /// - action: What to do when hotkey is pressed
    /// - Returns error if hotkey is invalid or conflicts
    /// Used by: setup_default_hotkeys(), update_hotkeys command
    /// Calls: tauri-plugin-global-shortcut register API
    /// Change notes: Updated for Tauri v2 plugin API
    pub fn register_hotkey(&mut self, hotkey: &str, action: ShortcutAction) -> Result<(), String> {
        log::info!("Registering hotkey: {} -> {:?}", hotkey, action);
        
        // Parse the shortcut string
        let shortcut = hotkey.parse::<Shortcut>()
            .map_err(|e| format!("Invalid hotkey format '{}': {}", hotkey, e))?;
        
        // Check if already registered
        if self.shortcuts.contains_key(hotkey) {
            log::warn!("Hotkey {} already registered, updating action", hotkey);
            self.unregister_hotkey(hotkey)?;
        }
        
        // Clone values for the closure
        let app_handle = self.app_handle.clone();
        let action_clone = action.clone();
        let hotkey_str = hotkey.to_string();
        
        // Register with plugin
        self.app_handle.global_shortcut()
            .on_shortcut(shortcut, move |app_handle, _shortcut, event| {
                if event.state == ShortcutState::Pressed {
                    log::info!("Hotkey pressed: {}", hotkey_str);
                    
                    // Emit event to frontend/backend
                    let _ = app_handle.emit("hotkey-pressed", &action_clone);
                    
                    // Also handle directly for now (will be moved to controller later)
                    match &action_clone {
                        ShortcutAction::JoinPrimary => {
                            log::info!("Join primary target requested");
                        }
                        ShortcutAction::JoinTarget { id } => {
                            log::info!("Join target {} requested", id);
                        }
                        ShortcutAction::Hangup => {
                            log::info!("Hangup requested");
                        }
                    }
                }
            })
            .map_err(|e| format!("Failed to register hotkey: {}", e))?;
        
        self.shortcuts.insert(hotkey.to_string(), action);
        log::info!("Successfully registered hotkey: {}", hotkey);
        Ok(())
    }
    
    /// Unregister a hotkey
    /// What: Removes a previously registered hotkey
    /// Why: Needed when updating hotkeys or cleaning up
    /// Used by: register_hotkey (for updates), drop impl
    /// Calls: tauri-plugin-global-shortcut unregister API
    pub fn unregister_hotkey(&mut self, hotkey: &str) -> Result<(), String> {
        log::info!("Unregistering hotkey: {}", hotkey);
        
        // Parse the shortcut string
        let shortcut = hotkey.parse::<Shortcut>()
            .map_err(|e| format!("Invalid hotkey format: {}", e))?;
        
        self.app_handle.global_shortcut()
            .unregister(shortcut)
            .map_err(|e| format!("Failed to unregister hotkey: {}", e))?;
        
        self.shortcuts.remove(hotkey);
        Ok(())
    }
    
    /// Unregister all hotkeys
    /// What: Removes all registered hotkeys
    /// Why: Cleanup on shutdown or when resetting all hotkeys
    /// Used by: drop impl, reset command
    /// Calls: unregister for each hotkey
    pub fn unregister_all(&mut self) -> Result<(), String> {
        log::info!("Unregistering all hotkeys");
        
        let hotkeys: Vec<String> = self.shortcuts.keys().cloned().collect();
        for hotkey in hotkeys {
            if let Err(e) = self.unregister_hotkey(&hotkey) {
                log::error!("Failed to unregister {}: {}", hotkey, e);
            }
        }
        
        self.shortcuts.clear();
        Ok(())
    }
    
    /// Setup default hotkeys from settings
    /// What: Registers the default join/hangup hotkeys
    /// Why: Called on app startup to enable hotkeys
    /// Used by: App setup after loading settings
    /// Calls: register_hotkey
    pub fn setup_default_hotkeys(&mut self, keybinds: &justcall::models::settings::Keybinds) -> Result<(), String> {
        log::info!("Setting up default hotkeys");
        
        // Register join primary
        if !keybinds.join_primary.is_empty() {
            self.register_hotkey(&keybinds.join_primary, ShortcutAction::JoinPrimary)?;
        }
        
        // Register hangup
        if !keybinds.hangup.is_empty() {
            self.register_hotkey(&keybinds.hangup, ShortcutAction::Hangup)?;
        }
        
        Ok(())
    }
    
    /// Check if a hotkey is already registered
    /// What: Checks if a hotkey string is in use
    /// Why: Prevents conflicts when adding new hotkeys
    /// Used by: Settings UI validation
    pub fn is_registered(&self, hotkey: &str) -> bool {
        self.shortcuts.contains_key(hotkey)
    }
    
    /// Get all registered hotkeys
    /// What: Returns map of hotkey -> action
    /// Why: Settings UI needs to show current hotkeys
    /// Used by: get_hotkeys command
    pub fn get_registered_hotkeys(&self) -> &HashMap<String, ShortcutAction> {
        &self.shortcuts
    }
}

impl Drop for GlobalShortcutService {
    /// Cleanup on drop
    /// What: Unregisters all hotkeys when service is dropped
    /// Why: Prevents hotkeys from remaining active after app closes
    fn drop(&mut self) {
        log::info!("GlobalShortcutService dropping, cleaning up hotkeys");
        let _ = self.unregister_all();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_shortcut_action_serialization() {
        let action = ShortcutAction::JoinTarget { id: "test-123".to_string() };
        let json = serde_json::to_string(&action).unwrap();
        let parsed: ShortcutAction = serde_json::from_str(&json).unwrap();
        
        match parsed {
            ShortcutAction::JoinTarget { id } => assert_eq!(id, "test-123"),
            _ => panic!("Wrong action type"),
        }
    }
}