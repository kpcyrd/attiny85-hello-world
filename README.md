# attiny85-hello-world

To setup your build environment, make sure the following packages are installed (names may vary across Linux distributions):

```sh
pacman -S avr-gcc avr-libc elf2nucleus git
```

If you already have [rustup](https://rustup.rs/) setup, also run the following:

```sh
rustup component add rust-src
```

If you want to avoid rustup you can also instead use:

```sh
pacman -S rust rust-src
```

You can then download, build and flash over usb with [micronucleus](https://github.com/micronucleus/micronucleus) and [elf2nucleus](https://github.com/kpcyrd/elf2nucleus):

```sh
git clone https://github.com/kpcyrd/attiny85-hello-world
cd attiny85-hello-world
RUSTC_BOOTSTRAP=1 cargo run --release
```

`RUSTC_BOOTSTRAP=1` is necessary because this target has not been fully stabilized yet and is still subject to change, you may need to make changes to stay compatible with future Rust compilers.

## Building firmware into elf file

```sh
RUSTC_BOOTSTRAP=1 cargo build --release
```

the built firmware is located at **target/avr-attiny85/release/attiny85-hello-world.elf**. You can then flash it using [elf2nucleus](https://github.com/kpcyrd/elf2nucleus):

```sh
elf2nucleus target/avr-attiny85/release/attiny85-hello-world.elf
```

To extract the raw binary (for example for flashing with micronucleus directly), use:

```sh
elf2nucleus target/avr-attiny85/release/attiny85-hello-world.elf ./firmware.bin
```

## Building firmware into ihex file

```sh
RUSTC_BOOTSTRAP=1 cargo build --release
avr-objcopy --output-target=ihex target/avr-attiny85/release/attiny85-hello-world.elf target/avr-attiny85/release/attiny85-hello-world.hex
```

the built firmware is located at **target/avr-attiny85/release/attiny85-hello-world.hex**. You can then flash it using [micronucleus](https://github.com/micronucleus/micronucleus):

```sh
micronucleus --timeout 60 --run --no-ansi target/avr-attiny85/release/attiny85-hello-world.hex
```
