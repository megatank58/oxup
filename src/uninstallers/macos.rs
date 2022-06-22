pub fn uninstall_m() {
    std::fs::remove_file(format!("{}/.oxido/oxido", std::env::var("HOME").unwrap())).unwrap();
}
