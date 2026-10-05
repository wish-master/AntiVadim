use tauri::Manager;

#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

fn main() {
    tauri::Builder::default()
        .enable_macos_default_menu(false)
        .setup(|app| {
            #[cfg(target_os = "macos")]
            {
                use objc2::MainThreadMarker;
                use objc2_app_kit::{NSApp, NSApplicationPresentationOptions};

                if let Some(window) = app.get_webview_window("main") {
                    // 1. Strip the borders, titlebar, and traffic light controls
                    let _ = window.set_decorations(false);
                    let _ = window.set_resizable(false);
                    let _ = window.set_always_on_top(true);

                    // 2. Kill the sliding Dock and Menu bar behavior natively
                    unsafe {
                        if let Some(mtm) = MainThreadMarker::new() {
                            let shared_app = NSApp(mtm);
                            let options = NSApplicationPresentationOptions::HideDock
                                | NSApplicationPresentationOptions::HideMenuBar
                                | NSApplicationPresentationOptions::DisableProcessSwitching // Blocks Cmd+Tab
                                | NSApplicationPresentationOptions::DisableForceQuit;     // Blocks Cmd+Option+Esc

                            shared_app.setPresentationOptions(options);
                        }
                    }

                    // 3. Size a clean, borderless canvas to the physical screen boundaries
                    if let Ok(Some(monitor)) = window.current_monitor() {
                        let size = monitor.size();
                        let _ = window.set_size(tauri::Size::Physical(*size));
                        let _ = window.set_position(tauri::Position::Physical(tauri::PhysicalPosition { x: 0, y: 0 }));
                    }

                    let _ = window.show();
                    let _ = window.set_focus();
                }
            }
            Ok(())
        })
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![greet])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
