/// Call state machine for managing call lifecycle
/// What: Defines all possible states and valid transitions
/// Why: Prevents invalid states and enforces proper call flow
/// Used by:
///   - CallController to manage state transitions
///   - UI to show appropriate interface
///   - Hotkey handlers to validate actions
/// Change notes: Adding states requires updating transition logic

/// Represents the current state of a call
/// What: All possible states in the call lifecycle
/// Why: Clear state management prevents race conditions
/// Used by: CallController, UI state management
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CallState {
    /// No active call, ready to start
    Idle,
    /// Attempting to join a call
    Connecting,
    /// Successfully joined and in call
    InCall,
    /// Leaving the call
    Disconnecting,
}

impl CallState {
    /// Check if transition to next state is valid
    /// What: Validates state machine transitions
    /// Why: Enforces proper flow and prevents illegal states
    /// Used by: CallController::transition_to()
    /// Calls: None
    /// Events: None
    /// Change notes: Update when adding new states
    pub fn can_transition_to(&self, next: CallState) -> bool {
        use CallState::*;
        
        match (*self, next) {
            // From Idle
            (Idle, Connecting) => true,
            
            // From Connecting
            (Connecting, InCall) => true,         // Successfully connected
            (Connecting, Disconnecting) => true,   // User cancelled or error
            
            // From InCall
            (InCall, Disconnecting) => true,       // User hangs up
            
            // From Disconnecting
            (Disconnecting, Idle) => true,          // Clean disconnect
            
            // All other transitions are invalid
            _ => false,
        }
    }
    
    /// Check if we're busy (not idle)
    /// What: Simple helper to check if in any active state
    /// Why: UI and hotkeys need to know if call is active
    /// Used by: Hotkey handlers, UI state checks
    pub fn is_busy(&self) -> bool {
        !matches!(self, CallState::Idle)
    }
    
    /// Get human-readable description
    /// What: User-friendly state names
    /// Why: For logging and debug output
    /// Used by: Error messages, logs
    pub fn description(&self) -> &'static str {
        match self {
            CallState::Idle => "ready",
            CallState::Connecting => "connecting",
            CallState::InCall => "in call",
            CallState::Disconnecting => "disconnecting",
        }
    }
}

impl std::fmt::Display for CallState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.description())
    }
}

impl Default for CallState {
    fn default() -> Self {
        CallState::Idle
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_valid_transitions() {
        // Idle transitions
        assert!(CallState::Idle.can_transition_to(CallState::Connecting));
        assert!(!CallState::Idle.can_transition_to(CallState::InCall));
        assert!(!CallState::Idle.can_transition_to(CallState::Disconnecting));
        assert!(!CallState::Idle.can_transition_to(CallState::Idle));
        
        // Connecting transitions
        assert!(!CallState::Connecting.can_transition_to(CallState::Idle));
        assert!(!CallState::Connecting.can_transition_to(CallState::Connecting));
        assert!(CallState::Connecting.can_transition_to(CallState::InCall));
        assert!(CallState::Connecting.can_transition_to(CallState::Disconnecting));
        
        // InCall transitions
        assert!(!CallState::InCall.can_transition_to(CallState::Idle));
        assert!(!CallState::InCall.can_transition_to(CallState::Connecting));
        assert!(!CallState::InCall.can_transition_to(CallState::InCall));
        assert!(CallState::InCall.can_transition_to(CallState::Disconnecting));
        
        // Disconnecting transitions
        assert!(CallState::Disconnecting.can_transition_to(CallState::Idle));
        assert!(!CallState::Disconnecting.can_transition_to(CallState::Connecting));
        assert!(!CallState::Disconnecting.can_transition_to(CallState::InCall));
        assert!(!CallState::Disconnecting.can_transition_to(CallState::Disconnecting));
    }
    
    #[test]
    fn test_state_machine_flow() {
        let mut state = CallState::Idle;
        
        // Happy path: Idle -> Connecting -> InCall -> Disconnecting -> Idle
        assert!(state.can_transition_to(CallState::Connecting));
        state = CallState::Connecting;
        
        assert!(state.can_transition_to(CallState::InCall));
        state = CallState::InCall;
        
        assert!(state.can_transition_to(CallState::Disconnecting));
        state = CallState::Disconnecting;
        
        assert!(state.can_transition_to(CallState::Idle));
        state = CallState::Idle;
        
        // Cancel path: Idle -> Connecting -> Disconnecting -> Idle
        assert!(state.can_transition_to(CallState::Connecting));
        state = CallState::Connecting;
        
        assert!(state.can_transition_to(CallState::Disconnecting));
        state = CallState::Disconnecting;
        
        assert!(state.can_transition_to(CallState::Idle));
    }
    
    #[test]
    fn test_is_busy() {
        assert!(!CallState::Idle.is_busy());
        assert!(CallState::Connecting.is_busy());
        assert!(CallState::InCall.is_busy());
        assert!(CallState::Disconnecting.is_busy());
    }
    
    #[test]
    fn test_descriptions() {
        assert_eq!(CallState::Idle.description(), "ready");
        assert_eq!(CallState::Connecting.description(), "connecting");
        assert_eq!(CallState::InCall.description(), "in call");
        assert_eq!(CallState::Disconnecting.description(), "disconnecting");
    }
    
    #[test]
    fn test_display() {
        assert_eq!(format!("{}", CallState::Idle), "ready");
        assert_eq!(format!("{}", CallState::InCall), "in call");
    }
    
    #[test]
    fn test_default() {
        assert_eq!(CallState::default(), CallState::Idle);
    }
    
    // Edge case tests
    
    #[test]
    fn test_no_self_transitions() {
        // Verify no state can transition to itself
        for state in [CallState::Idle, CallState::Connecting, CallState::InCall, CallState::Disconnecting] {
            assert!(!state.can_transition_to(state), "{:?} should not transition to itself", state);
        }
    }
    
    #[test]
    fn test_thread_safety() {
        // CallState is Copy, so it's inherently thread-safe
        let state = CallState::InCall;
        
        // This would not compile if CallState wasn't Send + Sync
        std::thread::spawn(move || {
            assert_eq!(state, CallState::InCall);
        }).join().unwrap();
    }
    
    #[test]
    fn test_exhaustive_state_coverage() {
        // This test ensures we handle all states
        let states = [
            CallState::Idle,
            CallState::Connecting,
            CallState::InCall,
            CallState::Disconnecting,
        ];
        
        // Check every state has at least one valid transition
        for state in &states {
            let mut has_valid_transition = false;
            for next in &states {
                if state.can_transition_to(*next) {
                    has_valid_transition = true;
                    break;
                }
            }
            
            // Special case: Idle doesn't need outgoing transitions in disconnected state
            if *state != CallState::Idle || !has_valid_transition {
                assert!(has_valid_transition || *state == CallState::Idle, 
                    "{:?} should have at least one valid transition", state);
            }
        }
    }
    
    #[test]
    fn test_no_backwards_transitions() {
        // Verify we can't go backwards in the flow (except Disconnecting -> Idle)
        assert!(!CallState::Connecting.can_transition_to(CallState::Idle));
        assert!(!CallState::InCall.can_transition_to(CallState::Idle));
        assert!(!CallState::InCall.can_transition_to(CallState::Connecting));
        assert!(!CallState::Disconnecting.can_transition_to(CallState::Connecting));
        assert!(!CallState::Disconnecting.can_transition_to(CallState::InCall));
        
        // Only allowed backwards transition
        assert!(CallState::Disconnecting.can_transition_to(CallState::Idle));
    }
    
    #[test]
    fn test_state_equality() {
        assert_eq!(CallState::Idle, CallState::Idle);
        assert_ne!(CallState::Idle, CallState::Connecting);
        
        // Test derive(Eq) works correctly
        let state1 = CallState::InCall;
        let state2 = CallState::InCall;
        assert!(state1 == state2);
    }
    
    #[test]
    fn test_debug_output() {
        // Ensure Debug trait provides useful output
        let state = CallState::Connecting;
        let debug_str = format!("{:?}", state);
        assert!(debug_str.contains("Connecting"));
    }
}
