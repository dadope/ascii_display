echo "removing script call from .bashrc"

sed -e "s/ascii_display//g" -i ~/.bashrc
rm -rf ~/.local/bin/ascii_display
rm -rf ~/.asciiDisplay/

echo "Removed script, have a good day!"