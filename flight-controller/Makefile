
.PHONY: debug build release flash

build:
	cargo build

debug:
	cargo run

release:
	cargo build --release

format:
	cargo fmt

openocd:
	openocd -f interface/stlink-v2-1.cfg -f target/stm32f3x.cfg

flash:
	openocd -f interface/stlink-v2-1.cfg -f target/stm32f3x.cfg -f ./scripts/flash.cfg