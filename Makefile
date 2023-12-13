build:
	RUSTC_BOOTSTRAP=1 cargo build --release
	avr-objcopy --output-target=ihex target/avr-attiny85/release/attiny85-hello-world.elf target/avr-attiny85/release/attiny85-hello-world.hex

.PHONY: build
