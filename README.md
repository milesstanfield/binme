# Generate release binary
```bash
rm -rf binaries/release.txt && \
  cargo build --release && \
  cp -rf target/release/binme binaries/release.txt
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
