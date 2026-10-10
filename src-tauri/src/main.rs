mod config;

use config::AppConfig;
use std::str::FromStr;
use tauri::Manager;
use tauri_plugin_global_shortcut::{Shortcut, ShortcutState};

fn main() {
    let app_config = AppConfig::load();
    let exit_shortcut = Shortcut::from_str(&app_config.exit_shortcut)
        .expect("failed to parse exit shortcut");
    let kiosk_url = app_config.url;

    tauri::Builder::default()
        .enable_macos_default_menu(false)
        .setup(move |app| {
            let mut config = app.config().app.windows[0].clone();
            config.url = tauri_utils::config::WebviewUrl::External(kiosk_url.parse().unwrap());
            
            let app_handle = app.handle().clone();
            let window = tauri::WebviewWindowBuilder::from_config(app, &config)?
                .on_new_window(move |url, _features| {
                    if let Some(w) = app_handle.get_webview_window("main") {
                        let _ = w.navigate(url);
                    }

                    tauri::webview::NewWindowResponse::Deny
                })
                .initialization_script(r#"
                    window.open = (url) => {
                        if (url) {
                            window.location.href = url;
                        }

                        return window;
                    };

                    document.addEventListener('click', (e) => {
                        const a = e.target.closest('a');
                        if (a && a.href) {
                            const target = (a.getAttribute('target') || a.target || '').toLowerCase().trim();
                            if (target === '_blank' || target === '_new' || target === 'blank') {
                                e.preventDefault();
                                window.location.href = a.href;
                            }
                        }
                    }, true);
                "#)
                .build()?;

            #[cfg(target_os = "macos")]
            {
                use objc2::MainThreadMarker;
                use objc2_app_kit::{NSApp, NSApplicationPresentationOptions};

                // 1. Strip the borders, titlebar, and traffic light controls
                let _ = window.set_decorations(false);
                let _ = window.set_resizable(false);
                let _ = window.set_always_on_top(true);

                // 2. Kill the sliding Dock and Menu bar behavior natively
                if let Some(mtm) = MainThreadMarker::new() {
                    let shared_app = NSApp(mtm);
                    let options = NSApplicationPresentationOptions::HideDock
                        | NSApplicationPresentationOptions::HideMenuBar
                        | NSApplicationPresentationOptions::DisableProcessSwitching // Blocks Cmd+Tab
                        | NSApplicationPresentationOptions::DisableForceQuit      // Blocks Cmd+Option+Esc
                        | NSApplicationPresentationOptions::DisableSessionTermination
                        | NSApplicationPresentationOptions::DisableHideApplication
                        | NSApplicationPresentationOptions::DisableAppleMenu;

                        shared_app.setPresentationOptions(options);
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
            Ok(())
        })
        .on_window_event(|window, event| {
            if let tauri::WindowEvent::Focused(false) = event {
                let _ = window.set_focus();
            }
        })
        .plugin(tauri_plugin_opener::init())
        .plugin(
            tauri_plugin_global_shortcut::Builder::new()
                .with_shortcut(exit_shortcut)
                .unwrap()
                .with_handler(move |app_handle, shortcut, event| {
                    if event.state == ShortcutState::Pressed && shortcut == &exit_shortcut {
                        app_handle.exit(0);
                    }
                })
                .build(),
        )
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
