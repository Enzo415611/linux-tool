// Prevent console window in addition to Slint window in Windows release builds when, e.g., starting the app via file manager. Ignored on other platforms.
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::{error::Error, process::{Command, Stdio}, thread};

slint::include_modules!();

fn main() -> Result<(), Box<dyn Error>> {
    let ui = AppWindow::new()?;
    
    ui.on_installApp({
        let _ui_handle = ui.as_weak();
        move || {
            thread::spawn(|| {
               install_app("code".to_string()); 
            });                        
        }
    });
    
    // ui.on_request_increase_value({
    //     let ui_handle = ui.as_weak();
    //     move || {
    //         let ui = ui_handle.unwrap();
    //         ui.set_counter(ui.get_counter() + 1);
    //     }
    // });

    ui.run()?;

    Ok(())
}


fn install_app(app_name: String) {
    let status = Command::new("pkexec")
        .args(["yay", "-S", "--noconfirm" ,app_name.as_str()])
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .status();
    
    if let Ok(s) = status {
        if s.success() {
            println!("sucesso")
        }
    }    
}