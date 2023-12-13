# Generate release binary
```bash
rm -rf binaries/release.txt && \
  cargo build --release && \
  cp -rf target/release/binme binaries/release.txt
```

# Download/Update
```bash
sudo rm -rf /usr/local/bin/binme && \
  rm -rf release.txt && \
  curl -H "Authorization: token $GITHUB_TOKEN" \
    -o release.txt \
    https://raw.githubusercontent.com/milesstanfield/binme/main/binaries/release.txt && \
  sudo chmod +x release.txt && \
  sudo mv release.txt /usr/local/bin/binme
```
