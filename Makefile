all: build

run:
	trunk serve --address=0.0.0.0

build:
	trunk build --release

cargo-build:
	cargo build
