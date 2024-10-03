# Project Template for rp235x-hal
Almost entirely based off of [rp-rs/rp2040-project-template](https://github.com/rp-rs/rp2040-project-template/tree/main)
Currently `picotool` is the default runner so every time you want to flash, you will need to plug in the Pico 2 while holding down BOOTSEL, or some other means of getting there./
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

## Branches:
There are two branches in this repo: `Main` and `picotool-reset`
- `Main`: This is identical functionality to [rp-rs/rp2040-project-template](https://github.com/rp-rs/rp2040-project-template/tree/main). The only difference is that the runner is exclusively set to picotool
- `picotool-reset`: This is based on the [`Multicore FIFO`](https://github.com/rp-rs/rp-hal/blob/main/rp235x-hal-examples/src/bin/multicore_fifo_blink.rs) example. This uses the same idea of using both cores, but instead of a blocking wait in core1, it simply polls core1 to see if it's done. This allows for the Pico 2 to act as a USB device that picotool can recognize, and with a port of [usbd-picotool-reset](https://github.com/Navelwriter/usbd-picotool-reset), am able to reboot the pico into BOOTSEL mode using only the command line. 

## Requirements
- The standard Rust tooling (cargo, rustup) which you can install from https://rustup.rs/
- Toolchain support for the cortex-m0+ processors in the rp2040 (thumbv6m-none-eabi)
- Picotool and SDK for the rp2350. Both are easy to install through install script in `Appendix B` [here](https://datasheets.raspberrypi.com/pico/getting-started-with-pico.pdf)
- flip-link - this allows you to detect stack-overflows on the first core, which is the only supported target for now.
- Raspberry Pi Pico 2

## Installation
```sh
rustup target install thumbv6m-none-eabi
cargo install flip-link
```

## Running
Plug in the Pico 2 in BOOTSEL mode\
For a debug build
```sh
cargo run
```
For a release build
```sh
cargo run --release
```
