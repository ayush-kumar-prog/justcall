# JustCall - Polishing Todo List

## Minor Issues to Fix Later

### Settings UI
- [ ] **Remove button not working** - Click event not firing, needs investigation
  - No console output when clicked
  - Might need to rebind event handlers after render
  - Could implement custom confirmation modal

### UI/UX Improvements
- [ ] Add loading states for async operations
- [ ] Implement proper toast notifications
- [ ] Add keyboard shortcuts in settings (Esc to close modals)
- [ ] Better error messages with actionable steps
- [ ] Visual feedback when hotkeys are pressed

### Window Management
- [ ] Remember window position/size between sessions
- [ ] Smooth transitions when opening/closing windows
- [ ] Handle multi-monitor scenarios better
- [ ] Add minimize to tray option

### Hotkey System
- [ ] Validate hotkey conflicts with system shortcuts
- [ ] Show which hotkeys are already in use
- [ ] Add hotkey for mute/unmute during call
- [ ] Add hotkey for toggle video during call

### Call Features
- [ ] Add connection quality indicator
- [ ] Show participant count in window title
- [ ] Add reconnect capability on connection loss
- [ ] Pre-call device testing

### Security & Privacy
- [ ] Encrypt stored settings
- [ ] Add option to clear all data
- [ ] Implement secure code sharing via QR

### Performance
- [ ] Optimize window creation time
- [ ] Reduce memory usage when idle
- [ ] Lazy load Jitsi resources

## Priority for MVP
Focus on:
1. Core calling functionality
2. Reliable hotkey system
3. Basic settings management
4. Cross-platform compatibility

These polish items can be addressed in future iterations.
