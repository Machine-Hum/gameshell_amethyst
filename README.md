# Getting Started

```
sudo apt-get install -qq gcc-arm-linux-gnueabihf
rustup target add armv7-unknown-linux-gnueabihf
mkdir -p ~/.cargo
cp resources/config ~/.cargo/
cargo build --target=armv7-unknown-linux-gnueabihf
```
