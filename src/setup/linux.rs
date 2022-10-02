use colored::Colorize;
use std::fs::{copy, create_dir, metadata, remove_file, write};

use crate::success;

pub fn setup_l() {
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
    if metadata("./oxup-linux.tar.gz").is_ok() {
        remove_file("oxup-linux.tar.gz").unwrap();
    }

    success![format!("Created {}/.oxido", std::env::var("HOME").unwrap())]
}
