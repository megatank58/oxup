use std::env::{consts, var};

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum OS {
    Mac,
    Linux,
    Windows,
}

impl OS {
    pub fn from() -> OS {
        match consts::OS {
            "linux" => OS::Linux,
            "mac" => OS::Mac,
            _ => OS::Windows,
        }
    }

    pub fn path(&self) -> String {
        match self {
            OS::Windows => String::from(r"C:\bin\"),
            _ => var("HOME").unwrap() + "/.oxido/bin/",
        }
    }
}
