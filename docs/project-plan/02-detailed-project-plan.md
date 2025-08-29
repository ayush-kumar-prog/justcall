# JustCall Detailed Project Plan

## Overview
Building a cross-platform instant video calling app with global hotkeys. Press a hotkey → instantly in a call with your configured partner/group. No sign-in, no friction.

---

## Phase 1: Core Utilities (Pure Functions)

### Step 1.1: Code Generation Module
**Goal**: Create a function that generates secure, random codes for pairing

**What to build**:
```rust
// File: src/core/crypto.rs
pub fn generate_code_base32_100b() -> String {
    // Generate 16 random bytes (128 bits)
    // Encode to base32
    // Take first 20 characters (100 bits)
    // Format as: "xxxx-xxxx-xxxx-xxxx-xxxx"
}
```

**How to implement**:
1. Create project structure: `cargo new --name justcall .` (in project root)
2. Add dependencies to `Cargo.toml`:
   ```toml
   [dependencies]
   rand = "0.8"
   data-encoding = "2.5"
   ```
3. Create `src/core/mod.rs` and `src/core/crypto.rs`
4. Implement the function using `rand::rngs::OsRng` for cryptographic randomness

**How to test**:
```rust
// File: src/core/crypto.rs (at bottom)
#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashSet;
    
    #[test]
    fn test_code_uniqueness() {
        let mut codes = HashSet::new();
        for _ in 0..1000 {
            let code = generate_code_base32_100b();
            assert!(codes.insert(code), "Duplicate code generated!");
        }
    }
    
    #[test]
    fn test_code_format() {
        let code = generate_code_base32_100b();
        assert_eq!(code.len(), 24); // 20 chars + 4 hyphens
        assert!(code.chars().all(|c| c.is_alphanumeric() || c == '-'));
    }
}
```

**Run test**: `cargo test core::crypto`

**Success criteria**:
- Generates different codes each time
- Codes are exactly 24 characters (with hyphens)
- Only uses base32 alphabet (a-z, 2-7)
- No duplicates in 1000 generations

---

### Step 1.2: Room Derivation Module
**Goal**: Convert a pairing code into a deterministic Jitsi room name

**What to build**:
```rust
// File: src/core/room.rs
pub fn room_id_from_code(code: &str) -> String {
    // Remove hyphens from code
    // Hash with SHA256: "justcall-v1|" + code
    // Encode hash to base32
    // Return: "jc-" + first 16 chars
}
```

**How to implement**:
1. Add to `Cargo.toml`:
   ```toml
   sha2 = "0.10"
   ```
2. Create `src/core/room.rs`
3. Use SHA256 with domain separation ("justcall-v1|" prefix)

**How to test**:
```rust
#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_deterministic_room_id() {
        let code = "abcd-efgh-ijkl-mnop-qrst";
        let room1 = room_id_from_code(code);
        let room2 = room_id_from_code(code);
        assert_eq!(room1, room2, "Same code should produce same room");
    }
    
    #[test]
    fn test_different_codes_different_rooms() {
        let room1 = room_id_from_code("aaaa-aaaa-aaaa-aaaa-aaaa");
        let room2 = room_id_from_code("bbbb-bbbb-bbbb-bbbb-bbbb");
        assert_ne!(room1, room2, "Different codes should produce different rooms");
    }
    
    #[test]
    fn test_room_format() {
        let room = room_id_from_code("test-test-test-test-test");
        assert!(room.starts_with("jc-"));
        assert_eq!(room.len(), 19); // "jc-" + 16 chars
    }
}
```

**Manual testing**:
1. Create a simple main.rs that generates a code and derives a room
2. Run multiple times, verify same code → same room
3. Try edge cases: empty string, very long string, special characters

---

### Step 1.3: Platform Detection
**Goal**: Detect OS and return appropriate default hotkeys

**What to build**:
```rust
// File: src/core/platform.rs
#[derive(Debug, Clone)]
pub struct KeybindDefaults {
    pub join_primary: String,
    pub hangup: String,
    pub join_target_prefix: String, // e.g., "Cmd+Opt+" on macOS
}

pub fn get_default_keybinds() -> KeybindDefaults {
    // Detect OS using cfg!(target_os = "...")
    // Return appropriate defaults
}
```

**How to implement**:
1. Create `src/core/platform.rs`
2. Use Rust's built-in `cfg!` macro for OS detection
3. Return struct with platform-specific defaults:
   - macOS: Cmd+Opt+J (join), Cmd+Opt+H (hangup)
   - Windows/Linux: Ctrl+Alt+J, Ctrl+Alt+H

**How to test**:
```rust
#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_defaults_returned() {
        let defaults = get_default_keybinds();
        assert!(!defaults.join_primary.is_empty());
        assert!(!defaults.hangup.is_empty());
    }
    
    #[test]
    #[cfg(target_os = "macos")]
    fn test_macos_defaults() {
        let defaults = get_default_keybinds();
        assert_eq!(defaults.join_primary, "Cmd+Opt+J");
        assert_eq!(defaults.hangup, "Cmd+Opt+H");
    }
    
    #[test]
    #[cfg(target_os = "windows")]
    fn test_windows_defaults() {
        let defaults = get_default_keybinds();
        assert_eq!(defaults.join_primary, "Ctrl+Alt+J");
    }
}
```

**Cross-platform testing**:
1. Use `cargo test` on each OS
2. Or use: `cargo test --target x86_64-pc-windows-msvc` (requires cross-compilation setup)
3. Create a simple executable that prints the defaults, test on each platform

---

## Phase 2: Data Layer

### Step 2.1: Settings Schema
**Goal**: Define the data structures for app configuration

**What to build**:
```rust
// File: src/models/mod.rs and src/models/settings.rs
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Settings {
    pub version: u32,
    pub app_settings: AppSettings,
    pub keybinds: Keybinds,
    pub targets: Vec<Target>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Target {
    pub id: String,
    pub label: String,
    pub code: String,
    pub is_primary: bool,
    pub defaults: CallDefaults,
}
// ... more structs
```

**How to implement**:
1. Add to `Cargo.toml`:
   ```toml
   serde = { version = "1.0", features = ["derive"] }
   serde_json = "1.0"
   ```
2. Create comprehensive structs for all settings
3. Use serde attributes for clean JSON

**How to test**:
```rust
#[test]
fn test_settings_serialization() {
    let settings = Settings {
        version: 1,
        // ... fill in test data
    };
    
    // Serialize to JSON
    let json = serde_json::to_string_pretty(&settings).unwrap();
    println!("JSON output:\n{}", json);
    
    // Deserialize back
    let parsed: Settings = serde_json::from_str(&json).unwrap();
    assert_eq!(settings.version, parsed.version);
}

#[test]
fn test_backwards_compatibility() {
    // Test that old JSON formats still parse
    let old_json = r#"{"version": 1, "targets": []}"#;
    let result = serde_json::from_str::<Settings>(old_json);
    assert!(result.is_ok(), "Should handle missing fields");
}
```

**Manual validation**:
1. Create sample JSON files in `tests/fixtures/`
2. Write a script to validate they parse correctly
3. Test with malformed JSON to ensure good error messages

---

### Step 2.2: Settings Store
**Goal**: Load and save settings to disk persistently

**What to build**:
```rust
// File: src/storage/settings_store.rs
pub struct SettingsStore {
    settings: Settings,
    file_path: PathBuf,
}

impl SettingsStore {
    pub fn load() -> Result<Self> { }
    pub fn save(&self) -> Result<()> { }
    pub fn get_target(&self, id: &str) -> Option<&Target> { }
    pub fn add_target(&mut self, target: Target) -> Result<()> { }
}
```

**How to implement**:
1. Use `dirs` crate to find app data directory:
   ```toml
   dirs = "5.0"
   anyhow = "1.0"  # for error handling
   ```
2. Store in:
   - macOS: `~/Library/Application Support/JustCall/settings.json`
   - Windows: `%APPDATA%\JustCall\settings.json`
   - Linux: `~/.config/justcall/settings.json`
3. Create directory if it doesn't exist
4. Handle missing file (create defaults)

**How to test**:
```rust
#[test]
fn test_save_and_load() {
    use tempfile::TempDir;
    
    // Create temp directory for test
    let temp_dir = TempDir::new().unwrap();
    let file_path = temp_dir.path().join("settings.json");
    
    // Save settings
    let store = SettingsStore::new_with_path(file_path.clone());
    store.add_target(create_test_target());
    store.save().unwrap();
    
    // Load from disk
    let loaded = SettingsStore::load_from_path(file_path).unwrap();
    assert_eq!(loaded.targets.len(), 1);
}

#[test]
fn test_missing_file_creates_defaults() {
    let store = SettingsStore::load_from_path("/nonexistent/path").unwrap();
    assert_eq!(store.settings.version, 1);
    assert!(store.settings.targets.is_empty());
}
```

**Integration testing**:
1. Run app, add targets via CLI
2. Quit app, verify JSON file exists and is valid
3. Delete file, run app, verify defaults created
4. Corrupt JSON file, verify graceful error handling

---

### Step 2.3: State Machine
**Goal**: Define call states and valid transitions

**What to build**:
```rust
// File: src/core/call_state.rs
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum CallState {
    Idle,
    Connecting,
    InCall,
    Disconnecting,
}

impl CallState {
    pub fn can_transition_to(&self, next: CallState) -> bool {
        match (self, next) {
            (CallState::Idle, CallState::Connecting) => true,
            (CallState::Connecting, CallState::InCall) => true,
            // ... define all valid transitions
            _ => false,
        }
    }
}
```

**How to implement**:
1. Define the enum with all states
2. Create transition validation logic
3. Add helper methods like `is_busy()`

**How to test**:
```rust
#[test]
fn test_valid_transitions() {
    assert!(CallState::Idle.can_transition_to(CallState::Connecting));
    assert!(CallState::Connecting.can_transition_to(CallState::InCall));
    assert!(!CallState::Idle.can_transition_to(CallState::InCall)); // Invalid
}

#[test]
fn test_state_machine_flow() {
    let mut state = CallState::Idle;
    
    // Simulate full call flow
    assert!(state.can_transition_to(CallState::Connecting));
    state = CallState::Connecting;
    
    assert!(state.can_transition_to(CallState::InCall));
    state = CallState::InCall;
    
    assert!(state.can_transition_to(CallState::Disconnecting));
    state = CallState::Disconnecting;
    
    assert!(state.can_transition_to(CallState::Idle));
}
```

---

## Phase 3: Tauri Foundation

### Step 3.1: Minimal Tauri App
**Goal**: Create basic Tauri app with system tray

**Setup steps**:
1. Initialize Tauri in existing Cargo project:
   ```bash
   cd /Users/kumar/Documents/Projects/justcall/justcall
   npm create tauri-app@latest . -- --template vanilla
   ```
2. When prompted, select "Use current directory"
3. This creates `src-tauri/` folder with Tauri config

**What to build**:
1. Modify `src-tauri/tauri.conf.json`:
   ```json
   {
     "tauri": {
       "systemTray": {
         "iconPath": "icons/icon.png",
         "iconAsTemplate": true
       }
     }
   }
   ```
2. Add tray menu in `src-tauri/src/main.rs`
3. Hide main window on start

**How to test**:
1. Run: `npm run tauri dev`
2. Look for tray icon in system tray/menu bar
3. Right-click icon, see menu with "Quit"
4. Click Quit, app should close
5. No main window should appear

**Verification checklist**:
- [ ] Tray icon visible
- [ ] Menu appears on click
- [ ] Quit works
- [ ] No window on launch
- [ ] Icon looks good on your OS

---

### Step 3.2: Settings Window
**Goal**: Create a basic settings UI

**What to build**:
1. Create `src/settings.html`:
   ```html
   <!DOCTYPE html>
   <html>
   <head>
       <title>JustCall Settings</title>
       <link rel="stylesheet" href="styles.css">
   </head>
   <body>
       <h1>Settings</h1>
       <div id="targets-list"></div>
       <button id="add-target">Add Target</button>
   </body>
   </html>
   ```
2. Basic CSS for clean look
3. Wire up "Settings" menu item to open this window

**How to test**:
1. Click tray icon → Settings
2. Window should open with title "JustCall Settings"
3. Window should be normal size (600x400)
4. Close button should work
5. Can reopen after closing

**UI testing checklist**:
- [ ] Window opens centered
- [ ] Responsive to resize
- [ ] Native close/minimize/maximize work
- [ ] Keyboard shortcuts (Cmd+W) close window
- [ ] Multiple opens don't create multiple windows

---

## Phase 4: Platform Integration

### Step 4.1: Global Hotkey Service
**Goal**: Register system-wide keyboard shortcuts

**What to build**:
```rust
// File: src-tauri/src/services/global_shortcuts.rs
use tauri::GlobalShortcutManager;

pub struct GlobalShortcutService {
    shortcuts: HashMap<String, String>, // hotkey -> action
}

impl GlobalShortcutService {
    pub fn register_hotkey(&mut self, hotkey: &str, action: &str) -> Result<()> {
        // Use Tauri's global shortcut API
        // Check for conflicts
        // Store in map
    }
    
    pub fn unregister_all(&mut self) { }
}
```

**How to implement**:
1. Add Tauri global-shortcut plugin
2. Handle registration errors (conflicts)
3. Emit events when hotkeys pressed

**How to test**:
1. Register test hotkey (e.g., Ctrl+Alt+T)
2. Press it from any app
3. Should see event in console
4. Try registering same key twice (should error)
5. Try system hotkeys (should fail gracefully)

**Test scenarios**:
- Focus on different apps, press hotkey
- Register invalid hotkey format
- Register OS-reserved keys
- Spam hotkey rapidly
- Register 10+ hotkeys

---

### Step 4.2: Window Manager
**Goal**: Create and manage the video call window

**What to build**:
```rust
// File: src-tauri/src/services/conference_window.rs
pub struct ConferenceWindow {
    window: Option<tauri::Window>,
}

impl ConferenceWindow {
    pub fn open(&mut self, always_on_top: bool) -> Result<()> {
        // Create new Tauri window
        // Set size, position, always-on-top
        // Load conference.html
    }
    
    pub fn close(&mut self) { }
    pub fn is_open(&self) -> bool { }
}
```

**How to test**:
1. Call `open()` - window appears
2. Verify always-on-top works
3. Try to create second window (should reuse)
4. Close window programmatically
5. Window disappears fully

**Edge case testing**:
- Multi-monitor setups
- Window restore after minimize
- Full screen apps interaction
- Virtual desktop switching
- Sleep/wake behavior

---

## Phase 5: Call Controller

### Step 5.1: Basic Call Controller
**Goal**: Implement the core business logic state machine

**What to build**:
```rust
// File: src-tauri/src/controllers/call_controller.rs
use std::sync::Mutex;

pub struct CallController {
    state: Mutex<CallState>,
    current_target: Mutex<Option<String>>,
    window_manager: ConferenceWindow,
    settings_store: SettingsStore,
}

impl CallController {
    pub fn join(&self, target_id: &str) -> Result<()> {
        let mut state = self.state.lock().unwrap();
        
        // Check if we can transition
        if *state != CallState::Idle {
            return Err(anyhow!("Already in call"));
        }
        
        // Update state
        *state = CallState::Connecting;
        
        // Get target from settings
        // Derive room ID
        // Open window
        // Pass room info to webview
    }
    
    pub fn hangup(&self) -> Result<()> { }
}
```

**How to test**:
```rust
#[test]
fn test_join_when_idle() {
    let controller = create_test_controller();
    assert!(controller.join("test-target").is_ok());
    assert_eq!(controller.get_state(), CallState::Connecting);
}

#[test]
fn test_join_when_busy() {
    let controller = create_test_controller();
    controller.join("target1").unwrap();
    
    // Second join should fail
    let result = controller.join("target2");
    assert!(result.is_err());
    assert!(result.unwrap_err().to_string().contains("Already in call"));
}
```

**Integration testing**:
1. Create mock window manager
2. Simulate full call flow
3. Verify state transitions
4. Test concurrent access (mutex behavior)

---

### Step 5.2: Controller ↔ Hotkey Integration
**Goal**: Connect hotkeys to controller actions

**What to build**:
```rust
// In main.rs or app setup
fn setup_hotkeys(app: &mut App) -> Result<()> {
    let controller = app.state::<CallController>();
    let shortcuts = app.state::<GlobalShortcutService>();
    
    // Register join primary
    shortcuts.register("Cmd+Opt+J", move || {
        controller.join_primary()?;
    })?;
    
    // Register hangup
    shortcuts.register("Cmd+Opt+H", move || {
        controller.hangup()?;
    })?;
}
```

**How to test**:
1. Launch app
2. Press Cmd+Opt+J
3. Should see state change to Connecting
4. Press Cmd+Opt+H  
5. Should see state change to Disconnecting → Idle

**Test matrix**:
| Current State | Hotkey | Expected Result |
|--------------|---------|-----------------|
| Idle | Join | → Connecting |
| Connecting | Join | Toast "Already joining" |
| InCall | Join | Toast "Already in call" |
| Idle | Hangup | Nothing happens |
| InCall | Hangup | → Disconnecting |

---

## Phase 6: Jitsi Integration

### Step 6.1: Jitsi Bridge (TypeScript)
**Goal**: Create TypeScript wrapper for Jitsi iFrame API

**What to build**:
```typescript
// File: src/webview/jitsi_bridge.ts
interface JitsiOptions {
    roomName: string;
    displayName?: string;
    startWithAudioMuted?: boolean;
    startWithVideoMuted?: boolean;
}

class JitsiBridge {
    private api: any;
    
    async createMeeting(options: JitsiOptions): Promise<void> {
        // Load Jitsi external API script
        // Create iframe with options
        // Set up event listeners
        // Emit events to Tauri backend
    }
    
    hangup(): void {
        this.api?.executeCommand('hangup');
    }
}
```

**How to test in isolation**:
1. Create `test-jitsi.html`:
   ```html
   <!DOCTYPE html>
   <html>
   <head>
       <script src='https://meet.jit.si/external_api.js'></script>
       <script src="jitsi_bridge.js"></script>
   </head>
   <body>
       <div id="meet"></div>
       <button onclick="testJoin()">Test Join</button>
   </body>
   </html>
   ```
2. Open in browser (not Tauri yet)
3. Click Test Join
4. Should see Jitsi iframe load
5. Verify no pre-join screen

**Event testing**:
- Monitor console for events
- Join with two browsers
- Verify participantJoined fires
- Test hangup
- Test connection failures

---

### Step 6.2: Conference Page
**Goal**: Create the actual video call page

**What to build**:
```html
<!-- File: src/conference.html -->
<!DOCTYPE html>
<html>
<head>
    <title>JustCall</title>
    <style>
        body { margin: 0; overflow: hidden; }
        #jitsi-container { width: 100vw; height: 100vh; }
    </style>
</head>
<body>
    <div id="jitsi-container"></div>
    <script src="webview/jitsi_bridge.js"></script>
    <script>
        // Listen for room info from Tauri
        window.__TAURI__.event.listen('start-call', (event) => {
            const bridge = new JitsiBridge();
            bridge.createMeeting(event.payload);
        });
    </script>
</body>
</html>
```

**How to test**:
1. Load page in Tauri webview
2. Emit 'start-call' event with room info
3. Jitsi should auto-load and join
4. Video/audio should work
5. UI should be minimal

**Quality checks**:
- [ ] No scrollbars
- [ ] Jitsi fills entire window
- [ ] Toolbar is accessible
- [ ] Can toggle camera/mic
- [ ] Hangup button works

---

### Step 6.3: Tauri ↔ Webview Events
**Goal**: Two-way communication between Rust and JavaScript

**What to build**:
```rust
// Rust side - emit events
window.emit("start-call", json!({
    "roomName": room_id,
    "displayName": "You",
    "config": target.defaults
}))?;

// Rust side - listen to events
window.listen("call-joined", |event| {
    println!("Call joined!");
    controller.transition_to(CallState::InCall);
});
```

```typescript
// JS side - emit to Rust
window.__TAURI__.emit('call-joined', {});
window.__TAURI__.emit('participant-joined', { id: participant.id });
window.__TAURI__.emit('call-ended', {});
```

**How to test**:
1. Add console logs on both sides
2. Trigger each event type
3. Verify bidirectional flow
4. Test with malformed payloads
5. Test rapid event firing

**Event flow diagram to verify**:
```
User presses hotkey
  → Rust: controller.join()
  → Rust: emit("start-call")
  → JS: receives event
  → JS: creates Jitsi
  → Jitsi: fires videoConferenceJoined
  → JS: emit("call-joined")
  → Rust: transition to InCall
```

---

## Phase 7: Target Management

### Step 7.1: Target CRUD Operations
**Goal**: Add/edit/remove targets through settings UI

**What to build**:
```typescript
// File: src/settings.js
class SettingsManager {
    async loadTargets() {
        const targets = await window.__TAURI__.invoke('get_targets');
        this.renderTargets(targets);
    }
    
    async addTarget() {
        // Generate code (call Rust)
        // Show add target dialog
        // Save to settings store
    }
    
    async removeTarget(id: string) {
        if (confirm('Remove this target?')) {
            await window.__TAURI__.invoke('remove_target', { id });
        }
    }
}
```

**How to test**:
1. Open settings
2. Click "Add Target"
3. Enter label "Test Partner"
4. Code should auto-generate
5. Save - should appear in list
6. Remove - should disappear
7. Restart app - changes persist

**Edge cases**:
- Long labels
- Unicode in labels
- Duplicate labels (allowed)
- Remove primary target
- Remove during active call

---

### Step 7.2: Invite System
**Goal**: Share targets via URL scheme

**What to build**:
1. Register URL scheme in Tauri:
   ```json
   "tauri": {
     "bundle": {
       "identifier": "com.justcall.app",
       "protocols": {
         "schemes": ["justcall"]
       }
     }
   }
   ```

2. Handle deep links:
   ```rust
   // Parse justcall://add?c=<code>&label=<label>
   fn handle_url_scheme(url: String) {
       // Parse query params
       // Validate code format
       // Add to targets
       // Show confirmation
   }
   ```

**How to test**:
1. Build app and install
2. Create test URL: `justcall://add?c=test-code-1234&label=Alice`
3. Click link in browser
4. App should open (or focus)
5. Confirmation dialog appears
6. Accept - target added

**Security testing**:
- Malformed URLs
- XSS attempts in label
- Missing parameters
- Duplicate additions
- Very long parameters

---

### Step 7.3: Multi-Target Hotkeys
**Goal**: Assign per-target hotkeys

**What to build**:
```rust
// Dynamic hotkey registration
pub fn register_target_hotkeys(targets: &[Target]) -> Result<()> {
    for (index, target) in targets.iter().enumerate() {
        if let Some(hotkey) = &target.custom_hotkey {
            // Register custom hotkey
        } else if index < 9 {
            // Register Cmd+Opt+[1-9]
            let hotkey = format!("Cmd+Opt+{}", index + 1);
            register_hotkey(&hotkey, move || {
                controller.join(&target.id)?;
            })?;
        }
    }
}
```

**How to test**:
1. Add 3 targets
2. Press Cmd+Opt+1 - joins first
3. Hangup
4. Press Cmd+Opt+2 - joins second
5. Reorder targets in settings
6. Hotkeys should update

**Stress testing**:
- 10+ targets
- Conflicting hotkeys
- Remove target while in call
- Rapid hotkey switching
- Invalid hotkey formats

---

## Phase 8: Polish & Edge Cases

### Step 8.1: Permissions Handling
**Goal**: Handle camera/microphone permissions gracefully

**Platform-specific implementation**:

**macOS**:
```xml
<!-- Info.plist additions -->
<key>NSCameraUsageDescription</key>
<string>JustCall needs camera access for video calls</string>
<key>NSMicrophoneUsageDescription</key>
<string>JustCall needs microphone access for calls</string>
```

**Windows**: Handled by Windows 10/11 privacy settings

**Linux**: Usually just works if devices are available

**How to test**:
1. Fresh install on system with no permissions
2. Join call - OS permission prompt appears
3. Deny camera - should join audio only
4. Deny both - should show error
5. Re-enable in OS settings - should work

**Recovery flows**:
- Show "Enable in System Preferences" message
- Retry button after enabling
- Join audio-only option
- Settings link to OS permissions

---

### Step 8.2: Error States & Toasts
**Goal**: User-friendly error messages

**What to build**:
```typescript
// File: src/toast.ts
class ToastManager {
    show(message: string, type: 'info' | 'error' | 'warning') {
        // Create toast element
        // Auto-dismiss after 3s
        // Stack multiple toasts
        // Click to dismiss
    }
}

// Usage throughout app
toast.show('Already in a call', 'warning');
toast.show('Failed to connect', 'error');
toast.show('Partner joined', 'info');
```

**How to test each error**:
1. **Already in call**: Join, try join again
2. **Network error**: Disconnect internet, try join
3. **Invalid target**: Delete target, use old hotkey
4. **Permissions denied**: Deny camera/mic
5. **Hotkey conflict**: Register system hotkey

**Toast behavior**:
- Max 3 toasts visible
- Newer pushes older up
- Click dismisses immediately  
- Auto-dismiss after 3 seconds
- Different colors per type

---

### Step 8.3: Onboarding Flow
**Goal**: First-run setup wizard

**What to build**:
```typescript
// File: src/onboarding.js
class OnboardingWizard {
    steps = [
        'welcome',
        'create-first-target',
        'set-hotkeys',
        'test-permissions',
        'all-done'
    ];
    
    async start() {
        // Show if no targets exist
        // Guide through each step
        // Test hotkeys work
        // Test camera/mic
        // Save and close
    }
}
```

**Step-by-step testing**:
1. Delete all app data
2. Launch app - wizard appears
3. Step 1: Welcome - explains app
4. Step 2: Create target - generate code
5. Step 3: Set hotkeys - test capture
6. Step 4: Camera/mic test
7. Step 5: Try it now!

**Edge cases**:
- Close wizard early
- Skip permissions
- Back button behavior
- Restart wizard
- Upgrade from old version

---

## Phase 9: Distribution

### Step 9.1: Build & Package
**Goal**: Create installable packages

**Build commands**:
```bash
# Development
npm run tauri dev

# Production builds
npm run tauri build # Creates all platforms

# Platform specific
npm run tauri build -- --target universal-apple-darwin # macOS
npm run tauri build -- --target x86_64-pc-windows-msvc # Windows
npm run tauri build -- --target x86_64-unknown-linux-gnu # Linux
```

**Output locations**:
- macOS: `target/release/bundle/dmg/JustCall.dmg`
- Windows: `target/release/bundle/msi/JustCall.msi`
- Linux: `target/release/bundle/appimage/JustCall.AppImage`

**Testing installers**:
1. Copy to fresh VM/machine
2. Install without admin rights
3. Verify app appears in Applications
4. Verify URL scheme registered
5. Verify auto-start option works
6. Uninstall cleanly

**Code signing** (macOS):
- Need Apple Developer ID
- Sign with: `codesign --deep --force --verify --verbose --sign "Developer ID" JustCall.app`
- Notarize for Gatekeeper

---

### Step 9.2: Auto-Update (Optional)
**Goal**: Ship updates seamlessly

**Setup**:
1. Host update JSON on GitHub/S3
2. Configure in tauri.conf.json:
   ```json
   "updater": {
     "active": true,
     "endpoints": ["https://yourdomain.com/update.json"],
     "dialog": true,
     "pubkey": "YOUR_PUBLIC_KEY"
   }
   ```

**Testing updates**:
1. Build v1.0.0, install
2. Build v1.0.1 with visible change
3. Host update manifest
4. Launch v1.0.0
5. Should prompt to update
6. Accept - downloads and installs
7. Relaunches with v1.0.1

---

## Development Workflow

### Daily Development Cycle
1. Pick a step from current phase
2. Create feature branch
3. Implement with tests
4. Run: `cargo test` and `npm test`
5. Manual testing per step guide
6. Commit with clear message
7. PR with checklist

### Testing Philosophy
- Unit tests for pure functions
- Integration tests for workflows  
- Manual tests for UX
- Cross-platform verification

### Code Quality Checklist
- [ ] Comments explain "used by" references
- [ ] Error messages are helpful
- [ ] No unwraps in production code
- [ ] Settings changes are backwards compatible
- [ ] New features have tests

---

## Success Metrics

Each phase is complete when:
1. All tests pass
2. Manual testing checklist complete
3. No crashes or hangs
4. Works on all target platforms
5. Code is documented
6. Next developer can understand it

Remember: Build incrementally, test constantly, ship working code!
