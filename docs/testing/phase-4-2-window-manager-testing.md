# Phase 4.2: Window Manager Testing Guide

## Overview
This guide covers testing the ConferenceWindow service which manages the video call window lifecycle.

## Prerequisites
1. Ensure you have a target configured (run the app and add a target through Settings)
2. Have the development server running: `npm run tauri dev`
3. Check the system tray for the JustCall icon

## Test Cases

### 1. Basic Window Creation
**Goal**: Verify window opens when hotkey is pressed

**Steps**:
1. Make sure JustCall is running (check system tray)
2. Press the join hotkey (default: Cmd+Opt+J on macOS, Ctrl+Alt+J on Windows/Linux)
3. Conference window should appear

**Expected**:
- Window opens centered on screen
- Window size is 1024x768
- Loading spinner shows briefly
- Window title is "JustCall"
- If always-on-top is enabled in settings, window stays above others

### 2. Window Reuse
**Goal**: Verify that pressing join while window is open reuses existing window

**Steps**:
1. Open conference window (press join hotkey)
2. While window is open, press join hotkey again
3. Observe behavior

**Expected**:
- Existing window gains focus
- No new window is created
- Window updates with new call configuration if different target

### 3. Hangup Functionality
**Goal**: Verify window closes when hangup hotkey is pressed

**Steps**:
1. Open conference window (press join hotkey)
2. Press hangup hotkey (default: Cmd+Opt+H on macOS, Ctrl+Alt+H on Windows/Linux)
3. Observe behavior

**Expected**:
- Window closes gracefully
- No crash or error
- Can open new window after hangup

### 4. Jitsi Integration
**Goal**: Verify Jitsi loads correctly in the window

**Steps**:
1. Ensure you have internet connection
2. Open conference window
3. Wait for Jitsi to load

**Expected**:
- Loading spinner disappears
- Jitsi interface appears
- Camera/microphone permissions may be requested (first time)
- Can see self-view if camera is enabled
- Toolbar with mute/camera/hangup buttons is visible

### 5. Error Handling
**Goal**: Test error scenarios

**Test 5.1 - No Internet**:
1. Disconnect from internet
2. Try to open conference window
3. Expected: Error message appears with retry button

**Test 5.2 - No Primary Target**:
1. Remove all targets from settings
2. Press join primary hotkey
3. Expected: Warning in console, no window opens

### 6. Window Properties
**Goal**: Verify window behaves correctly

**Tests**:
- Minimize: Window can be minimized and restored
- Resize: Window can be resized (minimum 640x480)
- Close button: Native close button ends call and closes window
- Always-on-top: If enabled, window stays above other windows
- Multi-monitor: Window opens on active monitor

### 7. State Management
**Goal**: Verify only one call at a time

**Steps**:
1. Open conference window
2. Try to join a different target
3. Expected: Current behavior ignores the request (future: may show toast)

### 8. Performance
**Goal**: Ensure smooth operation

**Tests**:
- Window opens within 1-2 seconds
- No memory leaks after multiple open/close cycles
- CPU usage reasonable during call
- Window closes immediately on hangup

## Edge Cases to Test

### Multi-Monitor Setup
1. Connect external monitor
2. Open window on primary screen
3. Move to secondary screen
4. Close and reopen - should remember position (future feature)

### Window Recovery
1. Open conference window
2. Force quit the app (not normal quit)
3. Restart app
4. Should be able to open new window without issues

### Rapid Actions
1. Rapidly press join/hangup hotkeys
2. Expected: Actions are debounced, no double windows or crashes

### Different Targets
1. Configure multiple targets with hotkeys
2. Test joining each target
3. Verify correct room is joined for each

## Known Limitations (MVP)
- No call state persistence across app restarts
- No incoming call notifications
- Window doesn't remember last position/size
- No preview before joining

## Debugging

Check logs for issues:
- Console logs in DevTools (if development build)
- Application logs in:
  - macOS: `~/Library/Logs/JustCall/`
  - Windows: `%APPDATA%\JustCall\logs\`
  - Linux: `~/.config/justcall/logs/`

Common issues:
- **Window doesn't open**: Check if app is running, hotkey is correct
- **Jitsi doesn't load**: Check internet connection, firewall settings
- **Permissions denied**: Grant camera/microphone access in system settings

## Success Criteria
- [ ] Window opens reliably on hotkey press
- [ ] Jitsi loads and connects successfully
- [ ] Window closes properly on hangup
- [ ] No crashes during normal use
- [ ] Edge cases handled gracefully

