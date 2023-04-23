use crate::{info, os::OS, success};

use std::{
    env::var,
    fs::{copy, create_dir_all, metadata, remove_file, set_permissions, write, Permissions},
    os::unix::prelude::PermissionsExt,
};

pub fn setup(os: OS) {
    match os {
        OS::Windows => {
            if metadata("C:\\bin\\oxido").is_err() {
                create_dir_all("C:\\bin\\oxido").unwrap();
                info!["Created directory C:\\bin\\oxido"];
            }

            copy("oxate.exe", "C:\\bin\\oxido\\oxate.exe").unwrap();
            remove_file("oxate.exe").unwrap();
        }
        OS::Mac | OS::Linux => {
            let home = var("HOME").unwrap();
            if metadata(format!("{home}/.oxido")).is_err() {
                create_dir_all(format!("{home}/.oxido/bin")).unwrap();
            }

            write(
                format!("{home}/.oxido/env"),
                "
        #!/bin/sh
        # oxate shell setup
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

            if metadata("./oxate").is_ok() {
                set_permissions("./oxate", Permissions::from_mode(0o770)).unwrap();
                copy("oxate", format!("{home}/.oxido/bin/oxate")).unwrap();
                remove_file("oxate").unwrap();
            }

            success![format!("Created {home}/.oxido")];
        }
    }
}
