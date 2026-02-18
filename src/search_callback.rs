use std::{rc::Rc, sync::{Arc, Mutex}, time::Duration};

use slint::{ComponentHandle, ModelRc, VecModel, Weak};

use crate::{AppState, AppWindow, Logic, PackagesInfo, aur_api::search_pkg};

pub fn search_pkg_callback(logic: &Logic<'_>, ui_handle: Weak<AppWindow>, app_state_arc: Arc<Mutex<AppState>>) {
    
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
                
                println!("{:?}", pkgs);
                
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
}