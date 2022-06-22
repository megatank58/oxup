# oxup

Oxup is a tool for managing installations and packages of oxido.

## Installation

You can install oxup by running the commands below or from [github releases](https://github.com/oxidite/oxup/releases), unzip the download and add the binary to your path.

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
tar -xf oxup-linux.tar.gz
mkdir $HOME/.oxido
mv oxup $HOME/.oxido
export PATH="$HOME/.oxido:$PATH"
echo "export PATH="$HOME/.oxido:$PATH"" >> $HOME/.bashrc
```

### Macos

```bash
wget https://github.com/oxidite/oxup/releases/latest/download/oxup-darwin.zip
unzip oxup-darwin.zip
mkdir $HOME/.oxido
mv oxup $HOME/.oxido
export PATH="$HOME/.oxido:$PATH"
echo "export PATH="$HOME/.oxido:$PATH"" >> $HOME/.bashrc
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
