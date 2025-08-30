# Debugging JustCall Connection Issues

## How the Pairing Code System Works

1. **Shared Code**: Both devices use the same pairing code (e.g., `7aap-ofsd-vxrb-nu4c-ei4z`)
2. **Room Derivation**: The code is hashed to create a deterministic Jitsi room ID
   - Your code: `7aap-ofsd-vxrb-nu4c-ei4z`
   - Generated room: `jc-rggejhvgqeeesdsg`
   - Jitsi URL: `https://meet.jit.si/jc-rggejhvgqeeesdsg`
3. **Connection**: Both devices join the same room on Jitsi's servers

## Why It Works in Development

- Yes, this works in localhost/dev mode! 
- Jitsi runs on their public servers (meet.jit.si)
- No deployment needed - the room exists on Jitsi's infrastructure
- Both devices just need to join the same room name

## Debugging Steps

### 1. Check the Console Logs

Open the Developer Console in the conference window:
- **macOS**: Right-click in the window → Inspect Element → Console
- Look for these log messages:
  ```
  === START CALL EVENT RECEIVED ===
  Room ID: jc-rggejhvgqeeesdsg
  Jitsi URL will be: https://meet.jit.si/jc-rggejhvgqeeesdsg
  ```

### 2. Check Terminal Logs

In the terminal where `npm run tauri dev` is running, look for:
```
Primary target found: Testreal1 with code: 7aap-ofsd-vxrb-nu4c-ei4z
Generated room ID from code '7aap-ofsd-vxrb-nu4c-ei4z': 'jc-rggejhvgqeeesdsg'
```

### 3. Test Jitsi Directly

Open `test-jitsi.html` in a regular browser on both devices:
```bash
open test-jitsi.html  # macOS
```

This tests if Jitsi works without the Tauri wrapper.

### 4. Common Issues

1. **Stuck on "Connecting to call..."**
   - The Tauri API might not be loading correctly
   - Check browser console for errors
   - Try refreshing the app (Cmd+R)

2. **Network/Firewall**
   - Ensure both devices can access meet.jit.si
   - Corporate firewalls might block WebRTC
   - Try on a personal hotspot

3. **Different Room IDs**
   - Verify both devices show the same code in settings
   - Check for typos or extra spaces
   - Use the test script: `node test-room-derivation.js`

4. **Permissions**
   - Grant camera/microphone access when prompted
   - Check System Preferences → Security & Privacy

### 5. Quick Test

The fastest way to verify everything works:
1. Open `test-jitsi.html` on both MacBooks
2. Click "Join Room" on both
3. If you can see each other → Jitsi works, issue is in the app
4. If you can't → Network/firewall issue

## What the Logs Tell Us

When working correctly, you should see:
1. Hotkey pressed → "Join primary target requested"
2. Code found → "Primary target found: X with code: Y"
3. Room generated → "Generated room ID from code 'Y': 'jc-...''"
4. Window opens → Conference page loads
5. Jitsi connects → "Conference joined" in console
6. Partner joins → "Participant joined" event

The room ID ensures both devices meet in the same virtual room without any server coordination.
