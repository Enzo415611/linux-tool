use std::{
    io::{self},
    process::{Command, Output},
};


pub fn pkg_is_installed(pkg_name: String) -> Result<Output, io::Error> {
    let out = Command::new("yay")
        .args(["-Q", pkg_name.as_str()])
        .output()?;
    
    println!("{}",String::from_utf8_lossy(&out.stdout));
    Ok(out)
}

//pub fn uninstall() {}
