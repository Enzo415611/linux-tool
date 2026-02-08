// Prevent console window in addition to Slint window in Windows release builds when, e.g., starting the app via file manager. Ignored on other platforms.
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::{error::Error, io, process::{Command, Stdio}, rc::Rc, thread};

use raur::{Package, Raur};
use slint::{ComponentHandle, Model, ModelRc, SharedString, SharedVector, ToSharedString, VecModel};



slint::include_modules!();
#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let ui = AppWindow::new()?;
    let logic = ui.global::<Logic>();
    
    let ui_handle = ui.as_weak();
    logic.on_search_app({
        move |app_name| {
            let handle = ui_handle.unwrap();

            slint::spawn_local(async_compat::Compat::new(async move {
                let logic = handle.global::<Logic>(); 

                let pkgs = search_app(&app_name.to_string()).await;
                
                if let Ok(pkgs) = pkgs {
                    let mut pkgs_shared: Vec<SharedString> = vec![];
                    for pkg in pkgs {
                        pkgs_shared.push(pkg.package_base.to_shared_string());
                    }
                    
                    let the_model = Rc::new(VecModel::from(pkgs_shared));
                    logic.set_apps_list(ModelRc::from(the_model));
                    
                }
                
                println!("{:?}", app_name.clone());
            })).unwrap();  
        }
    });
    
    // ui.on_request_increase_value({
    //     let ui_handle = ui.as_weak();
    //     move || {
    //         let ui = ui_handle.unwrap();
    //         ui.set_counter(ui.get_counter() + 1);
    //     }
    // });

    ui.run()?;

    Ok(())
}

async fn search_app(app_name: &String) -> Result<Vec<Package>, raur::Error> {
    let raur = raur::Handle::new();
    let pkgs = raur.search(app_name).await?;
    for pkg in &pkgs {
        println!("{}", pkg.package_base);
    }    
    Ok(pkgs)
}