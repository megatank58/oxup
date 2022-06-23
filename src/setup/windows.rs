use std::fs::{create_dir, metadata, remove_file, copy};

pub fn setup_w() {
    if !metadata("C:\\bin\\oxido").is_ok() {
        create_dir("C:\\bin\\oxido").unwrap();
        println!("created directory C:\\bin\\oxido");
    }

    copy(
        "oxup.exe",
        "C:\\bin\\oxido\\oxup.exe",
    )
    .unwrap();
    remove_file("oxup.exe").unwrap();
    remove_file("oxup-windows.zip").unwrap();
}
