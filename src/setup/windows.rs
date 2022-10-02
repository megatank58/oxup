use crate::info;
use colored::Colorize;
use std::fs::{copy, create_dir, metadata, remove_file};

pub fn setup_w() {
    if metadata("C:\\bin\\oxido").is_err() {
        create_dir("C:\\bin\\oxido").unwrap();
        info!["Created directory C:\\bin\\oxido"];
    }

    copy("oxup.exe", "C:\\bin\\oxido\\oxup.exe").unwrap();
    remove_file("oxup.exe").unwrap();
    remove_file("oxup-windows.zip").unwrap();
}
