// vou usar a crate reqwest para pesquisa no aur para aumentar a velocidade da pesquisa
// aur api: https://aur.archlinux.org/rpc/?v=5&type=search&arg=firefox

use std::io;

use reqwest::{StatusCode};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
struct AurResponse {
    results: Vec<Package>
}

#[derive(Serialize, Deserialize, Debug)]
struct Package {
    description: Option<String>,
    name: String,
    version: String,
    maintainer: Option<String>,
    co_maintainers: Option<String>,
    depends: String
}


pub async fn search_pkg_teste(pkg_name: &str) -> Result<StatusCode, reqwest::Error> {
    let body: AurResponse = reqwest::get(format!("https://aur.archlinux.org/rpc/?v=5&type=search&arg={}", pkg_name))
        .await?
        .json::<AurResponse>()
        .await?;
    
    for pkg in body.results {
        println!("{:?}", pkg);
    }
    
    
    Ok(StatusCode::OK)

}