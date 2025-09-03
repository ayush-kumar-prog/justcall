/// Call Controller (Simplified)
/// What: Manages call lifecycle without complex state machine
/// Why: Provides clean separation between hotkeys and window management
/// Used by: Hotkey handlers in lib.rs
/// Note: This is a simplified version after the full controller was deleted

use crate::services::conference_window::{ConferenceWindow, ConferenceConfig};
use blink::core::CallState;
use std::sync::Mutex;
use tauri::AppHandle;

pub struct CallController {
    /// Current call state
    state: Mutex<CallState>,
    
    /// Handle to emit events
    app_handle: AppHandle,
}

impl CallController {
    /// Create new call controller
    pub fn new(app_handle: AppHandle) -> Self {
        Self {
            state: Mutex::new(CallState::Idle),
            app_handle,
        }
    }
    
    /// Join a call - simplified version
    /// Just opens the window without complex state checks
    pub fn join(&self, _target_id: String, window: &mut ConferenceWindow, config: ConferenceConfig) -> Result<(), String> {
        // For now, just open the window
        window.open(config)
    }
    
    /// Hangup - simplified version
    pub fn hangup(&self, window: &mut ConferenceWindow) -> Result<(), String> {
        window.close();
        Ok(())
    }
    
    /// Handle conference joined event
    pub fn on_conference_joined(&self) {
        // Simplified - just log
        log::info!("Conference joined");
    }
    
    /// Handle conference left event
    pub fn on_conference_left(&self) {
        // Simplified - just log
        log::info!("Conference left");
    }
}