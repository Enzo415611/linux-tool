// Prevent console window in addition to Slint window in Windows release builds when, e.g., starting the app via file manager. Ignored on other platforms.
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod package_control;
mod aur_api;


use std::{error::Error, rc::Rc, time::Duration};

use slint::{ModelRc, SharedString, ToSharedString, VecModel};

use crate::{aur_api::search_pkg, package_control::install_pkg};

slint::include_modules!();



#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let ui = AppWindow::new()?;
    let logic = ui.global::<Logic>();

    let ui_handle = ui.as_weak();
    
    logic.on_search_pkg({
        move |app_name| {
            let handle = ui_handle.unwrap();
            
            slint::spawn_local(async_compat::Compat::new(async move {
                let logic = handle.global::<Logic>();
                _=tokio::time::sleep(Duration::from_millis(700)).await;

                let pkgs = search_pkg(&app_name).await;
                
                if let Ok(pkgs) = pkgs {
                    let mut pkgs_info: Vec<PackagesInfo> = vec![];
                    let mut packages_info: PackagesInfo;
                    for pkg in pkgs {
                        let description = pkg.description;

                        let maintainer = match pkg.maintainer {
                            Some(ma) => ma,
                            None => String::from("NA"),
                        };

                        packages_info = PackagesInfo {
                            package_base: pkg.package_base.into(),
                            version: pkg.version.into(),
                            description: description.into(),
                            maintainer: maintainer.into(),
                        };

                        pkgs_info.push(packages_info);
                    }

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