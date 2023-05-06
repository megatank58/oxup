#!/bin/sh
BLUE='\033[0;34m'
NONE='\033[0m'
PREFIX="${BLUE}=>${NONE} "
echo -e "${PREFIX}Downloading oxate"

if [[ "$OSTYPE" == "linux-gnu"* ]]; then
    curl --proto '=https' --tlsv1.2 -sSfLO https://github.com/oxidic/oxate/releases/latest/download/oxate
    chmod +x oxate
    ./oxate setup
elif [[ "$OSTYPE" == "darwin"* ]]; then
    curl --proto '=https' --tlsv1.2 -sSfLO https://github.com/oxidic/oxate/releases/latest/download/oxate-darwin
    chmod +x oxate-darwin
    ./oxate-darwin setup
else
    curl --proto '=https' --tlsv1.2 -sSfLO https://github.com/oxidic/oxate/releases/latest/download/oxate.exe
    ./oxate.exe setup
fi

#!/bin/sh
BLUE='\033[0;34m'
NONE='\033[0m'
PREFIX="${BLUE}=>${NONE} "
echo -e "${PREFIX}Downloading oxate"

if [[ "$OSTYPE" == "linux-gnu"* ]]; then
    curl --proto '=https' --tlsv1.2 -sSfLO https://github.com/oxidic/oxate/releases/latest/download/oxate
    chmod +x oxate
    ./oxate setup
elif [[ "$OSTYPE" == "darwin"* ]]; then
    curl --proto '=https' --tlsv1.2 -sSfLO https://github.com/oxidic/oxate/releases/latest/download/oxate-darwin
    chmod +x oxate-darwin
    ./oxate-darwin setup
else
    curl --proto '=https' --tlsv1.2 -sSfLO https://github.com/oxidic/oxate/releases/latest/download/oxate.exe
    ./oxate.exe setup
fi

read -p "${PREFIX}Add binary to path (y/n)" confirm && if [[ ! "$confirm" =~ ^[Yy]+([eE][sS])?$ ]]; then 
    if [[ $SHELL == *"bash" ]]; then
        echo '. "$HOME/.oxido/env"' >> $HOME/.bashrc
    elif [[ $SHELL == *"zsh" ]]; then
        echo '. "$HOME/.oxido/env"' >> $HOME/.zshrc
    else
        echo -e "${PREFIX}Write \`. \"$HOME/.oxido/env\"\` in the shell config to put oxate in path."
    fi
fi
