# Manual Code Setup (Direct File Edit)

## Settings File Location on macOS
```
~/Library/Application Support/justcall/settings.json
```

## Steps to Add Code Manually:

1. **Open Terminal** and navigate to settings:
```bash
cd ~/Library/Application\ Support/justcall/
```

2. **Check if settings.json exists**:
```bash
ls -la
```

3. **If file doesn't exist**, create the directory and file:
```bash
mkdir -p ~/Library/Application\ Support/justcall/
```

4. **Edit the file** with your preferred editor:
```bash
nano ~/Library/Application\ Support/justcall/settings.json
# or
code ~/Library/Application\ Support/justcall/settings.json
# or
vim ~/Library/Application\ Support/justcall/settings.json
```

## Settings.json Format

If the file is **empty or doesn't exist**, paste this entire structure:

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
    "join_primary": "Ctrl+Shift+J",
    "hangup": "Ctrl+Alt+Shift+H",
    "target_hotkeys": {}
  },
  "targets": [
    {
      "id": "primary-target",
      "label": "Partner",
      "code": "PASTE-THE-CODE-HERE",
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

## Important: Replace "PASTE-THE-CODE-HERE"

Replace `"PASTE-THE-CODE-HERE"` with the actual code from Device A, for example:
```json
"code": "gpcr-b4b6-vzu2-t64q-sa5c",
```

## If settings.json already exists:

Look for the `"targets"` array and add your target:
```json
"targets": [
  {
    "id": "new-target-id",
    "label": "Partner",
    "code": "gpcr-b4b6-vzu2-t64q-sa5c",
    "type": "person",
    "is_primary": true,
    "call_defaults": {
      "start_with_audio": true,
      "start_with_video": true
    },
    "created_at": "2024-01-01T00:00:00Z"
  }
]
```

## After Editing:

1. **Save the file**
2. **Restart the app** (if it's running)
3. **Test**: Press the hotkey (Ctrl+Shift+J or configured key)
4. Both laptops should connect to the same video call!

## Quick Command (One-liner)

If you want to quickly check your current settings:
```bash
cat ~/Library/Application\ Support/justcall/settings.json | python3 -m json.tool
```

## Troubleshooting

- Make sure the JSON is valid (no missing commas, quotes, etc.)
- The `code` field must match EXACTLY between both devices
- Set `"is_primary": true` for the target you want to use with the main hotkey
- If hotkeys don't work, check the `keybinds` section matches your OS
