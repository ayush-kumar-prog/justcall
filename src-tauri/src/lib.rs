use tauri::{
    menu::{Menu, MenuItem},
    tray::{MouseButton, MouseButtonState, TrayIconBuilder, TrayIconEvent},
    Manager,
};

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_log::Builder::new().build())
        .setup(|app| {
            // Create menu items
            let quit_i = MenuItem::with_id(app, "quit", "Quit", true, None::<&str>)?;
            let settings_i = MenuItem::with_id(app, "settings", "Settings", true, None::<&str>)?;
            
            // Create menu
            let menu = Menu::with_items(app, &[&settings_i, &quit_i])?;
            
            // Create tray icon with menu
            let _tray = TrayIconBuilder::new()
                .icon(app.default_window_icon().unwrap().clone())
                .tooltip("JustCall - Right-click for menu")
                .menu(&menu)
                .menu_on_left_click(false) // Right-click only for menu
                .on_menu_event(|app, event| match event.id.as_ref() {
                    "quit" => {
                        log::info!("User selected quit from tray menu");
                        app.exit(0);
                    }
                    "settings" => {
                        log::info!("User selected settings from tray menu");
                        // Show settings window
                        if let Some(window) = app.get_webview_window("main") {
                            let _ = window.show();
                            let _ = window.set_focus();
                        }
                    }
                    _ => {
                        log::warn!("Unknown menu item: {:?}", event.id);
                    }
                })
                .on_tray_icon_event(|tray, event| {
                    // Only log significant events, not every mouse movement
                    match event {
                        TrayIconEvent::Click {
                            button: MouseButton::Left,
                            button_state: MouseButtonState::Up,
                            ..
                        } => {
                            log::debug!("Tray icon left-clicked");
                        }
                        TrayIconEvent::DoubleClick { .. } => {
                            log::debug!("Tray icon double-clicked");
                        }
                        // Ignore Move, Enter, Leave events to reduce noise
                        TrayIconEvent::Move { .. } |
                        TrayIconEvent::Enter { .. } |
                        TrayIconEvent::Leave { .. } => {}
                        _ => {
                            log::debug!("Tray event: {:?}", event);
                        }
                    }
                })
                .build(app)?;
            
            // Hide main window on startup
            if let Some(window) = app.get_webview_window("main") {
                window.hide()?;
            }
            
            log::info!("JustCall initialized successfully");
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}