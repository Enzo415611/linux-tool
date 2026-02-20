// vou usar a crate reqwest para pesquisa no aur para aumentar a velocidade da pesquisa
// aur api: https://aur.archlinux.org/rpc/?v=5&type=search&arg=firefox

use serde::{Deserialize, Serialize};

use crate::AppState;

#[derive(Deserialize, Debug, Default)]
struct AurResponse {
    pub resultcount: u32,
    pub results: Vec<Package>,

    #[serde(rename = "type")]
    pub response_type: String,

    #[warn(unused)]
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

pub async fn search_pkg(
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
