![linux](https://github.com/KanarekLife/rust_figlet/workflows/linux/badge.svg)
![windows](https://github.com/KanarekLife/rust_figlet/workflows/windows/badge.svg)
![macos](https://github.com/KanarekLife/rust_figlet/workflows/macos/badge.svg)

# rust_figlet
Small program written in rust to work as figlet replacement for all available major platforms.

## How to install it?
To download binary head into 'Releases' tab, select your os, unzip it and just run executable. It is
recommended to place this binary in folder that is in your PATH variable. Then just run it via './rust_figlet'
 or './rust_figlet.exe'.
 
 ## How to compile it?
 1. `git clone https://github.com/KanarekLife/rust_figlet`
 2. Make sure you have downloaded rustlang (stable chain) from https://rust-lang.org
 3. (ONLY ON LINUX) Download libssl-dev (or equivalent) via your package manager
 4. Head into downloaded folder and run `cargo build --release` and cargo wil download all dependencies
 and compile `rust_figlet` into `./target/release` folder.
 
 ## Can I test it via browser?
 Yea sure! Head into http://rustfiglet-demo.kanareklife.repl.run/ to run it into your browser and test it!
 
 #### Created by KanarekLife @ 2020 (https://github.com/KanarekLife)
