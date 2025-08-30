# Missing Tray Icon on macOS

## Problem
The app is running (hotkeys work) but the tray icon is not visible in the menu bar.

## Common Causes & Solutions

### 1. Hidden in Menu Bar
- Hold `Cmd` and drag menu bar items to check if it's hidden behind others
- Some apps can push icons off-screen

### 2. macOS Permissions
- Go to System Preferences → Security & Privacy → Privacy
- Check if JustCall has necessary permissions
- Try: Accessibility permissions (for global hotkeys)

### 3. Company Laptop Restrictions
If it's a company laptop:
- MDM (Mobile Device Management) policies might hide certain tray icons
- Check with IT if there are restrictions on menu bar apps
- The app still works because the process runs, just the UI is hidden

### 4. Icon File Issue
The tray icon might be transparent or missing:
- Check if `src-tauri/icons/icon.png` exists
- Icon might not be included in the build

### 5. Quick Fixes to Try

1. **Restart the app completely**:
   ```bash
   # Kill any running instances
   pkill -f justcall
   # Start fresh
   npm run tauri dev
   ```

2. **Check if process is running**:
   ```bash
   ps aux | grep -i justcall
   ```

3. **Reset menu bar**:
   ```bash
   killall SystemUIServer
   ```

4. **Check Console for errors**:
   - Open Console.app
   - Filter for "justcall"
   - Look for tray-related errors

### 6. Temporary Workaround
Since hotkeys work, you can still use the app:
- Settings window might not be accessible
- But calling functionality works fine
- Configure settings on the working machine, copy settings.json to the problem machine

### 7. Debug Commands
Run these in terminal where the app is running:
```bash
# Check if tray initialization succeeded
grep -i "tray" *.log
grep -i "icon" *.log
```

## Settings File Location
If you can't access settings via UI:
- macOS: `~/Library/Application Support/justcall/settings.json`
- You can manually edit this file

## Note
The fact that hotkeys work means the core app is running fine. This is purely a UI/tray display issue, likely due to corporate policies or macOS restrictions.
