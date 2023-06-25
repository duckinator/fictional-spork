# GBA experiment

This is an experiment with programming for the GBA.

## Building

### Prerequisites

You will need the following installed in order to build and run this project:

* [`rustup`](https://rustup.rs/)
* `arm-none-eabi-binutils` for assembling and linking
    * Windows: [GNU Arm Embedded Toolchain](https://developer.arm.com/tools-and-software/open-source-software/developer-tools/gnu-toolchain/gnu-rm/downloads).
        Make sure you select "Add path to environment variable" during the install
    * Debian and derivatives (e.g. Ubuntu, raspberry pi OS, linux mint): `sudo apt install binutils-arm-none-eabi`
    * Arch linux and derivatives: `sudo pacman -S arm-none-eabi-binutils`
    * FreeBSD: `sudo pkg install arm-none-eabi-binutils`

The [mgba](https://mgba.io) emulator, with `mgba-qt` in your `PATH`.

For testing on real hardware, you also need `agb-gbafix` (`cargo install agb-gbafix`).

### Running in an emulator

Once you have the prerequisites installed, you should be able to build using
the usual `cargo build` or `cargo build --release`.


The resulting file will be in `target/thumbv4t-none-eabi/debug/gba_demo` or `target/thumbv4t-none-eabi/release/gba_demo` depending on
whether you did a release or debug build.

If you have `mgba-qt` in your path, you can run it in the mgba emulator
using `cargo run` or `cargo run --release`.

## Shipping a .gba file for real hardware

To run it on real hardware, you will need to convert the built file into a
file suitable for running on a real Game Boy Advance.

First build the binary in release mode using the instructions above, then do the following:

```sh
agb-gbafix target/thumbv4t-none-eabi/release/gba_demo -o gba_demo.gba
```
