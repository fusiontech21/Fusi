echo "Updating Fusi..."

if [ ! -d ~/Fusi ]; then
    echo "Fusi repo not found, cloning..."
    git clone https://github.com/fusiontech21/Fusi.git ~/Fusi
fi

cd ~/Fusi

git pull

cargo build --release
sudo cp target/release/fusi /usr/local/bin/
sudo chmod +x /usr/local/bin/fusi

echo "© Fusi updated successfully!"