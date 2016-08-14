.PHONY: build
build:
	@cargo build

.PHONY: build-pi
build-pi: raspberrypi-tools
	@rustup target add arm-unknown-linux-gnueabihf
	@cargo build --target=arm-unknown-linux-gnueabihf

raspberrypi-tools:
	@git clone https://github.com/raspberrypi/tools.git raspberrypi-tools