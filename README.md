# Generate release binary
```bash
rm -rf bin/binme && \
  cargo build --release && \
  cp -rf target/release/binme binaries/release.txt
```

# Download/Update
```bash
sudo rm -rf /usr/local/bin/binme && \
  curl -L https://github.com/milesstanfield/binme/raw/main/bin/release.txt > binme && \
  sudo chmod +x binme && \
  sudo mv binme /usr/local/bin
```
