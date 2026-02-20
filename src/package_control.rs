use std::{
    io::{self},
    process::Command,
};

pub fn pkg_is_installed(pkg_name: &String) -> Result<bool, io::Error> {
    let out = Command::new("pacman")
        .args(["-Qq", pkg_name.as_str()])
        .output()?;

    let out_str = String::from_utf8_lossy(&out.stdout).to_string();
        
    if pkg_name.trim() == out_str.trim() {
        Ok(true)
    } else {
        Ok(false)
    }
}
