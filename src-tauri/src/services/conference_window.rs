/// Conference window manager
/// What: Manages the video call window lifecycle
/// Why: Provides clean interface for window creation and management
/// Used by: CallController (phase 5), hotkey handlers
/// Calls: Tauri window API, emits window events
/// Change notes: Enforces single window instance, handles edge cases

use tauri::{WebviewUrl, WebviewWindowBuilder, Emitter, Listener, Manager};
use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
pub struct ConferenceConfig {
    pub room_id: String,
    pub display_name: String,
    pub start_with_audio_muted: bool,
    pub start_with_video_muted: bool,
    pub always_on_top: bool,
}

pub struct ConferenceWindow {
    /// The active conference window (if any)
    window: Option<tauri::WebviewWindow>,
    /// App handle for creating windows
    app_handle: tauri::AppHandle,
}

impl ConferenceWindow {
    /// Create new conference window manager
    /// What: Initializes the manager with app handle
    /// Why: Needs app handle to create Tauri windows
    /// Used by: App state initialization
    pub fn new(app_handle: tauri::AppHandle) -> Self {
        Self {
            window: None,
            app_handle,
        }
    }
    
    /// Open conference window
    /// What: Creates or shows the video call window
    /// Why: Entry point for starting a video call
    /// Contract:
    /// - config: Room and display settings
    /// - Reuses existing window if open
    /// - Window is centered, 1024x768 default
    /// - Returns error if window creation fails
    /// Used by: CallController::join() (phase 5)
    /// Calls: Tauri WebviewWindowBuilder
    /// Events: Emits "conference-window-ready" after creation
    /// Change notes: If changing window size, update conference.html responsive CSS
    pub fn open(&mut self, config: ConferenceConfig) -> Result<(), String> {
        log::info!("Opening conference window for room: {}", config.room_id);
        
        // Check if window already exists in Tauri's window manager
        let window_label = "conference";
        if let Some(existing) = self.app_handle.get_webview_window(window_label) {
            log::info!("Conference window already exists, focusing");
            let _ = existing.show();
            let _ = existing.set_focus();
            
            // Update room config for existing window
            existing.emit("start-call", &config)
                .map_err(|e| format!("Failed to emit to existing window: {}", e))?;
            
            // Update our reference
            self.window = Some(existing);
            return Ok(());
        }
        
        // Create new window
        let window_label = "conference";
        
        // Encode config as URL parameter
        let config_json = serde_json::to_string(&config).unwrap_or_default();
        let encoded_config = urlencoding::encode(&config_json);
        let url = format!("conference.html?config={}", encoded_config);
        
        let window = WebviewWindowBuilder::new(
            &self.app_handle,
            window_label,
            WebviewUrl::App(url.into())
        )
        .title("JustCall")
        .inner_size(1024.0, 768.0)
        .min_inner_size(640.0, 480.0)
        .resizable(true)
        .always_on_top(config.always_on_top)
        .fullscreen(false)
        .skip_taskbar(false)  // Show in taskbar/dock
        .decorations(true)    // Native window chrome
        .visible(false)       // Start hidden, show after load
        .initialization_script(
            r#"
            console.log('Conference window initialization script running...');
            // Ensure Tauri API is available
            if (!window.__TAURI__ && window.__TAURI_INTERNALS__) {
                window.__TAURI__ = window.__TAURI_INTERNALS__;
            }
            console.log('Init script - window.__TAURI__:', window.__TAURI__);
            "#
        )
        .build()
        .map_err(|e| format!("Failed to create window: {}", e))?;
        
        // Clone for event handlers
        let window_clone = window.clone();
        let config_clone = config.clone();
        let app_handle = self.app_handle.clone();
        
        // Clone for different approach
        let window_clone2 = window.clone();
        let config_json = serde_json::to_string(&config).unwrap_or_default();
        
        // Wait for DOM ready before showing and emitting config
        window.once("dom-ready", move |event| {
            log::info!("Conference window DOM ready event received: {:?}", event);
            log::info!("Emitting start-call with config: {:?}", &config_clone);
            
            // Try the original emit approach
            match window_clone.emit("start-call", &config_clone) {
                Ok(_) => log::info!("Successfully emitted start-call event"),
                Err(e) => {
                    log::error!("Failed to emit start-call: {}", e);
                    
                    // Fallback: Try using eval to inject the config directly
                    let js_code = format!(
                        r#"
                        console.log('Injecting config via eval');
                        if (window.bridge && window.bridge.createMeeting) {{
                            const config = {};
                            console.log('Creating meeting with injected config:', config);
                            window.bridge.createMeeting(config);
                        }} else {{
                            console.error('Bridge not ready, storing config for later');
                            window.__injectedConfig = {};
                        }}
                        "#,
                        config_json, config_json
                    );
                    
                    if let Err(e) = window_clone2.eval(&js_code) {
                        log::error!("Failed to inject config via eval: {}", e);
                    }
                }
            }
            
            // Show window after config sent
            let _ = window_clone.show();
            let _ = window_clone.set_focus();
            
            // Notify app that window is ready
            app_handle.emit("conference-window-ready", ())
                .unwrap_or_else(|e| log::error!("Failed to emit window ready: {}", e));
        });
        
        // Store window reference
        self.window = Some(window);
        
        log::info!("Conference window created successfully");
        Ok(())
    }
    
    /// Close conference window
    /// What: Closes and cleans up the video call window
    /// Why: Called when ending a call
    /// Used by: CallController::hangup() (phase 5)
    /// Calls: Window close API
    /// Events: Window emits "closed" event automatically
    pub fn close(&mut self) {
        log::info!("Closing conference window");
        
        if let Some(window) = self.window.take() {
            // Emit cleanup event first
            let _ = window.emit("end-call", ());
            
            // Small delay to allow cleanup
            std::thread::sleep(std::time::Duration::from_millis(100));
            
            // Close window
            if let Err(e) = window.close() {
                log::error!("Failed to close window: {}", e);
            }
        }
        
        self.window = None;
    }
    
    /// Check if conference window is open
    /// What: Returns true if window exists and is visible
    /// Why: Prevents duplicate windows and helps state management
    /// Used by: CallController state checks
    pub fn is_open(&self) -> bool {
        if let Some(window) = &self.window {
            window.is_visible().unwrap_or(false)
        } else {
            false
        }
    }
    
    /// Send command to conference window
    /// What: Sends commands to the webview (mute, toggle video, etc)
    /// Why: Allows backend to control call features
    /// Used by: Future in-call hotkeys, tray menu actions
    /// Events: Emits custom events to webview
    pub fn send_command(&self, command: &str, payload: serde_json::Value) -> Result<(), String> {
        if let Some(window) = &self.window {
            window.emit(command, payload)
                .map_err(|e| format!("Failed to send command: {}", e))
        } else {
            Err("No active conference window".to_string())
        }
    }
    
    /// Get window handle (for testing/debugging)
    /// What: Returns reference to underlying Tauri window
    /// Why: Allows direct window manipulation if needed
    /// Used by: Tests, debug commands
    pub fn window(&self) -> Option<&tauri::WebviewWindow> {
        self.window.as_ref()
    }
}

impl Drop for ConferenceWindow {
    /// Cleanup on drop
    /// What: Ensures window is closed when manager is dropped
    /// Why: Prevents orphaned windows
    fn drop(&mut self) {
        log::debug!("ConferenceWindow dropping, cleaning up");
        self.close();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_conference_config_serialization() {
        let config = ConferenceConfig {
            room_id: "jc-test123".to_string(),
            display_name: "Test User".to_string(),
            start_with_audio_muted: false,
            start_with_video_muted: true,
            always_on_top: true,
        };
        
        let json = serde_json::to_string(&config).unwrap();
        assert!(json.contains("jc-test123"));
        assert!(json.contains("Test User"));
    }
}
