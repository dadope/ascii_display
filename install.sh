echo "Building project..."

cargo build --release

echo "Finished building, installing locally..."
echo "ascii_display" >> ~/.bashrc
cp target/release/ascii_display ~/.local/bin/

echo "The program should now be installed!"