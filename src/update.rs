use crate::{install::install, Updateable, OS};
use std::{env::var, error::Error, fs::remove_file};

pub async fn update(update: Updateable, os: OS) -> Result<(), Box<dyn Error>> {
    let (bin, oxate) = match update {
        Updateable::Oxido => ("oxido", false),
        Updateable::Oxate => ("oxate", true),
    };

    remove_file(match os {
        OS::Windows => format!(r"C:\bin\{bin}.exe"),
        _ => {
            format!("{}/.oxido/bin/{bin}", var("HOME").unwrap())
        }
    })?;

    install(os, oxate).await?;

    Ok(())
}
