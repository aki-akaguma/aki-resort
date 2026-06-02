# Changelog: aki-resort

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Fixed
- Handle fractional seconds parsing correctly in `--according-to time`
- Correct time parsing when components (hours/minutes) are missing in `--according-to time`
- Prevent panic in tail processing when tail size exceeds total line count
- Ensure consistent sorting behavior with regex by placing unmatched lines at the end
- Ensure unique flag does not skip the first empty line
- Resolve `clippy::uninlined_format_args` warnings

## [0.2.1] - 2026-05-19

### Changed
- Support floating-point sorting using `f64` and `total_cmp` in `src/sort/numeric.rs`
- Remove aggressive `shrink_to_fit()` in `src/run.rs` for better performance
- Use `String::with_capacity()` for colored output in `src/run.rs`
- Remove unnecessary `Result` wraps in `src/sort/string.rs`
- Update dependencies: `flood-tide` (0.2.14), `flood-tide-gen` (0.2.2), `runnel` (0.4.2), and `regex` (1.12)
- Bump minimum supported Rust version to 1.68.0

### Fixed
- `clippy::uninlined_format_args` and `clippy::needless_borrow` warnings

### Removed
- `memx-cdy` dependency

## [0.2.0] - 2025-09-15

### Added
- Project specifications in `specs` directory
- Comprehensive test coverage
- `execute_with_env()` function

### Changed
- `IntoIterator` compatibility for arguments in `execute()`
- Update dependencies: `runnel` (0.4.0), `rust-version-info-file` (0.2), and `regex` (1.11)
- Downgrade `rayon` to `1.10.*`
- Refactor `src/run.rs` and `src/lib.rs` for better maintainability

### Fixed
- Handling of cases with no matches
- Minimum supported version in documentation

### Removed
- `execute_env()` function
- `base_dir=` from `-X` options

## [0.1.25] - 2024-06-19

### Added
- GitHub Actions workflows for Ubuntu, macOS, and Windows
- Test status badges in `README.tpl`
- Miri support for tests
- Tarpaulin support in `Makefile`

### Changed
- Rename `config` to `config.toml`
- Bump minimum supported Rust version to 1.65.0
- Refactor `Makefile`
- Update dependencies: `flood-tide` (0.2.11), `flood-tide-gen` (0.1.22), `memx-cdy` (0.1.13), `runnel` (0.3.19), `exec-target` (0.2.8), `indoc` (2.0.0), and `rust-version-info-file` (0.1.10)

### Removed
- `COPYING` file

### Fixed
- `LICENSE-APACHE` and `LICENSE-MIT` files
- License file consistency
- Clippy warnings: `redundant_static_lifetimes`, `needless_borrow`, `bool_assert_comparison`, `uninlined_format_args`, `unused_imports`, and `non_canonical_partial_ord_impl`
- Minimum supported Rust version from 1.56.0 to 1.58.0

## [0.1.24] - 2023-01-17

### Fixed
- Hour parsing error when encountering empty strings

## [0.1.23] - 2023-01-11

### Added
- Badges in `README.tpl`
- `rust-version = "1.56.0"` in `Cargo.toml`

### Changed
- Reformat `CHANGELOG.md`
- Update dependencies: `anyhow` (1.0.68), `flood-tide` (0.2.8), `flood-tide-gen` (0.1.19), `memx-cdy` (0.1.10), `runnel` (0.3.15), `regex` (1.7.1), `rayon` (1.6.1), and `semver` (1.0.16)

### Fixed
- Clippy warnings: `PartialEq` derivation without `Eq` implementation, and `uninlined_format_args`

## [0.1.22] - 2022-06-18

### Changed
- Migrate to Rust 2021 edition
- Update dependencies: `flood-tide` (0.2.5), `memx` (0.1.21), `memx-cdy` (0.1.8), `runnel` (0.3.11), `exec-target` (0.2.6), `flood-tide-gen` (0.1.16), `rust-version-info-file` (0.1.6), `semver` (1.0.10), and `crossbeam-channel` (0.5.5)

## [0.1.21] - 2022-05-22

### Changed
- Update dependencies: `runnel` (0.3.10), `memx` (0.1.20), `anyhow` (1.0.57), `libc` (0.2.126), `regex` (1.5.6), `rayon` (1.5.3), `exec-target` (0.2.5), and `rust-version-info-file` (0.1.5)

## [0.1.20] - 2021-12-18

### Added
- `--according-to time` command option

### Changed
- Update dependencies: `anyhow` (1.0.51) and `libc` (0.2.112)

## [0.1.19] - 2021-11-15

### Added
- Additional documentation

### Changed
- Bump minimum supported Rust version to 1.47.0
- Update dependencies: `flood-tide` (0.2.4), `memx` (0.1.18), `memx-cdy` (0.1.7), `runnel` (0.3.9), `anyhow` (1.0.45), `libc` (0.2.107), `exec-target` (0.2.4), `flood-tide-gen` (0.1.15), and `rust-version-info-file` (0.1.3)

## [0.1.18] - 2021-09-11

### Added
- `indoc` (1.0.3) dependency

### Changed
- Address all `cargo clippy` warnings
- Update dependencies: `anyhow` (1.0.43), `flood-tide-gen` (0.1.14), `flood-tide` (0.2.3), `memx-cdy` (0.1.6), `runnel` (0.3.8), and `exec-target` (0.2.3)
- Use `env!(concat!("CARGO_BIN_EXE_", env!("CARGO_PKG_NAME")))` for `TARGET_EXE_PATH`

## [0.1.17] - 2021-06-24

### Added
- Fast memory operations via `memx_cdy::memx_init()`

### Changed
- Use `env!("CARGO_BIN_EXE_aki-resort")` for `TARGET_EXE_PATH`

### Fixed
- Issue with `#[cfg(feature = "debian_build")]`

## [0.1.16] - 2021-06-06

### Changed
- Update dependencies: `semver` (1.0.3)

## [0.1.15] - 2021-06-03

### Added
- Support for `debian_build` feature

### Changed
- Update dependencies: `flood-tide` (0.2.2) and `regex` (1.5.4)

### Fixed
- Bug in `-X rust-version-info` command option

## [0.1.14] - 2021-05-03

### Added
- Support for 32-bit CPUs: i686, armv7, and mipsel

### Changed
- Update dependencies: `regex` (1.5.3)

### Fixed
- Arithmetic overflow on i686 (1024 * 1024 * 1024 * 1024)

## [0.1.13] - 2021-04-23

### Fixed
- Issues in `build.rs`

## [0.1.12] - 2021-04-23

### Added
- `-X` command option

### Changed
- Update dependencies: `flood-tide-gen` (0.1.12), `flood-tide` (0.2.1), and `regex` (1.4.6)

## [0.1.11] - 2021-04-19

### Changed
- Update dependencies: `flood-tide-gen` (0.1.10)

## [0.1.10] - 2021-04-07

### Changed
- Update dependencies: `flood-tide` (0.2), `anyhow` (1.0.40), `flood-tide-gen` (0.1.8), and `runnel` (0.3.6)

## [0.1.9] - 2021-04-01

### Added
- `--head` and `--tail` command options

### Changed
- Update dependencies: `anyhow` (1.0.40)

### Fixed
- Unwanted coloring on empty matches

## [0.1.8] - 2021-03-22

### Added
- `--color <when>` command option
- Additional content to `--help`

### Changed
- Update `regex` to v1.4.5 to resolve stack overflows

## [0.1.7] - 2021-03-14

### Changed
- Update `regex` to resolve a memory leak

## [0.1.6] - 2021-03-08

### Changed
- Update `runnel` and `rustc_version` (0.3) dependencies

## [0.1.5] - 2021-03-08

### Changed
- Update `runnel` dependency

## [0.1.4] - 2021-03-07

### Changed
- Use `rayon::slice::ParallelSliceMut` for parallel sorting
- Rename `xtask/src/cmd.txt` to `xtask/src/aki-resort-cmd.txt`

## [0.1.3] - 2021-03-06

### Fixed
- Excessive memory usage

## [0.1.2] - 2021-03-05

### Added
- `-u, --unique` option
- `--according-to version` option
- `--according-to month` option
- Extensive documentation

### Changed
- Rename `sort_key` directory to `sort`

### Removed
- `-k, --key <keydef>` option
- `--field-separator <sep>` option

## [0.1.1] - 2021-03-03

### Added
- Usage examples to command help

### Changed
- Rename `-e, --regex` option to `-e, --exp`

## [0.1.0] - 2021-03-01

- Initial release

[Unreleased]: https://github.com/aki-akaguma/aki-resort/compare/v0.2.1..HEAD
[0.2.1]: https://github.com/aki-akaguma/aki-resort/compare/v0.2.0..v0.2.1
[0.2.0]: https://github.com/aki-akaguma/aki-resort/compare/v0.1.25..v0.2.0
[0.1.25]: https://github.com/aki-akaguma/aki-resort/compare/v0.1.24..v0.1.25
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
