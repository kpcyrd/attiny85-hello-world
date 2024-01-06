build:
	RUSTC_BOOTSTRAP=1 cargo build --release

.PHONY: build

upload:
	RUSTC_BOOTSTRAP=1 cargo run --release

.PHONY: upload
