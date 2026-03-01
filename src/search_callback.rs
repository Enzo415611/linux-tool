use std::{
    sync::{Arc, Mutex}, time::Duration
};

use slint::{ComponentHandle, ModelRc, Weak};

use crate::{AppState, AppWindow, Logic, aur_api::{aur_db, aur_is_installed}, pacman::pacman_db};

pub fn search_pkg_callback(
    logic: &Logic<'_>,
    ui_handle: Weak<AppWindow>,
    app_state_arc: Arc<Mutex<AppState>>,
) {
    logic.on_search_pkg({
        move |pkg_name| {
            let handle = ui_handle.unwrap();
            let app_state = Arc::clone(&app_state_arc);
            // pacman    
            slint::spawn_local(async_compat::Compat::new(async move {
                let logic = handle.global::<Logic>();
                _ = tokio::time::sleep(Duration::from_millis(1000)).await;
                
                
                let aur_is_installed = aur_is_installed();
                
                let pacman_pkg = pacman_db(pkg_name.as_str());
                
                if aur_is_installed {
                    let aur_pkg = aur_db(app_state, &pkg_name).await;
                
                    for pkg in aur_pkg {
                        pacman_pkg.push(pkg);
                    }
                }
                
                
                logic.set_loading_pkgs(false);                    
                logic.set_pkgs_info(ModelRc::from(pacman_pkg));
            }))
            .unwrap();
        }
    });
}
