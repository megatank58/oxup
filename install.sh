#!/bin/sh
BLUE='\033[0;34m'
NONE='\033[0m'
PREFIX="${BLUE}=>${NONE} "
echo "${PREFIX}Downloading oxate"

curl --proto '=https' --tlsv1.2 -sSfLO https://github.com/oxidic/oxate/releases/latest/download/oxate
chmod +x oxate
./oxate setup

echo "${PREFIX}Write \`. \"$HOME/.oxido/env\"\` in the shell config to put oxate in path."
