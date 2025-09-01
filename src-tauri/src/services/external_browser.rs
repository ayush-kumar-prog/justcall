// External browser service - opens meetings in system default browser
// This is now the primary way to join meetings

use tauri::AppHandle;
use tauri_plugin_shell::ShellExt;

pub struct ExternalBrowserService;

impl ExternalBrowserService {
    /// Opens a meeting URL in the system's default browser
    pub fn open_meeting(app_handle: &AppHandle, room_id: &str) -> Result<(), String> {
        let url = format!("https://meet.jit.si/{}", room_id);
        
        log::info!("Opening meeting in external browser: {}", url);
        
        // In Tauri v2, we use the shell plugin's open command
        app_handle
            .shell()
            .open(&url, None)
            .map_err(|e| format!("Failed to open browser: {}", e))?;
        
        Ok(())
    }
}