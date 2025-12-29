#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]

mod state;
mod modules;
mod downloads;
mod updater;
mod commands;

use tauri::{
    generate_handler, Builder, Manager, WindowUrl, CustomMenuItem, SystemTray, SystemTrayMenu,
    window::{WindowBuilder, TitleBarStyle},
};

fn main() {
    let app = Builder::default()
        .system_tray(SystemTray::new().with_menu(SystemTrayMenu::new()))
        .invoke_handler(generate_handler![
            commands::new_tab,
            commands::close_tab,
            commands::select_tab,
            commands::duplicate_tab,
            commands::navigate,
            commands::toggle_incognito,
            commands::pin_tab,
            commands::unpin_tab,
            commands::save_state,
            commands::load_state,
            commands::get_downloads,
            commands::cancel_download,
            commands::pause_download,
            commands::resume_download,
            commands::get_history,
            commands::clear_history,
            commands::search_modules,
            commands::install_module,
            commands::uninstall_module,
            commands::enable_module,
            commands::disable_module,
            commands::check_updates,
            commands::apply_update,
            commands::get_keybindings,
            commands::set_keybinding,
            commands::evaluate_expression,
            commands::search_google,
            commands::toggle_adblock,
            commands::toggle_vpn,
            commands::set_theme,
        ])
        .setup(|app| {
            // Initialize data directories (relative to executable)
            let exec_dir = std::env::current_exe()
                .unwrap()
                .parent()
                .unwrap()
                .to_path_buf();
            
            let data_dir = exec_dir.join("data");
            let profiles_dir = exec_dir.join("profiles");
            let modules_dir = exec_dir.join("modules");
            let downloads_dir = exec_dir.join("downloads");
            
            let _ = std::fs::create_dir_all(&data_dir);
            let _ = std::fs::create_dir_all(&profiles_dir);
            let _ = std::fs::create_dir_all(&modules_dir);
            let _ = std::fs::create_dir_all(&downloads_dir);
            
            // Create main window
            let main_window = WindowBuilder::new(
                app,
                "main",
                WindowUrl::App("/index.html".into()),
            )
            .title("HyprBrowser")
            .decorations(false)
            .transparent(true)
            .inner_size(1280.0, 720.0)
            .min_inner_size(800.0, 600.0)
            .title_bar_style(TitleBarStyle::Transparent)
            .build()?;
            
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

