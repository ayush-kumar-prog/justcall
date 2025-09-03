// Tauri command handlers
// What: Bridge between frontend and backend for settings management
// Why: Provides secure API for frontend to interact with settings
// Used by: settings.js (frontend), lib.rs (backend)

use crate::state::AppState;
use crate::services::global_shortcuts::ShortcutAction;
use serde_json::Value;
use tauri::State;

#[tauri::command]
pub async fn get_settings(state: State<'_, AppState>) -> Result<Value, String> {
    let store = state.settings_store.lock().unwrap();
    let settings = store.settings();
    
    serde_json::to_value(settings)
        .map_err(|e| format!("Failed to serialize settings: {}", e))
}

#[tauri::command]
pub async fn save_settings(
    settings: Value,
    state: State<'_, AppState>,
) -> Result<(), String> {
    // First, update hotkeys if they changed
    let old_keybinds = {
        let store = state.settings_store.lock().unwrap();
        store.settings().keybinds.clone()
    };
    
    // Deserialize the new settings
    let new_settings: blink::models::Settings = serde_json::from_value(settings)
        .map_err(|e| format!("Invalid settings format: {}", e))?;
    
    // Update hotkeys if changed
    if old_keybinds != new_settings.keybinds {
        log::info!("Hotkeys changed, updating global shortcuts");
        
        let mut shortcuts = state.shortcuts.lock().unwrap();
        
        // Unregister old hotkeys
        if let Err(e) = shortcuts.unregister_all() {
            log::error!("Failed to unregister old hotkeys: {}", e);
        }
        
        // Register new hotkeys
        if let Err(e) = shortcuts.setup_default_hotkeys(&new_settings.keybinds) {
            log::error!("Failed to setup new hotkeys: {}", e);
            // Continue anyway - settings should still be saved
        }
    }
    
    // Update the store
    let mut store = state.settings_store.lock().unwrap();
    *store.settings_mut() = new_settings;
    
    // Save to disk
    store.save()
        .map_err(|e| format!("Failed to save settings: {}", e))
}

#[tauri::command]
pub async fn generate_code() -> Result<String, String> {
    Ok(blink::core::crypto::generate_code_base32_100b())
}

#[tauri::command]
pub async fn validate_hotkey(hotkey: String, state: State<'_, AppState>) -> Result<bool, String> {
    // Check if hotkey is already in use
    let shortcuts = state.shortcuts.lock().unwrap();
    Ok(!shortcuts.is_registered(&hotkey))
}

#[tauri::command]
pub async fn test_hotkey(hotkey: String, state: State<'_, AppState>) -> Result<(), String> {
    // Temporarily register a hotkey to test if it works
    let mut shortcuts = state.shortcuts.lock().unwrap();
    
    // Try to register
    shortcuts.register_hotkey(&hotkey, ShortcutAction::JoinPrimary)?;
    
    // Immediately unregister
    shortcuts.unregister_hotkey(&hotkey)?;
    
    Ok(())
}

#[tauri::command]
pub async fn remove_target(id: String, state: State<'_, AppState>) -> Result<bool, String> {
    let mut store = state.settings_store.lock().unwrap();
    
    store.remove_target(&id)
        .map_err(|e| format!("Failed to remove target: {}", e))
}
