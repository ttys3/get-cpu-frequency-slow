build:
	cargo build --release

static:
	cargo build --release --target x86_64-unknown-linux-musl

run:
	./target/release/get-cpu-frequency-slow 1
	./target/release/get-cpu-frequency-slow
