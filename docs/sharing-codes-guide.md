# How to Share Codes Between Devices

## Current Method (Temporary)

Since the invite system isn't implemented yet, here's how to share codes:

### On Device A (First Person):
1. Open Settings → Call Targets
2. Click "Add Target" and create a target
3. Copy the generated code (e.g., `gpcr-b4b6-vzu2-t64q-sa5c`)
4. Send this code to your partner (via text, email, etc.)

### On Device B (Second Person):
1. Open Settings → Call Targets
2. Click **"Import Code"** button
3. Paste the code from Device A
4. Enter a label (e.g., "Alex")
5. Click Import

Now both devices have the same code and will join the same video call!

## Future Method (Not Yet Implemented)

The app will eventually support:
- **Invite Links**: `justcall://add?c=xxxx-xxxx&label=Name`
- **QR Codes**: Scan to add target
- **Copy Invite**: Share a formatted invite message

## Testing Instructions

1. **Device A**: Create a target, copy the code
2. **Device B**: Import the code using "Import Code" button
3. **Both**: Press your join hotkeys
4. **Result**: You'll be in the same video call!

The "Import Code" button is a temporary solution until the proper invite system is implemented.
