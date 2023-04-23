BLUE='\033[0;34m'
NONE='\033[0m'
PREFIX="${BLUE}=>${NONE} "
echo -e "${PREFIX}Downloading oxate"

if [[ "$OSTYPE" == "linux-gnu"* ]]; then
    curl -LO https://github.com/oxidic/oxate/releases/latest/download/oxate
    chmod +x oxate
    ./oxate setup
elif [[ "$OSTYPE" == "darwin"* ]]; then
    curl -LO https://github.com/oxidic/oxate/releases/latest/download/oxate-darwin
    chmod +x oxate-darwin
    ./oxate-darwin setup
else
    curl -LO https://github.com/oxidic/oxate/releases/latest/download/oxate.exe
    ./oxate.exe setup
fi

echo -e "${PREFIX}Add the following lines to your shell configuration to add oxate to PATH: \`. \"$HOME/.oxido/env\"\`"

