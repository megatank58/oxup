use crate::{info, os::OS, success};

use std::{
    env::var,
    fs::{copy, create_dir_all, metadata, remove_file, set_permissions, write, Permissions},
    os::unix::prelude::PermissionsExt,
};

pub fn setup(os: OS) -> Result<(), Box<dyn std::error::Error>> {
    match os {
        OS::Windows => {
            if metadata("C:\\bin\\oxido").is_err() {
                create_dir_all("C:\\bin\\oxido")?;
                info!("Created directory C:\\bin\\oxido");
            }

            copy("oxate.exe", "C:\\bin\\oxido\\oxate.exe")?;
            remove_file("oxate.exe")?;
        }
        OS::Mac | OS::Linux => {
            let home = var("HOME")?;
            if metadata(format!("{home}/.oxido")).is_err() {
                create_dir_all(format!("{home}/.oxido/bin"))?;
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
            ?;

            if metadata("./oxate").is_ok() {
                set_permissions("./oxate", Permissions::from_mode(0o770))?;
                copy("oxate", format!("{home}/.oxido/bin/oxate"))?;
                remove_file("oxate")?;
            }

            success!(format!("Created {home}/.oxido"));
        }
    };

    Ok(())
}
