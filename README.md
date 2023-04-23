# Oxate

Oxate is a tool for managing installations and packages of oxido.

## Installation

```bash
curl --proto '=https' --tlsv1.2 -sLSf https://megatank58.tech/oxate_install.sh | sh
```

Then you can restart your shell and install oxido.

## Usage

```bash
oxate <command> [OPTIONS]
```

| Command     | Description                                                            |
| ---         | ---                                                                    |
| `add`       | add packages to your project                                           |
| `install`   | install oxido interpreter                                              |
| `setup`     | setup oxido directories                                                |
| `remove`    | remove packages from your project                                      |
| `update`    | update the oxido interpreter to the latest version permitted by semver |
| `uninstall` | uninstall oxido interpreter                                            |
| `version`   | prints the version                                                     |

| Option | Description          |
| ---    | ---                  |
| `-W`   | force run as windows |
| `-L`   | force run as linux   |
| `-M`   | force run as macos   |
| `-n`   | Do not check for updates |
| `-h`   | Print help information |
| `-V`   | Print version information |
  
