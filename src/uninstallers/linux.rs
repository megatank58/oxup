pub fn uninstall_l() {
    std::fs::remove_file(format!("{}/.oxido/oxido", std::env::var("HOME").unwrap())).unwrap();
}
