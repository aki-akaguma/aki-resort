
all: list

MAKEFILE_LIST = Makefile
# Self-documenting Makefile targets script from Stack Overflow
# Targets with comments on the same line will be listed.
list:
	@LC_ALL=C $(MAKE) -pRrq -f $(firstword $(MAKEFILE_LIST)) : 2>/dev/null | awk -v RS= -F: '/(^|\n)# Files(\n|$$)/,/(^|\n)# Finished Make data base/ {if ($$1 !~ "^[#.]") {print $$1}}' | sort | grep -E -v -e '^[^[:alnum:]]' -e '^$@$$'

.PHONY: list

readme: README.md

README.md: README.tpl src/lib.rs
	cargo readme > $@

fetch:
	cargo fetch

test:
	cargo test --offline

test-no-default-features:
	cargo test --offline --no-default-features

miri:
	MIRIFLAGS=-Zmiri-disable-isolation cargo +nightly miri test --offline

clean:
	@cargo clean
	@rm -f z.* *.log *.tmp *.profraw

clippy:
	cargo clippy --offline --tests --workspace -- -W clippy::uninlined_format_args

fmt:
	cargo fmt

doc:
	cargo doc

tarpaulin:
	#cargo tarpaulin --offline --engine llvm --out html --output-dir ./target
	cargo tarpaulin --offline --engine llvm --out lcov --output-dir ./target
	#cargo tarpaulin --offline --engine ptrace --out lcov --output-dir ./target
	genhtml -o target/lcov --demangle-cpp target/lcov.info

gen-src-cmd:
	cargo xtask gen-src-cmd gen-src-cmd


test-x86_64:
	cargo test --offline --release --target=x86_64-unknown-linux-musl

test-i686:
	cargo test --offline --release --target=i686-unknown-linux-musl

test-aarch64:
	cargo test --offline --release --target=aarch64-unknown-linux-musl

test-armv7:
	cargo test --offline --release --target=armv7-unknown-linux-musleabihf

test-mips64el:
	cargo test --offline --release --target=mips64el-unknown-linux-muslabi64

test-mipsel:
	cargo test --offline --release --target=mipsel-unknown-linux-musl
