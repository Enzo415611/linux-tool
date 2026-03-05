// Prevent console window in addition to Slint window in Windows release builds when, e.g., starting the app via file manager. Ignored on other platforms.
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod aur_api;
mod package_control;
mod pacman;
mod search_callback;
mod terminal;
mod config;

use std::{
    env,
    error::Error,
    sync::{Arc, Mutex}, thread, time::Duration,
};

use slint::ComponentHandle;

use crate::{
    aur_api::{Package, aur_is_installed}, config::{AppConfig, load_config}, package_control::pkg_is_installed, search_callback::search_pkg_callback, terminal::terminal
};

slint::include_modules!();


pub struct AppState {
    last_name: String,
    last_packages: Vec<Package>,
    package_info: PackageInfo,
    aur_is_installed: bool
}

impl AppState {
    fn new(
        last_name: String,
        last_packages: Vec<Package>,
        package_info: PackageInfo,
        aur_is_installed: bool
    ) -> Self {
        Self {
            last_name,
            last_packages,
            package_info,
            aur_is_installed
        }
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    const VERSION: &str = env!("CARGO_PKG_VERSION");
    
    let app_state_arc = Arc::new(Mutex::new(AppState::new(
        "".into(),
        vec![],
        PackageInfo::default(),
        aur_is_installed()
    )));


    let cfg = load_config()?;
    println!("config: {:?}", cfg);
    let ui = AppWindow::new()?;
    let logic = ui.global::<Logic>();
    let theme = ui.global::<Theme>();
    
    logic.set_app_version(VERSION.into());
    theme.set_dark_mode(cfg.app_theme);
    // salva o tema em /.config/linux-tool/config.toml
    theme.on_dark_mode_callback(move |current_theme|{
        _=confy::store("linux-tool", "config", AppConfig {
            app_theme: current_theme
        });
    });

    let aur_is_installed = {
        app_state_arc.lock().unwrap().aur_is_installed
    };

    logic.set_aur_is_installed(aur_is_installed);

    let ui_handle = ui.as_weak();
    let app_state_clone = Arc::clone(&app_state_arc);
    search_pkg_callback(&logic, ui_handle, app_state_clone);

    let app_state_clone2 = Arc::clone(&app_state_arc);
    let ui_handle2 = ui.as_weak();

    logic.on_pkg_selected_callback(move |_| {
        let ui = ui_handle2.unwrap();
        let logic = ui.global::<Logic>();

        let mut app_state = app_state_clone2.lock().unwrap();

        app_state.package_info = PackageInfo {
            description: logic.get_pkg_selected().description,
            package_base: logic.get_pkg_selected().package_base,
            version: logic.get_pkg_selected().version,
            maintainer: logic.get_pkg_selected().maintainer,
            repo: logic.get_pkg_selected().repo,
            is_installed: logic.get_pkg_selected().is_installed,
        };


        logic.set_pkg_selected(PackageInfo {
            description: logic.get_pkg_selected().description,
            package_base: logic.get_pkg_selected().package_base,
            version: logic.get_pkg_selected().version,
            maintainer: logic.get_pkg_selected().maintainer,
            is_installed: logic.get_pkg_selected().is_installed,
            repo: logic.get_pkg_selected().repo,
        });
    });

    // terminal
    let ui_handle3 = ui.as_weak();
    terminal(ui_handle3, &logic);

    let app_state_clone3 = Arc::clone(&app_state_arc);
    let ui_handle4 = ui.as_weak();
    thread::spawn(move|| {
        loop {
             let is_installed = {
                 let app_state = app_state_clone3.lock().unwrap();
                 let pkg_name = app_state.package_info.package_base.to_string();
                 let is_installed = pkg_is_installed(&pkg_name).unwrap_or_else(|_| false);
                 is_installed
             };


            let ui_handle = ui_handle4.clone();
            thread::sleep(Duration::from_millis(300));

            slint::invoke_from_event_loop(move || {
                if let Some(ui) = ui_handle.upgrade() {
                    let logic = ui.global::<Logic>();
                    let pkg = logic.get_pkg_selected();
                    logic.set_pkg_selected(PackageInfo {
                        package_base: pkg.package_base,
                        description: pkg.description,
                        maintainer: pkg.maintainer,
                        version: pkg.version,
                        repo: pkg.repo,
                        is_installed: is_installed
                    });
                }
            }).unwrap();
        }

    });


    logic.on_open_git(|| {
        _=open::that("https://github.com/Enzo415611/Enzo415611").unwrap();
    });

    ui.run()?;
    Ok(())
}
