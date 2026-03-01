use std::{fs, rc::Rc};

use alpm::{Alpm, SigLevel};
use slint::{ToSharedString, VecModel};

use crate::PackageInfo;

pub fn pacman_db(pkg_name: &str) -> Rc<VecModel<PackageInfo>> {
    let alpm_handle = Alpm::new("/", "/var/lib/pacman");
    let mut repos: Vec<String> = Vec::new();
    let mut pkgs: Vec<PackageInfo> = Vec::new();
    
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
            _=al.register_syncdb(repo, SigLevel::USE_DEFAULT);
        }
        
        let sync_dbs = al.syncdbs();
        
        for db in sync_dbs {
            if let Ok(pkg) = db.pkg(pkg_name) {
                let package: PackageInfo;
                
                let desc = pkg.desc().unwrap_or_else(|| "NA");
                
                let maintainer = pkg.packager().unwrap_or_else(|| "NA");
                
                package = PackageInfo {
                    package_base: pkg.name().to_shared_string(),
                    version: pkg.version().to_shared_string(),
                    description: desc.to_shared_string(),
                    maintainer: maintainer.to_shared_string(),
                    is_installed: false,
                    repo: db.name().to_shared_string()
                };
                pkgs.push(package);
                
            }
        }
    }
    
    Rc::new(VecModel::from(pkgs))
}