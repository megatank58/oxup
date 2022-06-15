# oxup

Installer for the Oxido programming language

## Installation

You can use `curl` or `wget` to download the script and run it, you may have to manually remove the oxido binary if its in the working directory with `rm -rf oxido`.

```bash
curl https://raw.githubusercontent.com/megatank58/oxup/main/install.sh | sh
```

## Uninstallation

Oxido ships as a single binary, which is placed at `/usr/bin/oxido`, removing this will also remove the compiler.

```bash
sudo rm -rf /usr/bin/oxido
```