use crate::{info, os::OS, success};

use std::fs::{copy, create_dir_all, metadata, remove_file, write};

pub fn setup(os: OS) {
    match os {
        OS::Windows => {
            if metadata("C:\\bin\\oxido").is_err() {
                create_dir_all("C:\\bin\\oxido").unwrap();
                info!["Created directory C:\\bin\\oxido"];
            }

            copy("oxup.exe", "C:\\bin\\oxido\\oxup.exe").unwrap();
            remove_file("oxup.exe").unwrap();
        }
        OS::Mac | OS::Linux => {
            if metadata(format!("{}/.oxido", std::env::var("HOME").unwrap())).is_err() {
                create_dir_all(format!("{}/.oxido/bin", std::env::var("HOME").unwrap())).unwrap();
            }

            write(
                format!("{}/.oxido/env", std::env::var("HOME").unwrap()),
                "
        #!/bin/sh
        # oxup shell setup
        # affix colons on either side of $PATH to simplify matching
        case \":${PATH}:\" in
            *:\"$HOME/.oxido/bin\":*)
                ;;
            *)
                # Prepending path in case a system-installed oxido needs to be overridden
                export PATH=\"$HOME/.oxido/bin:$PATH\"
                ;;
        esac
        ",
            )
            .unwrap();

            if metadata("./oxup").is_ok() {
                copy(
                    "oxup",
                    format!("{}/.oxido/bin/oxup", std::env::var("HOME").unwrap()),
                )
                .unwrap();
                remove_file("oxup").unwrap();
            }

            success![format!("Created {}/.oxido", std::env::var("HOME").unwrap())];
        }
    }
}
