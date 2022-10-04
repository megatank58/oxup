use crate::{info, success, os::OS};

use std::fs::{copy, create_dir, metadata, remove_file, write};

pub fn setup(os: OS) {
    match os {
        OS::Windows => {
            if metadata("C:\\bin\\oxido").is_err() {
                create_dir("C:\\bin\\oxido").unwrap();
                info!["Created directory C:\\bin\\oxido"];
            }

            copy("oxup.exe", "C:\\bin\\oxido\\oxup.exe").unwrap();
            remove_file("oxup.exe").unwrap();
            remove_file("oxup-windows.zip").unwrap();
        }
        OS::Mac | OS::Linux => {
            if metadata(format!("{}/.oxido", std::env::var("HOME").unwrap())).is_err() {
                create_dir(format!("{}/.oxido", std::env::var("HOME").unwrap())).unwrap();
                create_dir(format!("{}/.oxido/bin", std::env::var("HOME").unwrap())).unwrap();
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

            copy(
                "oxup",
                format!("{}/.oxido/bin/oxup", std::env::var("HOME").unwrap()),
            )
            .unwrap();
            if metadata("./oxup").is_ok() {
                remove_file("oxup").unwrap();
            }
            if metadata("./oxup-darwin.zip").is_ok() {
                remove_file("oxup-darwin.zip").unwrap();
            }

            success![format!("Created {}/.oxido", std::env::var("HOME").unwrap())]
        }
    }
}
