# aki-resort

*aki-resort* sort lines of text.

## Features

*aki-resort*  sort lines of text. You can use regex to specify the KEY.

* command help

```text
aki-resort --help
```

```text
Usage:
  aki-resort [options]

sort lines of text.

Ordering options:
  -r, --reverse                 reverse the result of comparisons
      --according-to <word>     sort according to WORD: string, numeric, month, version

Other options:
  -e, --exp <exp>               regular expression. sort via this match point.
  -k, --key <keydef>            sort via a key. keydef gives location.
      --field-separator <sep>   use <sep> instead of non-blank to blank transition
  -u, --unique                  output only the first line of an equal.
      --max-buffer <size>       max buffer size. if reading size is more than <size>, then it not output, quit and display error message.

  -H, --help        display this help and exit
  -V, --version     display version information and exit

Examples:
  This sort via utf-8 code:
    cat file1.txt | aki-resort
  This sort via 1st chunk of numeric character according to numeric:
    cat file1.txt | aki-resort -e "[0-9]+" --according-to numeric
```

* minimum support rustc 1.38.0

## Quick install

1. you can install this into cargo bin path:

```text
cargo install aki-resort
```

2. you can build debian package:

```text
cargo deb
```

and install **.deb** into your local repository of debian package.

## Examples

The input data used in this example looks like this:

```
cat file1.txt
```

result output:
```
ABCDEFG:33:abc
OPQRSTU:222:opq
VWXYZ:4:vwx
HIJKLMN:1111:hij
```

### Example 1: simple sort

This sort via utf-8 code.

command line:
```
cat file1.txt | aki-resort
```

result output:
```
ABCDEFG:33:abc
HIJKLMN:1111:hij
OPQRSTU:222:opq
VWXYZ:4:vwx
```

### Example 2: numeric sort

This sort via 1st chunk of numeric character according to numeric.

command line:
```
cat file1.txt | aki-resort -e "[0-9]+" --according-to numeric
```

result output:
```
VWXYZ:4:vwx
ABCDEFG:33:abc
OPQRSTU:222:opq
HIJKLMN:1111:hij
```

## Library example

See [`fn execute()`] for this library examples.

[`fn execute()`]: crate::execute
