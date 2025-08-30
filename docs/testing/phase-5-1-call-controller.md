# Phase 5.1: Call Controller - Testing Guide

## What's New in Phase 5.1

The CallController adds proper state management to prevent issues like:
- Joining multiple calls at once
- Pressing hotkeys in wrong states
- Race conditions during connect/disconnect

### State Machine
```
Idle → Connecting → InCall → Disconnecting → Idle
```

## Benefits Over Phase 4

1. **State Enforcement**: Can't join while already in a call
2. **Better Error Messages**: "Cannot join while InCall" instead of window errors
3. **Event Integration**: Tracks when Jitsi actually connects
4. **Clean Architecture**: Centralized call logic

## How to Test

### 1. Basic Call Flow (Should Still Work)
1. Press join hotkey (e.g., Ctrl+Shift+J)
2. Window opens, Jitsi connects
3. Press hangup hotkey (e.g., Ctrl+Alt+Shift+H)
4. Window closes

### 2. State Protection (NEW)
1. Join a call
2. **While in call**, press join hotkey again
3. Expected: Nothing happens (no error window)
4. Check logs for: "Cannot join: current state is InCall"

### 3. Multi-Device Testing
1. Set up on two machines with same target code
2. Both press join hotkey
3. **They will join the same video call!**
4. Both can see/hear each other

### 4. Edge Cases Now Handled
- **Spam Protection**: Rapidly pressing join won't create multiple windows
- **State Tracking**: Console shows state changes (Idle → Connecting → InCall)
- **Clean Hangup**: Proper state cleanup on disconnect

## Console Messages to Look For

Success flow:
```
Starting call to target: [id]
Conference joined, transitioning to InCall
Hanging up call
Conference left, going idle
```

Protection working:
```
Cannot join: current state is InCall
Cannot join while Connecting
```

## Cross-Device Calling Confirmation

YES! Two devices with the same code WILL connect to each other:
1. Both derive same room ID from shared code
2. Both connect to same Jitsi room
3. Full video/audio between devices works

The CallController doesn't change this - it just adds safety!
