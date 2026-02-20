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
        logic.set_terminal_output("".to_shared_string());
        _ = writeln!(w, "yay -S {}", pkg_name);
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
        
        
        while let Ok(n) = reader.read(&mut buffer) {
            
            if n == 0 {
                break;
            }

            let output = String::from_utf8_lossy(&buffer[..n]).to_string();
            let clear_output = strip_ansi_escapes::strip_str(output);
            terminal_buffer.push_str(&clear_output);

            let ui_handle_clone = ui.clone();
            let display_text = terminal_buffer.clone();
            terminal_buffer.clear();
            // slint update ui
            slint::invoke_from_event_loop(move || {
                if let Some(ui) = ui_handle_clone.upgrade() {
                    let logic = ui.global::<Logic>();
                    let input = logic.get_terminal_input().to_string();

                    if input == "clear".to_string() {
                        logic.set_terminal_output("".to_shared_string());
                    }    
                    
                    logic.invoke_append_terminal_out(display_text.into());
                }
            })
            .unwrap();
            
        }
    });
}
