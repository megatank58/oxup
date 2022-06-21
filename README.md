# oxup

Oxup is a tool for managing installations and packages of oxido.

## Installation

You can download oxup from the releases page for your operating system, unzip the download and add the binary to your path.

### Windows

```bash
setx PATH "C:\oxido;%PATH%" # path to binary
```

### Linux

```bash
sudo install oxido /usr/local/bin
```

### Macos

```bash
sudo install oxido /usr/local/bin
```

## Usage

```bash
oxup <command> [OPTIONS]
```

## Commands

- `add` add packages to your project
- `install` install oxido interpreter
- `remove` remove packages from your project
- `update` update the oxido interpreter to the latest version permitted by semver
- `uninstall` uninstall oxido interpreter
- `version` prints the version

Options:
- `-W` force run as windows
- `-L` force run as linux
- `-M` force run as macos
