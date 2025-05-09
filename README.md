# Simple Logging Interface Program (SLIP)
This is an incredibly simple and lightweight program to receive logs from two projects of mine: [Void Stranger Endless Void](https://github.com/skirlez/void-stranger-endless-void), and [Nubby's Forgery](https://github.com/Skirlez/nubbys-forgery).

All it does is listen for UDP packets coming from localhost to a port, and spits out whatever it is given. With rainbow colors!

![image](https://github.com/user-attachments/assets/f987b062-1312-405c-8cbd-adb774942f89)

## Usage
`./slip` (uses port 1235)

`./slip [port]`

## Building
`cargo build [--release]`

For Nix users, you have `shell.nix`, from which you can do

`cargo build [--release] [--target x86_64-pc-windows-gnu]`

## License
`shell.nix` is taken (and modified) from https://github.com/jraygauthier/jrg-rust-cross-experiment/tree/master/simple-static-rustup-target-windows, so it is licensed under the Apache 2.0 license.

The rest is licensed under the AGPLv3 license.

## Contributing
Please contribute

## Is it any good?
Yes
