use std::env::consts;

#[derive(PartialEq, Eq, PartialOrd, Ord)]
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
}
