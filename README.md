# oxup

Oxup is a tool for managing installations and packages of oxido.

## Installation

You can download oxup from the [releases page](https://github.com/oxidite/oxup/releases) for your operating system, unzip the download and add the binary to your path.

### Windows

```sh
wget https://github.com/oxidite/oxup/releases/latest/download/oxup-windows.zip
unzip oxup-windows.zip
mkdir C:\oxido
setx PATH "C:\oxido;%PATH%" # path to binary
```

### Linux

```bash
wget https://github.com/oxidite/oxup/releases/latest/download/oxup-linux.tar.gz
tar -xf oxup_v1.0.0_x86_64-unknown-linux-musl.tar.gz
mkdir $HOME/.oxido
mv oxup $HOME/.oxido
export PATH=$PATH:$HOME/.oxido
```

### Macos

```bash
wget https://github.com/oxidite/oxup/releases/latest/download/oxup-darwin.zip
unzip oxup-darwin.zip
mkdir $HOME/.oxido
mv oxup $HOME/.oxido
export PATH=$PATH:$HOME/.oxido
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
