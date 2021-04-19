DEFAULT=build

.PHONY: debug build release flash

debug:
	cargo run

build:
	cargo build

release:
	cargo build --release

flash:
	echo 'not implemented'