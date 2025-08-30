# Important: Settings.json Location

## settings.json is NOT in the Git Repository!

The settings file is created **at runtime** when you first run the app and save settings.

### Location on macOS:
```
~/Library/Application Support/justcall/settings.json
```

NOT in the project directory!

### To Check/Create on Company Laptop:

1. **First, make sure you're in the right project**:
```bash
# Should see these files/folders:
ls ~/path/to/justcall/
# Expected: src/, src-tauri/, dist/, Cargo.toml, package.json
```

2. **Create the settings directory**:
```bash
mkdir -p ~/Library/Application\ Support/justcall/
```

3. **Create settings.json in the APPLICATION SUPPORT folder**:
```bash
nano ~/Library/Application\ Support/justcall/settings.json
```

4. **Paste this content** (replace YOUR-CODE with the actual code):
```json
{
  "version": 1,
  "app_settings": {
    "autostart": false,
    "always_on_top": true,
    "play_join_sound": true,
    "show_notifications": true,
    "theme": "system"
  },
  "keybinds": {
    "join_primary": "Cmd+Opt+J",
    "hangup": "Cmd+Opt+H",
    "target_hotkeys": {}
  },
  "targets": [
    {
      "id": "primary-target",
      "label": "Partner",
      "code": "YOUR-CODE",
      "type": "person",
      "is_primary": true,
      "call_defaults": {
        "start_with_audio": true,
        "start_with_video": true
      },
      "created_at": "2024-01-01T00:00:00Z"
    }
  ]
}
```

### Quick Debug Commands:

```bash
# Check if settings exists
ls -la ~/Library/Application\ Support/justcall/

# Show current settings (if exists)
cat ~/Library/Application\ Support/justcall/settings.json

# Check you're in the right project
pwd  # Should show path to justcall project
ls   # Should show src/, src-tauri/, dist/, etc.
```
