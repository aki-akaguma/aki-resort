
all: readme

readme: README.md

README.md: README.tpl src/lib.rs
	cargo readme > $@

test:
	cargo test

clean:
	cargo clean

clippy:
	cargo clippy --tests

test-x86_64:
	cargo test --release --target=x86_64-unknown-linux-musl

test-i686:
	cargo test --release --target=i686-unknown-linux-musl

test-aarch64:
	cargo test --release --target=aarch64-unknown-linux-musl

test-armv7:
	cargo test --release --target=armv7-unknown-linux-musleabihf

test-mips64el:
	cargo test --release --target=mips64el-unknown-linux-muslabi64

test-mipsel:
	cargo test --release --target=mipsel-unknown-linux-musl
