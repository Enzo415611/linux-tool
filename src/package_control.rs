use std::{io, process::{Child, Command}};

use raur::{Package, Raur};

// pub async fn search_pkg(app_name: &String) -> Result<Vec<Package>, raur::Error> {
//     let raur = raur::Handle::new();
//     let pkgs = raur.search(app_name).await?;
//     Ok(pkgs)
// }


pub fn install_pkg(pkg_name: String) -> Result<Child, io::Error> {
    let output = Command::new("pkexec")
        .args(["yay", "-S", "--noconfirm" ,pkg_name.as_str()])
        .spawn()?;    
    Ok(output)
}
