use crate::{install::install, Updateable, OS};

pub async fn update(update: Updateable, os: OS) -> Result<(), Box<dyn std::error::Error>> {
    let (bin, oxup) = match update {
        Updateable::Oxido => ("oxido", false),
        Updateable::Oxup => ("oxup", true),
    };

    std::fs::remove_file(match os {
        OS::Windows => format!(r"C:\bin\{bin}.exe"),
        _ => {
            format!("{}/.oxido/bin/{bin}", std::env::var("HOME").unwrap())
        }
    })?;

    install(os, oxup).await?;

    Ok(())
}
