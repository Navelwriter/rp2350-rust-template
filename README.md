# Project Template for rp235x-hal
The main branch is best used with cortex-m debugger using the picoprobe debugger as the runner. This allows for automatic flashing and line-by-line debugging in vscode.\
Also allows for defmt to be used for logging.

`picotool` is also available as a runner so every time you want to flash, you will need to plug in the Pico 2 while holding down BOOTSEL, or some other means of getting there.

This means that once it's in BOOTSEL mode, it's as easy as
```sh 
cargo run
```
for debugging
or 
```sh
cargo run --release
```
for release builds

## Cargo Generate (Recommended)
To use this template, you can (should) use cargo generate.\
This also renames all instances of `rp2040-project-template` to the name you provide, including the vscode workspace files. This makes it effortless to start a new project with this template.\
To install cargo generate, run
```sh
cargo install cargo-generate
```
Then to generate a new project, run
```sh
cargo generate --git https://github.com/Navelwriter/rp2350-rust-template.git
```
if you want to use the `picotool` runner, run
```sh
cargo generate --git https://github.com/Navelwriter/rp2350-rust-template.git --branch picotool-reset
```

Then it's as easy as entering the name of your project and you're ready to go!

If you have the picoprobe, or "debugprobe" as it's called now, connected properly to the Pico 2, you can start debugging right away in vscode.

## Features
- `defmt` logging
To use defmt, you need to run the following command in the terminal **during a debug session**
```sh
nc localhost 8765 | defmt-print -e target/thumbv8m*/debug/yuh
```
This will allow you to see the logs in real-time. 

Remember that defmt is only available in debug builds so you will need to have an active debug session, it defaults to halt on the main() entry so you can just enter the command right after pressing the debug button in vscode.

There is a log macro that you can use to log messages easily.
```rust
log!(info, "Hello, world!");
log!(warn, "This is a warning");
log!(error, "This is an error");
```
If you want to log a value, you can just use it the normal way in rust
```rust
let x = 5;
log!(info, "The value of x is {}", x);
```
If you run
```sh
cargo build --release
```
You will see that the log messages are not present in the binary. This is because defmt is only available in debug builds.

- `flip-link`
This is a tool that allows you to detect stack overflows in your code. It's a great tool to have in your toolbelt. This just "works" after you you install it through cargo and is compiled. It will automatically detect stack overflows and halt the program. You can then use the backtrace to see where the overflow happened.

## Branches:
There are two branches in this repo: `Main` and `picotool-reset`
- `Main`: This is identical functionality to [rp-rs/rp2040-project-template](https://github.com/rp-rs/rp2040-project-template/tree/main). The only difference is that the runner is using the raspberry-pi debug probe using vscode to parse through code. 
- `picotool-reset`: This is based on the [`Multicore FIFO`](https://github.com/rp-rs/rp-hal/blob/main/rp235x-hal-examples/src/bin/multicore_fifo_blink.rs) example. This uses both cores, but instead of a blocking wait in core1, it simply polls core1 to see if it's done. This allows for the Pico 2 to act as a USB device that picotool can recognize, and with a port of [usbd-picotool-reset](https://github.com/Navelwriter/usbd-picotool-reset), am able to reboot the pico into BOOTSEL mode using only the command line. 

## Requirements
- The standard Rust tooling (cargo, rustup) which you can install from https://rustup.rs/
- cargo-generate
- Toolchain support for the cortex-m33 processors in the rp2350 (thumbv8m.main-none-eabihf)
- Debug tool for the rp2350 (picoprobe) [here](https://www.raspberrypi.com/documentation/microcontrollers/debug-probe.html)
- Picotool(optional) and SDK for the rp2350. Both are easy to install through install script in `Appendix B` [here](https://datasheets.raspberrypi.com/pico/getting-started-with-pico.pdf)
- flip-link - this allows you to detect stack-overflows on the first core, which is the only supported target for now.
- Raspberry Pi Pico 2
- openocd, install with `sudo apt-get openocd`
- VSCode with Cortex-Debugger extension

## Installation
```sh
rustup target install thumbv8m.main-none-eabihf
cargo install flip-link
cargo install cargo-generate
```

```sh
cargo generate --git https://github.com/Navelwriter/rp2350-rust-template.git
```

## Running
Plug in the Pico 2 in BOOTSEL mode if you're using picotool.\

For line-by-line debugging in vscode, you can just press the debug button in vscode.\
To build a debug build

```sh
cargo run
```
For a release build
```sh
cargo run --release
```
