/// Call Controller
/// What: Manages call state and orchestrates call lifecycle
/// Why: Centralizes call logic and enforces state transitions
/// Used by: Hotkey handlers, tray menu actions
/// Calls: ConferenceWindow service, emits state events
/// Change notes: If adding new states, update can_transition logic

use crate::services::conference_window::{ConferenceWindow, ConferenceConfig};
use justcall::core::CallState;
use std::sync::Mutex;
use tauri::{AppHandle, Emitter};

pub struct CallController {
    /// Current call state
    state: Mutex<CallState>,
    
    /// ID of the target we're connected to (if any)
    current_target: Mutex<Option<String>>,
    
    /// Handle to emit events
    app_handle: AppHandle,
}

impl CallController {
    /// Create new call controller
    /// What: Initializes controller in Idle state
    /// Why: Starting point for all call flows
    /// Used by: App initialization
    pub fn new(app_handle: AppHandle) -> Self {
        Self {
            state: Mutex::new(CallState::Idle),
            current_target: Mutex::new(None),
            app_handle,
        }
    }
    
    /// Get current state
    /// What: Returns the current call state
    /// Why: For UI updates and testing
    /// Used by: Status displays, tests
    pub fn get_state(&self) -> CallState {
        *self.state.lock().unwrap()
    }
    
    /// Get current target
    /// What: Returns the ID of connected target
    /// Why: To show who we're connected to
    /// Used by: UI status, reconnect logic
    pub fn get_current_target(&self) -> Option<String> {
        self.current_target.lock().unwrap().clone()
    }
    
    /// Join a call with specific target
    /// What: Initiates a call to the given target
    /// Why: Main entry point for starting calls
    /// Contract:
    /// - target_id must exist in settings
    /// - Only works when Idle
    /// - Transitions to Connecting state
    /// Used by: Hotkey handlers, tray menu
    /// Calls: ConferenceWindow::open()
    /// Events: Emits "call-state-changed"
    /// Change notes: If adding pre-call checks, do it here
    pub fn join(&self, target_id: String, window: &mut ConferenceWindow, config: ConferenceConfig) -> Result<(), String> {
        let mut state = self.state.lock().unwrap();
        let current = *state;
        
        // Check if we can transition
        if !current.can_transition_to(CallState::Connecting) {
            log::warn!("Cannot join: current state is {:?}", current);
            return Err(format!("Cannot join while {}", current));
        }
        
        log::info!("Starting call to target: {}", target_id);
        
        // Update state
        *state = CallState::Connecting;
        self.emit_state_change(CallState::Connecting);
        
        // Store target
        *self.current_target.lock().unwrap() = Some(target_id);
        
        // Open conference window
        window.open(config)?;
        
        Ok(())
    }
    
    /// Hangup current call
    /// What: Ends the active call
    /// Why: Clean disconnection flow
    /// Contract:
    /// - Works in Connecting or InCall states
    /// - Transitions to Disconnecting then Idle
    /// Used by: Hotkey handlers, window close
    /// Calls: ConferenceWindow::close()
    /// Events: Emits "call-state-changed"
    pub fn hangup(&self, window: &mut ConferenceWindow) -> Result<(), String> {
        let mut state = self.state.lock().unwrap();
        let current = *state;
        
        // Check if there's anything to hang up
        if current == CallState::Idle {
            log::debug!("Already idle, nothing to hangup");
            return Ok(());
        }
        
        if !current.can_transition_to(CallState::Disconnecting) {
            log::warn!("Cannot hangup from state: {:?}", current);
            return Err(format!("Cannot hangup while {}", current));
        }
        
        log::info!("Hanging up call");
        
        // Update state
        *state = CallState::Disconnecting;
        self.emit_state_change(CallState::Disconnecting);
        
        // Close window
        window.close();
        
        // Clear target and go idle
        *self.current_target.lock().unwrap() = None;
        *state = CallState::Idle;
        self.emit_state_change(CallState::Idle);
        
        Ok(())
    }
    
    /// Handle conference joined event
    /// What: Updates state when Jitsi confirms we're in the call
    /// Why: Tracks actual connection status
    /// Used by: Conference window event handlers
    /// Events: Emits "call-state-changed"
    pub fn on_conference_joined(&self) {
        let mut state = self.state.lock().unwrap();
        
        if *state == CallState::Connecting {
            log::info!("Conference joined, transitioning to InCall");
            *state = CallState::InCall;
            self.emit_state_change(CallState::InCall);
        }
    }
    
    /// Handle conference left event
    /// What: Updates state when we leave the call
    /// Why: Ensures state matches reality
    /// Used by: Conference window event handlers
    pub fn on_conference_left(&self) {
        let mut state = self.state.lock().unwrap();
        
        if *state != CallState::Idle {
            log::info!("Conference left, going idle");
            *self.current_target.lock().unwrap() = None;
            *state = CallState::Idle;
            self.emit_state_change(CallState::Idle);
        }
    }
    
    /// Emit state change event
    /// What: Notifies app about state changes
    /// Why: For UI updates and logging
    /// Used by: All state transitions
    fn emit_state_change(&self, new_state: CallState) {
        let _ = self.app_handle.emit("call-state-changed", new_state);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_initial_state() {
        let app_handle = tauri::test::mock_app().handle();
        let controller = CallController::new(app_handle);
        assert_eq!(controller.get_state(), CallState::Idle);
        assert_eq!(controller.get_current_target(), None);
    }
    
    #[test]
    fn test_cannot_join_when_busy() {
        let app_handle = tauri::test::mock_app().handle();
        let controller = CallController::new(app_handle);
        
        // Manually set state to InCall
        *controller.state.lock().unwrap() = CallState::InCall;
        
        // Try to join
        let mut window = ConferenceWindow::new(tauri::test::mock_app().handle());
        let config = ConferenceConfig {
            room_id: "test-room".to_string(),
            display_name: "Test".to_string(),
            start_with_audio_muted: false,
            start_with_video_muted: false,
            always_on_top: false,
        };
        
        let result = controller.join("target1".to_string(), &mut window, config);
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("Cannot join while InCall"));
    }
}
