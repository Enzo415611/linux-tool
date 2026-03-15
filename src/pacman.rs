use std::{
    fs,
    rc::Rc,
    sync::{Arc, Mutex},
};

use alpm::{Alpm, SigLevel};
use slint::{ToSharedString, VecModel};

use crate::{AppState, PackageInfo};

pub struct PacmanPackage {
    pub package_base: String,
    pub version: String,
    pub description: String,
    pub maintainer: String,
    pub is_installed: bool,
    pub repo: String,
}

impl PacmanPackage {
    fn new(
        package_base: String,
        version: String,
        description: String,
        maintainer: String,
        is_installed: bool,
        repo: String,
    ) -> Self {
        Self {
            package_base,
            version,
            description,
            maintainer,
            is_installed,
            repo,
        }
    }
}

pub fn pacman_db(pkg_name: &str, app_state: Arc<Mutex<AppState>>) -> Rc<VecModel<PackageInfo>> {
    let alpm_handle = Alpm::new("/", "/var/lib/pacman");
    let mut repos: Vec<String> = Vec::new();
    let mut pkgs: Vec<PackageInfo> = Vec::new();
    let last_name = {
        let app_state = app_state.lock().unwrap();
        app_state.last_name.clone()
    };
    let mut app_state = app_state.lock().unwrap();

    if pkg_name == last_name {
        let last_pacman_packages = { &app_state.last_pacman_packages };
        for pkg in last_pacman_packages {
            pkgs.push(PackageInfo {
                package_base: pkg.package_base.to_shared_string(),
                description: pkg.description.to_shared_string(),
                version: pkg.version.to_shared_string(),
                maintainer: pkg.maintainer.to_shared_string(),
                repo: pkg.repo.to_shared_string(),
                is_installed: pkg.is_installed
            });
        }
    } else {
        if let Ok(al) = alpm_handle {
            if let Ok(entries) = fs::read_dir("/var/lib/pacman/sync") {
                for entry in entries.flatten() {
                    let path = entry.path();
                    if let Some(extension) = path.extension() {
                        if extension == "db" {
                            repos.push(path.file_stem().unwrap().display().to_string());
                        }
                    }
                }
            }

            for repo in repos {
                _ = al.register_syncdb(repo, SigLevel::USE_DEFAULT);
            }

            let sync_dbs = al.syncdbs();
            
            let mut package: PackageInfo;
            
            for db in sync_dbs {
                if let Ok(pkg) = db.pkg(pkg_name) {

                    let desc = pkg.desc().unwrap_or_else(|| "NA");
                    let maintainer = pkg.packager().unwrap_or_else(|| "NA");

                    package = PackageInfo {
                        package_base: pkg.name().to_shared_string(),
                        version: pkg.version().to_shared_string(),
                        description: desc.to_shared_string(),
                        maintainer: maintainer.to_shared_string(),
                        is_installed: false,
                        repo: db.name().to_shared_string(),
                    };

                    pkgs.push(package);

                    app_state
                        .last_pacman_packages
                        .push(PacmanPackage::new(
                            pkg.name().to_string(),
                            pkg.version().to_string(),
                            desc.to_string(),
                            maintainer.to_string(),
                            false,
                            db.name().to_string(),
                        ));
                }
            }
        }
    }

    Rc::new(VecModel::from(pkgs))
}