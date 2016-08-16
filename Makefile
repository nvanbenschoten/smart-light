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

.PHONY: db-up
db-up:
	cockroach start --background --host=0.0.0.0 --insecure
	cockroach sql -e 'CREATE DATABASE IF NOT EXISTS smart_light'

.PHONY: db-down
db-down:
	cockroach quit