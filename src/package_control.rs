use std::{
    io::{self},
    process::{Child, Command, Output},
};

pub fn install_pkg(pkg_name: String) -> Result<Child, io::Error> {
    let output = Command::new("pkexec")
        .args(["yay", "-S", pkg_name.as_str()])
        .spawn()?;
    
    
    println!("{:?}", output);
    Ok(output)
}

pub fn pkg_is_installed(pkg_name: String) -> Result<Output, io::Error> {
    let out = Command::new("yay")
        .args(["-Q", pkg_name.as_str()])
        .output()?;
    
    println!("{}",String::from_utf8_lossy(&out.stdout));
    Ok(out)
}

pub fn uninstall() {}
