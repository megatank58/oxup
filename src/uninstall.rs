use crate::{os::OS, success};
use std::{
    env::var,
    fs::{remove_dir_all, remove_file},
};

pub fn uninstall(os: OS) {
    match os {
        OS::Linux | OS::Mac => {
            remove_dir_all(format!("{}/.oxido/bin/oxido", var("HOME").unwrap())).unwrap();
        }
        OS::Windows => {
            remove_file(r"C:\bin\oxido").unwrap();
        }
    }

    success!["Oxido has been uninstalled"];
}
