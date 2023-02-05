# Changelog: aki-resort

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased] *
### Changed
* update depends: flood-tide(0.2.9), flood-tide-gen(0.1.20)
* update depends: memx-cdy(0.1.11), runnel(0.3.16)
* update depends: exec-taget(0.2.8), indoc(2.0.0), rust-version-info-file(0.1.8)

### Fixed
* license files
* clippy: `redundant_static_lifetimes`, `needless_borrow`, `bool_assert_comparison`
* rust-version: "1.56.0" to "1.58.0"


## [0.1.24] (2023-01-17)
### Fixed
* bug: can not parse hours: '': invalid digit found in string

## [0.1.23] (2023-01-11)
### Added
* badges into `README.tpl`
* rust-version = "1.56.0" into Cargo.toml

### Changed
* reformat `CHANGELOG.md`
* update depends: anyhow(1.0.68)
* update depends: flood-tide(0.2.8), flood-tide-gen(0.1.19)
* update depends: memx-cdy(0.1.10), runnel(0.3.15)
* update depends: regex(1.7.1)
* update depends: rayon(1.6.1), semver(1.0.16)

### Fixed
* clippy: you are deriving `PartialEq` and can implement `Eq`
* clippy: uninlined_format_args

## [0.1.22] (2022-06-18)
### Changed
* changes to edition 2021
* update depends: flood-tide(0.2.5)
* update depends: memx(0.1.21), memx-cdy(0.1.8), runnel(0.3.11)
* update depends: exec-target(v0.2.6), flood-tide-gen(0.1.16)
* update depends: rust-version-info-file(v0.1.6)
* update depends: semver(1.0.10)
* update depends: crossbeam-channel(0.5.5)

## [0.1.21] (2022-05-22)
### Changed
* update depends: runnel(0.3.10), memx(0.1.20)
* update depends: anyhow(1.0.57), libc(0.2.126), regex(1.5.6), rayon(1.5.3)
* update depends: exec-target(v0.2.5), rust-version-info-file(v0.1.5)

## [0.1.20] (2021-12-18)
### Added
* command option: `--according-to` time.

### Changed
* update depends: anyhow(1.0.51), libc(0.2.112)

## [0.1.19] (2021-11-15)
### Added
* more documents

### Changed
* minimum support rustc 1.47.0 (18bf6b4f0 2020-10-07)
* update depends: flood-tide(0.2.4), memx(0.1.18), memx-cdy(0.1.7), runnel(0.3.9)
* update depends: anyhow(1.0.45), libc(0.2.107)
* update depends: exec-target(v0.2.4), flood-tide-gen(0.1.15), rust-version-info-file(v0.1.3)

## [0.1.18] (2021-09-11)
### Added
* depends: indoc(1.0.3)

### Changed
* pass cargo clippy
* update depends: anyhow(1.0.43), flood-tide-gen(0.1.14), flood-tide(0.2.3), memx-cdy(0.1.6), runnel(0.3.8)
* rewite TARGET_EXE_PATH with `env!(concat!("CARGO_BIN_EXE_", env!("CARGO_PKG_NAME")))`
* update depends: exec-target(0.2.3)

## [0.1.17] (2021-06-24)
### Added
* `memx_cdy::memx_init(); // fast mem operation.`

### Changed
* rewite TARGET_EXE_PATH with `env!("CARGO_BIN_EXE_aki-resort")`

### Fixed
* bug: `#[cfg(feature = "debian_build")]`

## [0.1.16] (2021-06-06)
### Changed
* update depends: semver(1.0.3)

## [0.1.15] (2021-06-03)
### Added
* support `features = \["debian_build"\]`

### Changed
* update depends: flood-tide(0.2.2)
* update depends: regex(1.5.4)

### Fixed
* bug: command option: -X rust-version-info

## [0.1.14] (2021-05-03)
### Added
* support 32bit cpus: i686, armv7, mipsel

### Changed
* update depends: regex(1.5.3)

### Fixed
* bug: this arithmetic operation will overflow: 1024 * 1024 * 1024 * 1024 on i686

## [0.1.13] (2021-04-23)
### Fixed
* bug: build.rs

## [0.1.12] (2021-04-23)
### Added
* command option: `-X`

### Changed
* update depends: flood-tide-gen(0.1.12), flood-tide(0.2.1)
* update depends: bug fix: regex(1.4.6)

## [0.1.11] (2021-04-19)
### Changed
* update depends: flood-tide-gen(0.1.10)

## [0.1.10] (2021-04-07)
### Changed
* update depends: flood-tide(0.2)
* update depends: anyhow(1.0.40), flood-tide-gen(0.1.8), runnnel(0.3.6)

## [0.1.9] (2021-04-01)
### Added
* command option: `--head` and `--tail`

### Changed
* update depend: anyhow(1.0.40)

### Fixed
* bug: should not coloring at the empty match.

## [0.1.8] (2021-03-22)
### Added
* command option: `--color <when>`
* some contents to `--help`

### Changed
* update depend: regex v1.4.5: fixes stack overflows

## [0.1.7] (2021-03-14)
### Changed
* update crate: regex: fix memory leak

## [0.1.6] (2021-03-08)
### Changed
* update crate: runnel
* update crate: rustc_version ("0.3")

## [0.1.5] (2021-03-08)
### Changed
* update crate: runnel

## [0.1.4] (2021-03-07)
### Changed
* use rayon::slice::ParallelSliceMut, for parallel sort
* rename file: xtask/src/cmd.txt to xtask/src/aki-resort-cmd.txt

## [0.1.3] (2021-03-06)
### Fixed
* bug: too large memory

## [0.1.2] (2021-03-05)
### Added
* implement option: `-u, --unique`
* implement option: `--according-to version`
* implement option: `--according-to month`
* add many doc

### Changed
* rename directory `sort_key` to `sort`

### Removed
* remove option: `-k, --key <keydef>`
* remove option: `--field-separator <sep>`

## [0.1.1] (2021-03-03)
### Added
* examples to command help

### Changed
* change option `-e, --regex` to `-e, --exp`

## [0.1.0] (2021-03-01)
* first commit

[Unreleased]: https://github.com/aki-akaguma/aki-resort/compare/v0.1.24..HEAD
[0.1.24]: https://github.com/aki-akaguma/aki-resort/compare/v0.1.23..v0.1.24
[0.1.23]: https://github.com/aki-akaguma/aki-resort/compare/v0.1.22..v0.1.23
[0.1.22]: https://github.com/aki-akaguma/aki-resort/compare/v0.1.21..v0.1.22
[0.1.21]: https://github.com/aki-akaguma/aki-resort/compare/v0.1.20..v0.1.21
[0.1.20]: https://github.com/aki-akaguma/aki-resort/compare/v0.1.19..v0.1.20
[0.1.19]: https://github.com/aki-akaguma/aki-resort/compare/v0.1.18..v0.1.19
[0.1.18]: https://github.com/aki-akaguma/aki-resort/compare/v0.1.17..v0.1.18
[0.1.17]: https://github.com/aki-akaguma/aki-resort/compare/v0.1.16..v0.1.17
[0.1.16]: https://github.com/aki-akaguma/aki-resort/compare/v0.1.15..v0.1.16
[0.1.15]: https://github.com/aki-akaguma/aki-resort/compare/v0.1.14..v0.1.15
[0.1.14]: https://github.com/aki-akaguma/aki-resort/compare/v0.1.13..v0.1.14
[0.1.13]: https://github.com/aki-akaguma/aki-resort/compare/v0.1.12..v0.1.13
[0.1.12]: https://github.com/aki-akaguma/aki-resort/compare/v0.1.11..v0.1.12
[0.1.11]: https://github.com/aki-akaguma/aki-resort/compare/v0.1.10..v0.1.11
[0.1.10]: https://github.com/aki-akaguma/aki-resort/compare/v0.1.9..v0.1.10
[0.1.9]: https://github.com/aki-akaguma/aki-resort/compare/v0.1.8..v0.1.9
[0.1.8]: https://github.com/aki-akaguma/aki-resort/compare/v0.1.7..v0.1.8
[0.1.7]: https://github.com/aki-akaguma/aki-resort/compare/v0.1.6..v0.1.7
[0.1.6]: https://github.com/aki-akaguma/aki-resort/compare/v0.1.5..v0.1.6
[0.1.5]: https://github.com/aki-akaguma/aki-resort/compare/v0.1.4..v0.1.5
[0.1.4]: https://github.com/aki-akaguma/aki-resort/compare/v0.1.3..v0.1.4
[0.1.3]: https://github.com/aki-akaguma/aki-resort/compare/v0.1.2..v0.1.3
[0.1.2]: https://github.com/aki-akaguma/aki-resort/compare/v0.1.1..v0.1.2
[0.1.1]: https://github.com/aki-akaguma/aki-resort/compare/v0.1.0..v0.1.1
[0.1.0]: https://github.com/aki-akaguma/aki-resort/releases/tag/v0.1.0
