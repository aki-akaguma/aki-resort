aki-resort TBD
===
Unreleased changes. Release notes have not yet been written.

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
