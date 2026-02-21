// Prevent console window in addition to Slint window in Windows release builds when, e.g., starting the app via file manager. Ignored on other platforms.
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod aur_api;
mod package_control;
mod search_callback;
mod terminal;

use std::{
    env,
    error::Error,
    sync::{Arc, Mutex},
};

use crate::{
    aur_api::Package, package_control::pkg_is_installed, search_callback::search_pkg_callback,
    terminal::terminal,
};

slint::include_modules!();

pub struct AppState {
    last_name: String,
    last_packages: Vec<Package>,
    package_info: PackageInfo,
}

impl AppState {
    fn new(last_name: String, last_packages: Vec<Package>, package_info: PackageInfo) -> Self {
        Self {
            last_name,
            last_packages,
            package_info: package_info,
        }
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let app_state_arc = Arc::new(Mutex::new(AppState::new(
        "".into(),
        vec![],
        PackageInfo::default(),
    )));
    let ui = AppWindow::new()?;
    let logic = ui.global::<Logic>();

    let ui_handle = ui.as_weak();
    let app_state_clone = Arc::clone(&app_state_arc);
    search_pkg_callback(&logic, ui_handle, app_state_clone);

    let app_state_clone2 = Arc::clone(&app_state_arc);
    let ui_handle2 = ui.as_weak();

    logic.on_pkg_selected_callback(move |pkg_name| {
        let ui = ui_handle2.unwrap();
        let logic = ui.global::<Logic>();

        let mut app_state = app_state_clone2.lock().unwrap();
        let pkg_name = pkg_name.to_string();

        let is_installed = match pkg_is_installed(&pkg_name) {
            Ok(is) => is,
            Err(_) => false,
        };
        app_state.package_info.is_installed = is_installed;
        logic.set_pkg_selected(PackageInfo {
            description: logic.get_pkg_selected().description,
            package_base: logic.get_pkg_selected().package_base,
            version: logic.get_pkg_selected().version,
            maintainer: logic.get_pkg_selected().maintainer,
            is_installed: is_installed,
        });
    });
    
    
    // terminal
    let ui_handle = ui.as_weak();
    terminal(ui_handle, &logic);

    ui.run()?;

    Ok(())
}
