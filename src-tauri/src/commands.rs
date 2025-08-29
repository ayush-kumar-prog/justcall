// Tauri command handlers
// What: Bridge between frontend and backend for settings management
// Why: Provides secure API for frontend to interact with settings
// Used by: settings.js (frontend), lib.rs (backend)

use crate::state::AppState;
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
    let mut store = state.settings_store.lock().unwrap();
    
    // Deserialize the settings
    let new_settings: justcall::models::Settings = serde_json::from_value(settings)
        .map_err(|e| format!("Invalid settings format: {}", e))?;
    
    // Update the store
    *store.settings_mut() = new_settings;
    
    // Save to disk
    store.save()
        .map_err(|e| format!("Failed to save settings: {}", e))
}

#[tauri::command]
pub async fn generate_code() -> Result<String, String> {
    Ok(justcall::core::crypto::generate_code_base32_100b())
}
