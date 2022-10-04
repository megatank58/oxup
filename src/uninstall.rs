use crate::{os::OS, success};

pub fn uninstall(os: OS) {
    match os {
        OS::Linux | OS::Mac => {
            std::fs::remove_dir_all(format!(
                "{}/.oxido/bin/oxido",
                std::env::var("HOME").unwrap()
            ))
            .unwrap();
        }
        OS::Windows => {
            std::fs::remove_file(r"C:\bin\oxido").unwrap();
        }
    }

    success!["Oxido has been uninstalled"];
}
