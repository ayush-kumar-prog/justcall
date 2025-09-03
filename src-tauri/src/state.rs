// Application state management
// What: Shared state for Tauri app
// Why: Provides thread-safe access to settings and other stateful components
// Used by: commands.rs, lib.rs

use blink::storage::SettingsStore;
use crate::services::global_shortcuts::GlobalShortcutService;
use crate::services::conference_window::ConferenceWindow;
use crate::controllers::call_controller::CallController;
use std::sync::Mutex;

/// Application state
/// What: Central state management for the app
/// Why: Provides thread-safe access to services and stores
/// Used by: Commands, event handlers, app setup
pub struct AppState {
    pub settings_store: Mutex<SettingsStore>,
    pub shortcuts: Mutex<GlobalShortcutService>,
    pub conference_window: Mutex<ConferenceWindow>,
    pub call_controller: Mutex<CallController>,
}
