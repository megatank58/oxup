printf "Downloading..."

curl -s https://api.github.com/repos/megatank58/oxido/releases/latest \
| grep "browser_download_url.*tar.gz" \
| cut -d : -f 2,3\
| tr -d \" \
| wget -qi -

printf "\tdone\n"

printf "Unpacking..."

tar -xf oxido*.tar.gz

printf "\tdone\n"

printf "Installing..."

sudo install oxido /usr/bin/

printf "\tdone\n"

printf "Clearing up..."

rm -rf oxido_*

printf "\tdone\n"
