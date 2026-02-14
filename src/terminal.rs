use std::{env, sync::{Arc, Mutex}, thread};

use portable_pty::{CommandBuilder, PtyPair, PtySize, native_pty_system};
use slint::{ComponentHandle, ToSharedString, Weak};

use crate::{AppWindow, Logic};

pub fn terminal(ui: Weak<AppWindow>, logic: &Logic<'_>) {


    let pty_system = native_pty_system();
    let pair = pty_system.openpty(PtySize {
        rows: 24,
        cols: 80,
        pixel_width: 0,
        pixel_height: 0,
    }).unwrap();

    let writer_arc = Arc::new(Mutex::new(pair.master.take_writer().unwrap()));
    let writer_clone = writer_arc.clone();
    

    // button install pkg callback
    logic.on_install_pkg(move |pkg_name| {
        let mut w = writer_clone.lock().unwrap();
        _ = writeln!(w, "yay -S {}", pkg_name);
    });



    let shell = env::var("SHELL").unwrap_or("/bin/bash".into());

    let mut cmd = CommandBuilder::new(shell);
    cmd.arg("-l");
    pair.slave.spawn_command(cmd).unwrap();
    let mut reader = pair.master.try_clone_reader().unwrap();





    thread::spawn(move || {
        let mut buffer = [0u8; 1024];

        
        while let Ok(n) = reader.read(&mut buffer) {
            if n == 0 {break;}

            let output = String::from_utf8_lossy(&buffer[..n]).to_string();
            let clear_ansi_output = strip_ansi_escapes::strip_str(output);
            

            let ui_handle_clone = ui.clone();

            // slint update ui
            slint::invoke_from_event_loop(move || {
                if let Some(ui) = ui_handle_clone.upgrade() {
                    let logic = ui.global::<Logic>();

                    let current = logic.get_pkgs_info();
                    logic.set_terminal_output(format!("{:?} {}", current, clear_ansi_output).to_shared_string());
                }
            }).unwrap();
        }

    });
}