// vou usar a crate reqwest para pesquisa no aur para aumentar a velocidade da pesquisa
// aur api: https://aur.archlinux.org/rpc/?v=5&type=search&arg=firefox

use std::{future::Future, task::Poll, time::Duration};

use reqwest::{StatusCode};
use serde::{Deserialize, Serialize};
use tokio::{sync::mpsc::OwnedPermit, time::sleep};

#[derive(Deserialize, Debug, Default)]
struct AurResponse {
    pub resultcount: u32,
    pub results: Vec<Package>,
    
    #[serde(rename = "type")]
    pub response_type: String,
    
    pub version: u8,
}



#[derive(Serialize, Deserialize, Debug)]
pub struct Package {
    #[serde(rename = "Description")]
    pub description: String, 
    
    #[serde(rename = "FirstSubmitted")]
    pub first_submitted: i32,
    
    #[serde(rename = "ID")]
    pub id: i32,
    
    #[serde(rename = "LastModified")]
    pub last_modified: i32,
    
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
    pub url: String,
    
    #[serde(rename = "URLPath")]
    pub url_path: String,
    
    #[serde(rename = "Version")]
    pub version: String,
}

impl Package {
    fn new(
        description: String,
        first_submitted: i32,
        id: i32,
        last_modified: i32,
        maintainer: Option<String>,
        name: String,
        num_votes: i32,
        out_of_date: Option<i64>,
        package_base: String,
        package_base_id: i32,
        popularity: f64,
        url: String,
        url_path: String,
        version: String        
    ) -> Self {
        Self { 
            description,
            first_submitted,
            id,
            last_modified,
            maintainer,
            name,
            num_votes,
            out_of_date,
            package_base,
            package_base_id,
            popularity,
            url,
            url_path,
            version
        }
    }
}


pub async fn search_pkg(pkg_name: &str) -> Result<Vec<Package>, reqwest::Error> {
    let mut last_name: String = "".to_string();
    
    // if last_name == pkg_name {
    //     return Ok(vec![]);
    //     println!("igual");
    // } else {
    //     println!("foi");
    //     last_name = pkg_name.into();
        
    // }
    
    let result: AurResponse = reqwest::get(format!("https://aur.archlinux.org/rpc/?v=5&type=search&arg={}", pkg_name))
        .await?
        .json::<AurResponse>()
        .await?;
    Ok(result.results)
    
}