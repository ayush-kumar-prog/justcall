# Phase 4.2 Window Manager - Fixes and Testing

## Issues Fixed

### 1. Window Not Opening on Hotkey Press
**Problem**: The conference window was created once but subsequent hotkey presses failed with "a webview with label `conference` already exists"

**Fix**: Updated the window reuse logic to check Tauri's window manager instead of relying on internal state:
```rust
// Now checks if window exists in Tauri's window manager
if let Some(existing) = self.app_handle.get_webview_window(window_label) {
    // Reuse existing window
}
```

### 2. Edit/Remove Buttons Not Working
**Problem**: The onclick handlers were calling `settingsManager` which wasn't globally accessible

**Fix**: Made `settingsManager` a global variable by assigning it to `window.settingsManager`

## How to Test

### 1. Restart the App
Since we rebuilt with fixes, you need to restart the app if it's still running:
1. Press Ctrl+C in the terminal running `npm run tauri dev`
2. Run `npm run tauri dev` again

### 2. Test Edit/Remove Buttons
1. Right-click tray icon → Settings
2. You should see your existing target
3. Click "Edit" - should open the edit modal with target data
4. Make changes and save
5. Click "Remove" - should remove the target after confirmation

### 3. Test Window Opening
1. Make sure you have a target configured
2. Press your configured hotkey (e.g., Ctrl+Shift+J)
3. The conference window should open
4. Press the hotkey again - window should focus (not create a new one)
5. Press hangup hotkey (e.g., Ctrl+Alt+Shift+H) - window should close
6. Press join hotkey again - new window should open

### 4. Full Flow Test
1. **Setup**: Ensure you have a primary target configured
2. **Join**: Press join hotkey → Window opens → Jitsi loads
3. **Reuse**: Press join hotkey again → Same window focuses
4. **Hangup**: Press hangup hotkey → Window closes
5. **Rejoin**: Press join hotkey → New window opens

### 5. Edge Cases
- **No Internet**: Disconnect WiFi, try to join - should show error
- **Multiple Targets**: Add 2+ targets, test switching between them
- **Rapid Keys**: Press join/hangup rapidly - should handle gracefully

## Expected Behavior
- ✅ Window opens on first join hotkey press
- ✅ Subsequent join presses focus existing window
- ✅ Hangup closes the window
- ✅ Can rejoin after hangup
- ✅ Edit/Remove buttons work in settings
- ✅ Jitsi loads automatically

## Debugging Tips
If issues persist:
1. Check the console logs for errors
2. Make sure the app restarted after fixes
3. Clear settings and reconfigure targets
4. Check if hotkeys are registered (look for "Successfully registered hotkey" in logs)

