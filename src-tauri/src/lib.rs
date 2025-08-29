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
            println!("Setting up JustCall app...");
            
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
                        println!("quit menu item was clicked");
                        app.exit(0);
                    }
                    "settings" => {
                        println!("settings menu item was clicked");
                        // Show settings window
                        if let Some(window) = app.get_webview_window("main") {
                            let _ = window.show();
                            let _ = window.set_focus();
                        }
                    }
                    _ => {
                        println!("menu item {:?} not handled", event.id);
                    }
                })
                .on_tray_icon_event(|tray, event| match event {
                    TrayIconEvent::Click {
                        button: MouseButton::Left,
                        button_state: MouseButtonState::Up,
                        ..
                    } => {
                        println!("left click pressed and released");
                        // For now, just log it
                    }
                    TrayIconEvent::Click {
                        button: MouseButton::Right,
                        ..
                    } => {
                        println!("right click detected");
                    }
                    _ => {
                        println!("unhandled tray event: {:?}", event);
                    }
                })
                .build(app)?;
            
            println!("Tray icon created with menu!");
            
            // Hide main window on startup
            if let Some(window) = app.get_webview_window("main") {
                window.hide()?;
                println!("Main window hidden");
            }
            
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}