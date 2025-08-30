// Application state management
// What: Shared state for Tauri app
// Why: Provides thread-safe access to settings and other stateful components
// Used by: commands.rs, lib.rs

use justcall::storage::SettingsStore;
use crate::services::global_shortcuts::GlobalShortcutService;
use std::sync::Mutex;

pub struct AppState {
    pub settings_store: Mutex<SettingsStore>,
    pub shortcuts: Mutex<GlobalShortcutService>,
}
