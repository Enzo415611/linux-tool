// Prevent console window in addition to Slint window in Windows release builds when, e.g., starting the app via file manager. Ignored on other platforms.
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod package_control;
mod aur_api;
mod terminal;
mod search_callback;


use std::{env, error::Error, sync::{Arc, Mutex}};

use crate::{aur_api::Package, package_control::{pkg_is_installed}, search_callback::search_pkg_callback, terminal::terminal};

slint::include_modules!();


pub struct AppState {
    last_name: String,
    last_packages: Vec<Package>
}

impl AppState {
    fn new(last_name: String, last_packages: Vec<Package>) -> Self {
        Self {
            last_name,
            last_packages,
        }
    }
}


#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let app_state_arc = Arc::new(Mutex::new(AppState::new("".into(), vec![])));    
    let ui = AppWindow::new()?;
    let logic = ui.global::<Logic>();
    
    
    let ui_handle = ui.as_weak();
    search_pkg_callback(&logic, ui_handle, app_state_arc);
    
    

    logic.on_pkg_selected_callback(|pkg_name| {
        let pkg_name =  pkg_name.to_string();
        _=pkg_is_installed(pkg_name);
    });

    // terminal
    let ui_handle = ui.as_weak();
    terminal(ui_handle, &logic);
   
    ui.run()?;

    Ok(())
}


