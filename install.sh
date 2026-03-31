echo "Installing Fusi..."
curl -L https://github.com/fusiontech21/Fusi/releases/latest/download/fusi -o /tmp/fusi
sudo chmod +x /tmp/fusi
sudo cp /tmp/fusi /usr/local/bin/fusi
echo "© Fusi installed successfully!"