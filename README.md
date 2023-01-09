# aki-resort

[![crate][crate-image]][crate-link]
[![Docs][docs-image]][docs-link]
![Rust Version][rustc-image]
![Apache2/MIT licensed][license-image]

sort lines of text. You can use regex to specify the KEY.

## Features

- sort lines of text. You can use regex to specify the KEY.
- minimum support rustc 1.56.1 (59eed8a2a 2021-11-01)

## Command help

```
aki-resort --help
```

```
Usage:
  aki-resort [options]

sort lines of text.

Ordering options:
  -r, --reverse                 reverse the result of comparisons
      --according-to <word>     sort according to <word>
  -h, --head <num>              unsort the first <num> lines.
  -t, --tail <num>              unsort the last <num> lines.

Other options:
      --color <when>            use markers to highlight the matching strings
  -e, --exp <exp>               regular expression. sort by the entires match
  -u, --unique                  output only the first line of an equal
      --max-buffer <size>       max buffer size

  -H, --help        display this help and exit
  -V, --version     display version information and exit

Option Parameters:
  <word>    'month', 'numeric', 'string', 'time', 'version'
  <when>    'always', 'never', or 'auto'
  <exp>     regular expression, sort by the entires match.
  <size>    if a reading size is more than <size>, then it is not output,
            quit and display error message.

Environments:
  AKI_RESORT_COLOR_SEQ_ST   color start sequence specified by ansi
  AKI_RESORT_COLOR_SEQ_ED   color end sequence specified by ansi

Examples:
  This sort via utf-8 code:
    cat file1.txt | aki-resort
  This sort via 1st chunk of numeric character according to numeric:
    cat file1.txt | aki-resort -e "[0-9]+" --according-to numeric
  This sort via 1st chunk of numeric character according to month:
    cat file1.txt | aki-resort -e ":([^:]+)$" --according-to month
  This sort via 1st chunk of numeric version character according to version:
    cat file1.txt | aki-resort -e "[^:]+:[^:]+:([0-9.]+):" --according-to version
  This sort via 1st chunk of numeric time character according to time:
    cat file1.txt | aki-resort -e "([0-9]+:([0-9]+:)?[0-9]+(.[0-9]+)?)" --according-to time
```

## Quick install

1. you can install this into cargo bin path:

```
cargo install aki-resort
```

2. you can build debian package:

```
cargo deb
```

and install **.deb** into your local repository of debian package.

## Examples

The input data used in this example looks like this:

```
cat fixtures/fruit.txt
```

result output:
```
Apple:33:3.3:good:Mar
Orange:222:1.1.2:good:Jan
Cherry:4:4:good:Oct
Kiwi:1111:1.1.11:good:Jun
```

### Example 1: simple sort

This sort via utf-8 code.

command line:
```
cat fixtures/fruit.txt | aki-resort
```

result output:
```
Apple:33:3.3:good:Mar
Cherry:4:4:good:Oct
Kiwi:1111:1.1.11:good:Jun
Orange:222:1.1.2:good:Jan
```

### Example 2: numeric sort

This sort via 1st chunk of numeric character according to numeric.

command line:
```
cat fixtures/fruit.txt | aki-resort -e "[0-9]+" --according-to numeric
```

result output:
```
Cherry:4:4:good:Oct
Apple:33:3.3:good:Mar
Orange:222:1.1.2:good:Jan
Kiwi:1111:1.1.11:good:Jun
```

### Example 3: version sort

This sort via 1st capture of version character according to version.

command line:
```
cat fixtures/fruit.txt | aki-resort -e "^[^:]+:[^:]+:([^:]+)" --according-to version
```

result output:
```
Orange:222:1.1.2:good:Jan
Kiwi:1111:1.1.11:good:Jun
Apple:33:3.3:good:Mar
Cherry:4:4:good:Oct
```

### Example 4: month sort

This sort via 1st capture of month character according to month.

command line:
```
cat fixtures/fruit.txt | aki-resort -e ":([^:]+)$" --according-to month
```

result output:
```
Orange:222:1.1.2:good:Jan
Apple:33:3.3:good:Mar
Kiwi:1111:1.1.11:good:Jun
Cherry:4:4:good:Oct
```

### Example 5: time sort

This sort via 1st capture of numeric time character according to time.

command line:
```
cat fixtures/fruit.txt | aki-resort -e "([0-9]+:([0-9]+:)?[0-9]+(.[0-9]+)?)" --according-to time
```

result output:
```
Cherry:4:4:good:Oct
Apple:33:3.3:good:Mar
Orange:222:1.1.2:good:Jan
Kiwi:1111:1.1.11:good:Jun
```

### Example 6: numeric sort with the header

This sort via 1st chunk of numeric character according to numeric.
And the 1st line is the fixed header.

command line:
```
cat fixtures/fruit_header.txt | aki-resort -e "[0-9]+" --according-to numeric -h 1
```

result output:
```
name:number:version:nice:month
Cherry:4:4:good:Oct
Apple:33:3.3:good:Mar
Orange:222:1.1.2:good:Jan
Kiwi:1111:1.1.11:good:Jun
```

## Library example

See [`fn execute()`] for this library examples.

[`fn execute()`]: crate::execute

# Changelogs

[This crate's changelog here.](https://github.com/aki-akaguma/aki-resort/blob/main/CHANGELOG.md)

# License

This project is licensed under either of

 * Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or
   https://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or
   https://opensource.org/licenses/MIT)

at your option.

[//]: # (badges)

[crate-image]: https://img.shields.io/crates/v/aki-resort.svg
[crate-link]: https://crates.io/crates/aki-resort
[docs-image]: https://docs.rs/aki-resort/badge.svg
[docs-link]: https://docs.rs/aki-resort/
[rustc-image]: https://img.shields.io/badge/rustc-1.56+-blue.svg
[license-image]: https://img.shields.io/badge/license-Apache2.0/MIT-blue.svg
