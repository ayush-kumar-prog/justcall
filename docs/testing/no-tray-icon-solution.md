# Solution for Mac Without Tray Icon

## Quick Access to Settings

Since your company Mac doesn't show the tray icon, here's how to access settings:

### Method 1: Direct URL (Easiest)
1. Make sure JustCall is running (test with hotkeys)
2. Open your browser
3. Go to: `http://127.0.0.1:1420/open-settings.html`
4. Click "Open Settings Window"

### Method 2: Manual Settings Edit
1. Open Finder
2. Press `Cmd+Shift+G` and go to: `~/Library/Application Support/justcall/`
3. Open `settings.json` in a text editor
4. You can manually add targets here

### Method 3: Use the Helper Page
I've created `open-settings.html` in the dist folder that can:
- Open the settings window programmatically
- Show instructions for importing codes
- Display the settings file location

## Testing Cross-Device Calls

### On Mac WITH Tray (Device A):
1. Create a target in Settings
2. Copy the code (e.g., `gpcr-b4b6-vzu2-t64q-sa5c`)
3. Send to Device B

### On Mac WITHOUT Tray (Device B):
1. Use Method 1 above to open Settings
2. Click "Import Code"
3. Paste the code from Device A
4. Enter a label
5. Click Import

### Both Devices:
1. Press your join hotkeys
2. You'll connect to the same video call!

## Current Button Status

After the latest fixes:
- **Import button**: Should work with console logs
- **Cancel button**: Should close the modal
- **Remove button**: Should remove and auto-save

Make sure to **close and reopen** the settings window to load the latest JavaScript fixes.
