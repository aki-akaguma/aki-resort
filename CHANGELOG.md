TBD: aki-resort
===
Unreleased changes. Release notes have not yet been written.

0.1.22 (2022-06-18)
=====

* changes to edition 2021
* update depends: flood-tide(0.2.5)
* update depends: memx(0.1.21), memx-cdy(0.1.8), runnel(0.3.11)
* update depends: exec-target(v0.2.6), flood-tide-gen(0.1.16)
* update depends: rust-version-info-file(v0.1.6)
* update depends: semver(1.0.10)
* update depends: crossbeam-channel(0.5.5)

0.1.21 (2022-05-22)
=====

* update depends: runnel(0.3.10), memx(0.1.20)
* update depends: anyhow(1.0.57), libc(0.2.126), regex(1.5.6), rayon(1.5.3)
* update depends: exec-target(v0.2.5), rust-version-info-file(v0.1.5)

0.1.20 (2021-12-18)
=====

* add according-to time.
* update depends: anyhow(1.0.51), libc(0.2.112)

0.1.19 (2021-11-15)
=====

* minimum support rustc 1.47.0 (18bf6b4f0 2020-10-07)
* add more documents
* update depends: flood-tide(0.2.4), memx(0.1.18), memx-cdy(0.1.7), runnel(0.3.9)
* update depends: anyhow(1.0.45), libc(0.2.107)
* update depends: exec-target(v0.2.4), flood-tide-gen(0.1.15), rust-version-info-file(v0.1.3)

0.1.18 (2021-09-11)
=====

* pass cargo clippy
* update depends: anyhow(1.0.43), flood-tide-gen(0.1.14), flood-tide(0.2.3), memx-cdy(0.1.6), runnel(0.3.8)
* rewite TARGET_EXE_PATH with `env!(concat!("CARGO_BIN_EXE_", env!("CARGO_PKG_NAME")))`
* update depends: exec-target(0.2.3)
* add depends: indoc(1.0.3)

0.1.17 (2021-06-24)
=====

* add `memx_cdy::memx_init(); // fast mem operation.`
* rewite TARGET_EXE_PATH with `env!("CARGO_BIN_EXE_aki-resort")`
* bug fix: `#[cfg(feature = "debian_build")]`

0.1.16 (2021-06-06)
=====

* update depends: semver(1.0.3)

0.1.15 (2021-06-03)
=====

* add support features = \["debian_build"\]
* bug fix command option: -X rust-version-info
* update depends: flood-tide(0.2.2)
* update depends: regex(1.5.4)

0.1.14 (2021-05-03)
=====

* bug fix: this arithmetic operation will overflow: 1024 * 1024 * 1024 * 1024 on i686
* add support 32bit cpus: i686, armv7, mipsel
* update depends: regex(1.5.3)

0.1.13 (2021-04-23)
=====

* fix build.rs

0.1.12 (2021-04-23)
=====

* update depends: flood-tide-gen(0.1.12), flood-tide(0.2.1)
* add command option: -X
* update depends: bug fix: regex(1.4.6)

0.1.11 (2021-04-19)
=====

* update depends: flood-tide-gen(0.1.10)

0.1.10 (2021-04-07)
=====

* update depends: flood-tide(0.2)
* update depends: anyhow(1.0.40), flood-tide-gen(0.1.8), runnnel(0.3.6)

0.1.9 (2021-04-01)
=====

* add option --head and --tail
* bug fix: should not coloring at the empty match.
* update depend: anyhow(1.0.40)

0.1.8 (2021-03-22)
=====

* update depend: regex v1.4.5: fixes stack overflows
* add --color <when>
* add some contents to --help

0.1.7 (2021-03-14)
=====

* update crate: regex: fix memory leak

0.1.6 (2021-03-08)
=====

* update crate: runnel
* update crate: rustc_version ("0.3")

0.1.5 (2021-03-08)
=====

* update crate: runnel

0.1.4 (2021-03-07)
=====

* use rayon::slice::ParallelSliceMut, for parallel sort
* rename file: xtask/src/cmd.txt to xtask/src/aki-resort-cmd.txt

0.1.3 (2021-03-06)
=====

* fix too large memory

0.1.2 (2021-03-05)
=====

* implement option: -u, --unique
* rename directory sort_key to sort
* implement option: --according-to vrsion
* implement option: --according-to month
* remove option: -k, --key <keydef>
* remove option: --field-separator <sep>
* add many doc

0.1.1 (2021-03-03)
=====

* change option '-e, --regex' to '-e, --exp'
* add examples to command help

0.1.0 (2021-03-01)
=====
first commit
