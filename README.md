# Generate release binary
```bash
./bin/build
```

# Download/Update
```bash
sudo rm -rf /usr/local/bin/binme &&
  rm -rf binme-release.txt &&
  curl -H "Authorization: token $GITHUB_TOKEN" \
    -o binme-release.txt \
    https://raw.githubusercontent.com/milesstanfield/binme/main/binaries/release.txt &&
  sudo chmod +x binme-release.txt &&
  sudo mv binme-release.txt /usr/local/bin/binme
```
