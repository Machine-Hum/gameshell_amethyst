# Getting Started

%s/1.0.3/1.0.2/g
https://stackoverflow.com/questions/60576277/rust-amethyst-error-e0433-failed-to-resolve-could-not-find-rt-in-qu

Straight compile. To straight compile I've spun up an Amazon AWS ec2 ARM instance. 
```
sudo apt install pkg-config libasound2-dev cmake libfreetype6-dev libfontconfig1-dev xclip libxcb-render-util0-dev libxcb-shape0-dev libxcb-xfixes0-dev
```

## Cross Comilation
I have not been able to get this to work yet. This is an issue outlined
[here](https://github.com/Machine-Hum/gameshell_amethyst/issues/1).

```
sudo apt-get install -qq gcc-arm-linux-gnueabihf
rustup target add armv7-unknown-linux-gnueabihf
mkdir -p ~/.cargo
cp resources/config ~/.cargo/
cargo build --target=armv7-unknown-linux-gnueabihf
```
