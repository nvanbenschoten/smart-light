.PHONY: build
build:
	@cargo build

.PHONY: build-pi
build-pi: raspberrypi-tools
	@rustup target add arm-unknown-linux-gnueabihf
	@cargo build --target=arm-unknown-linux-gnueabihf

.PHONY: clean
clean:
	@cargo clean

raspberrypi-tools:
	@git clone https://github.com/raspberrypi/tools.git raspberrypi-tools

.PHONY: db-start
db-start:
	cockroach start --background
	cockroach sql -e 'CREATE DATABASE IF NOT EXISTS smart_light'

.PHONY: db-stop
db-stop:
	cockroach quit