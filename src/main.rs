// Prevent console window in addition to Slint window in Windows release builds when, e.g., starting the app via file manager. Ignored on other platforms.
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod package_control;
mod aur_api;


use std::{error::Error, rc::Rc, sync::{Arc, Mutex}, time::Duration};

use slint::{ModelRc, VecModel};

use crate::{aur_api::{Package, search_pkg}, package_control::install_pkg};

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
    
    logic.on_search_pkg({
        move |app_name| {
            let handle = ui_handle.unwrap();
            let app_state = Arc::clone(&app_state_arc);
            
            slint::spawn_local(async_compat::Compat::new(async move {
                let logic = handle.global::<Logic>();
                _=tokio::time::sleep(Duration::from_millis(700)).await;
                
                let pkgs = {
                    let mut state = app_state.lock().unwrap();
                    search_pkg(&app_name, &mut state).await
                };
                
                if let Ok(pkgs) = pkgs {
                    let mut pkgs_info: Vec<PackagesInfo> = vec![];
                    let mut packages_info: PackagesInfo;

                    for pkg in &pkgs {
                        let description = match &pkg.description {
                            Some(dis) => dis,
                            None => &String::from("NA"),
                        };

                        let maintainer = match &pkg.maintainer {
                            Some(ma) => ma,
                            None => &String::from("NA"),
                        };

                        packages_info = PackagesInfo {
                            package_base: pkg.package_base.clone().into(),
                            version: pkg.version.clone().into(),
                            description: description.into(),
                            maintainer: maintainer.into(),
                        };

                        pkgs_info.push(packages_info);
                    }

                    app_state.lock().unwrap().last_packages = pkgs;

                    let the_model = Rc::new(VecModel::from(pkgs_info));
                    logic.set_pkgs_info(ModelRc::from(the_model));
                }
            }))
            .unwrap();
        }
    });
    
    logic.on_install_pkg(|pkg_name| {
        println!("{:?}", install_pkg(pkg_name.into()));
    });

    ui.run()?;

    Ok(())
}


