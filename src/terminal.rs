use std::{
    env,
    sync::{Arc, Mutex},
    thread,
};

use portable_pty::{native_pty_system, CommandBuilder, PtySize};
use slint::{ComponentHandle, ToSharedString, Weak};

use crate::{AppWindow, Logic};

pub fn terminal(ui: Weak<AppWindow>, logic: &Logic<'_>) {
    let pty_system = native_pty_system();
    let pair = pty_system
        .openpty(PtySize {
            rows: 24,
            cols: 80,
            pixel_width: 0,
            pixel_height: 0,
        })
        .unwrap();

    let writer_arc = Arc::new(Mutex::new(pair.master.take_writer().unwrap()));

    // button pkg callback install
    let writer_clone = writer_arc.clone();
    let ui_handle = ui.clone();
    
    logic.on_install_pkg(move |pkg_name| {
        let mut w = writer_clone.lock().unwrap();
        let ui = ui_handle.unwrap();
        let logic = ui.global::<Logic>();
        logic.set_terminal_output("Install".to_shared_string());
        _ = writeln!(w, "yay -S {}", pkg_name);
    });
    
    
    let writer_clone2 = writer_arc.clone();
    let ui_handle2 = ui.clone();
    logic.on_uninstall_pkg(move |pkg_name| {
        println!("Uninstall");
        let mut w = writer_clone2.lock().unwrap();
        let ui = ui_handle2.unwrap();
        let logic = ui.global::<Logic>();
        logic.set_terminal_output("Uninstall".to_shared_string());
        _ = writeln!(w, "yay -R {}", pkg_name);
    });

    let writer_clone1 = writer_arc.clone();

    logic.on_send_terminal_input(move |inp| {
        let mut w = writer_clone1.lock().unwrap();
        _ = writeln!(w, "{}", inp);
    });

    let shell = env::var("SHELL").unwrap_or("/bin/bash".into());

    let mut cmd = CommandBuilder::new(shell);
    cmd.arg("-l");
    pair.slave.spawn_command(cmd).unwrap();
    let mut reader = pair.master.try_clone_reader().unwrap();

    thread::spawn(move || {        
        let mut buffer = [0u8; 1024];
        let mut terminal_buffer = String::new();
        let mut log = String::new();

        
        while let Ok(n) = reader.read(&mut buffer) {
            
            if n == 0 {
                break;
            }

            let output = String::from_utf8_lossy(&buffer[..n]).to_string();
            let clear_output = strip_ansi_escapes::strip_str(output);
            terminal_buffer.push_str(&clear_output);
            log.push_str(&clear_output);
            
            let display_text = terminal_buffer.clone(); 
            let log = log.clone();
            
            terminal_buffer.clear();
            
            // slint update ui
            let ui_handle3 = ui.clone();
            slint::invoke_from_event_loop(move || {
                if let Some(ui) = ui_handle3.upgrade() {
                    let logic = ui.global::<Logic>();
                    let input = logic.get_terminal_input().to_string();
                    logic.set_log(log.into());
                    if input == "clear".to_string() {
                        logic.set_terminal_output("".to_shared_string());
                    }    
                    
                    logic.invoke_append_terminal_out(display_text.into());
                }
            })
            .unwrap();            
        }
        
        //println!("{}", app_state.log);
    });
}
