# Debugging Remove and Import Buttons

## Current Issues

### Remove Button
- The button is firing (we see console logs)
- The confirm dialog might be blocked by Tauri/WebKit
- It's immediately returning false or being cancelled

### Import Button
- No console logs = event handler not attached
- Button might not exist when JavaScript runs

## Quick Test

1. **Close and fully reopen** the Settings window
2. Open Developer Console (right-click → Inspect)
3. Look for these messages on load:
   - `Import button found: [element]` or `Import button not found!`

## Temporary Fix for Remove

Since confirm dialogs might be blocked, try this:
1. I've added error handling that will remove without confirmation if the dialog fails
2. Check console for "Confirm dialog error" message

## Next Steps

If Import button still doesn't work:
1. Hard refresh the page (Cmd+Shift+R on Mac)
2. Or close the app completely and restart

The buttons should show debug messages now to help identify the issue.
