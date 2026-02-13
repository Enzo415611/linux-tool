use std::{io::{self, Stdin}, process::{Child, Command, Output, Stdio}};

pub fn install_pkg(pkg_name: String) -> Result<Child, io::Error> {
    let output = Command::new("pkexec")
        .args(["yay", "-S", "--noconfirm" ,pkg_name.as_str()])
        .spawn()?;    
    Ok(output)
}


pub fn pkg_is_installed(pkg_name: String) -> Result<Output ,io::Error> {
    let out = Command::new("yay")
        .args(["-Q", pkg_name.as_str()])
        .stdout(Stdio::inherit())
        .output()?;
    
    println!("{:?}", out.stdout);
    
    Ok(out)
}

pub fn uninstall() {
    
}

