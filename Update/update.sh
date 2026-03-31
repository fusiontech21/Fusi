cd ~/Fusi
git pull
cargo build --release
sudo cp target/release/fusi /usr/local/bin/
echo "© Fusi updated successfully!"