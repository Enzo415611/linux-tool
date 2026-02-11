use std::{io, process::{Child, Command}};

pub fn install_pkg(pkg_name: String) -> Result<Child, io::Error> {
    let output = Command::new("pkexec")
        .args(["yay", "-S", "--noconfirm" ,pkg_name.as_str()])
        .spawn()?;    
    Ok(output)
}
