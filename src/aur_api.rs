use std::{
    sync::{Arc, Mutex},
};

use serde::{Deserialize, Serialize};
use slint::{ToSharedString};

use crate::{AppState, PackageInfo};

#[derive(Deserialize, Debug, Default)]
struct AurResponse {
    pub resultcount: u32,
    pub results: Vec<Package>,

    #[serde(rename = "type")]
    pub response_type: String,

    pub version: u8,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Package {
    #[serde(rename = "Description")]
    pub description: Option<String>,

    #[serde(rename = "FirstSubmitted")]
    pub first_submitted: i64,

    #[serde(rename = "ID")]
    pub id: i32,

    #[serde(rename = "LastModified")]
    pub last_modified: i64,

    #[serde(rename = "Maintainer")]
    pub maintainer: Option<String>,

    #[serde(rename = "Name")]
    pub name: String,

    #[serde(rename = "NumVotes")]
    pub num_votes: i32,

    #[serde(rename = "OutOfDate")]
    pub out_of_date: Option<i64>,

    #[serde(rename = "PackageBase")]
    pub package_base: String,

    #[serde(rename = "PackageBaseID")]
    pub package_base_id: i32,

    #[serde(rename = "Popularity")]
    pub popularity: f64,

    #[serde(rename = "URL")]
    pub url: Option<String>,

    #[serde(rename = "URLPath")]
    pub url_path: String,

    #[serde(rename = "Version")]
    pub version: String,
}

pub fn aur_is_installed() -> bool {
    if let Ok(alpm) = alpm::Alpm::new("/", "/var/lib/pacman") {
        let local_db = alpm.localdb();

        match local_db.pkg("yay") {
            Ok(_) => true,
            Err(_) => false,
        }
    } else {
        false
    }
}

pub async fn search_aur_pkg(
    pkg_name: &str,
    app_state: &mut AppState,
) -> Result<Vec<Package>, reqwest::Error> {
    if app_state.last_name == pkg_name {
        Ok(app_state.last_packages.clone())
    } else {
        app_state.last_name = pkg_name.into();
        let result: AurResponse = reqwest::get(format!(
            "https://aur.archlinux.org/rpc/v5/search/{}",
            pkg_name
        ))
        .await?
        .json::<AurResponse>()
        .await?;

        Ok(result.results)
    }
}

pub async fn aur_db(app_state: Arc<Mutex<AppState>>, pkg_name: &str) -> Vec<PackageInfo> {
    let pkgs = {
        let mut state = app_state.lock().unwrap();
        search_aur_pkg(&pkg_name, &mut state).await
    };

    if let Ok(pkgs) = pkgs {
        let mut pkgs_info: Vec<PackageInfo> = vec![];
        let mut packages_info: PackageInfo;
        let default = String::from("NA");

        for pkg in &pkgs {
            let description = &pkg.description.as_ref().unwrap_or_else(|| &default);
            let maintainer = &pkg.maintainer.as_ref().unwrap_or_else(|| &default);

            packages_info = PackageInfo {
                package_base: pkg.package_base.clone().into(),
                version: pkg.version.clone().into(),
                description: description.to_shared_string(),
                maintainer: maintainer.to_shared_string(),
                is_installed: false,
                repo: "Aur".to_shared_string(),
            };

            pkgs_info.push(packages_info);
        }

        app_state.lock().unwrap().last_packages = pkgs;

        pkgs_info
    } else {
        Vec::new()
    }
}
