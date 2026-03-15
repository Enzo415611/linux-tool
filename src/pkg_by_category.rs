use std::{
    rc::Rc,
    sync::{Arc, Mutex},
};

use slint::{ComponentHandle, Model, ModelRc, ToSharedString, VecModel};

use crate::{
    aur_api::search_aur_pkg, package_control::pkg_is_installed, pacman::pacman_db, AppState,
    AppWindow, Logic, PackageInfo,
};

const CATEGORIES: [(&str, &[&str]); 8] = [
    ("Games", &["steam", "wine", "lutris", "mangohud"]),
    ("Development", &["zed", "code", "neovim", "vim", "docker"]),
    ("Communication", &["discord", "telegram-desktop"]),
    ("Graphics", &["gimp", "blender"]),
    ("Internet", &["brave", "firefox", "chromium", "konqueror"]),
    (
        "Office",
        &["libreoffice-still", "libreoffice-fresh", "onlyoffice-bin"],
    ),
    (
        "Multimedia",
        &["vlc", "mpv", "obs-studio", "ffmpeg", "spotify-launcher"],
    ),
    (
        "DriversHardware",
        &[
            "mesa",
            "mesa-utils",
            "nvidia-open",
            "nvidia-utils",
            "nvidia-settings",
            "lact",
            "openrgb",
            "liquidctl",
        ],
    ),
];

pub fn pkg_by_category(category: &str, ui: AppWindow, app_state: Arc<Mutex<AppState>>) {
    let Some((_, pkgs_names)) = CATEGORIES.iter().find(|&&(cat, _)| cat == category) else {
        return;
    };

    let ui_weak = ui.as_weak();

    slint::spawn_local(async move {
        let ui = ui_weak.unwrap();
        let logic = ui.global::<Logic>();

        let yay_is_installed = logic.get_yay_is_installed();
        let mut all_pkgs: Vec<PackageInfo> = Vec::with_capacity(64);

        for &pkg in *pkgs_names {
            for pkg in pacman_db(pkg, app_state.clone()).iter() {
                if all_pkgs
                    .iter()
                    .any(|pk| pk.package_base == pkg.package_base)
                {
                } else {
                    all_pkgs.push(pkg);
                }
            }

            if yay_is_installed {
                if let Ok(pkgs) = search_aur_pkg(pkg, app_state.clone(), true).await {
                    for pkg in pkgs {
                        if all_pkgs
                            .iter()
                            .any(|pk| pk.package_base == pkg.package_base)
                        {
                        } else {
                            all_pkgs.push(PackageInfo {
                                package_base: pkg.package_base.to_shared_string(),
                                description: pkg
                                    .description
                                    .unwrap_or_else(|| "NA".into())
                                    .to_shared_string(),
                                maintainer: pkg
                                    .maintainer
                                    .unwrap_or_else(|| "NA".into())
                                    .to_shared_string(),
                                version: pkg.version.to_shared_string(),
                                repo: "Aur".to_shared_string(),
                                is_installed: pkg_is_installed(&pkg.package_base).unwrap_or_else(|_| false),
                            });
                        }
                    }
                }
            }
        }

        let model = ModelRc::new(Rc::new(VecModel::from(all_pkgs)));
        logic.set_pkgs_info_category(model);
    })
    .unwrap();
}
