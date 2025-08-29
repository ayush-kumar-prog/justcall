use tauri::{
    menu::{Menu, MenuItem},
    tray::{MouseButton, TrayIconBuilder, TrayIconEvent},
    Manager,
};

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .setup(|app| {
            // Enable debug logging
            if cfg!(debug_assertions) {
                app.handle().plugin(
                    tauri_plugin_log::Builder::default()
                        .level(log::LevelFilter::Info)
                        .build(),
                )?;
            }

            // Create tray icon
            let tray_menu = Menu::new(app.handle())?;
            let settings_item = MenuItem::with_id(app, "settings", "Settings", true, None::<&str>)?;
            let quit_item = MenuItem::with_id(app, "quit", "Quit", true, None::<&str>)?;
            
            tray_menu.append(&settings_item)?;
            tray_menu.append(&quit_item)?;

            let tray = TrayIconBuilder::new()
                .menu(&tray_menu)
                .tooltip("JustCall")
                .on_menu_event(move |app, event| match event.id.as_ref() {
                    "settings" => {
                        // Show settings window
                        if let Some(window) = app.get_webview_window("main") {
                            window.show().unwrap();
                            window.set_focus().unwrap();
                        }
                    }
                    "quit" => {
                        app.exit(0);
                    }
                    _ => {}
                })
                .on_tray_icon_event(|_app, event| {
                    match event {
                        TrayIconEvent::Click {
                            button: MouseButton::Left,
                            ..
                        } => {
                            // Left click - could show menu or toggle window
                            println!("Tray icon clicked");
                        }
                        _ => {}
                    }
                })
                .build(app)?;
            
            // Hide the main window on startup
            if let Some(window) = app.get_webview_window("main") {
                window.hide().unwrap();
            }

            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}