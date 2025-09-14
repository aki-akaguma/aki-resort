/*!
sort lines of text. You can use regex to specify the KEY.

# Features

- sort lines of text. You can use regex to specify the KEY.
- minimum support rustc 1.65.0 (897e37553 2022-11-02)

# Command help

```text
aki-resort --help
```

```text
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

# Quick install

1. you can install this into cargo bin path:

```text
cargo install aki-resort
```

2. you can build debian package:

```text
cargo deb
```

and install **.deb** into your local repository of debian package.

# Examples

The input data used in this example looks like this:

```text
cat fixtures/fruit.txt
```

result output:
```text
Apple:33:3.3:good:Mar
Orange:222:1.1.2:good:Jan
Cherry:4:4:good:Oct
Kiwi:1111:1.1.11:good:Jun
```

## Example 1: simple sort

This sort via utf-8 code.

command line:
```text
cat fixtures/fruit.txt | aki-resort
```

result output:
```text
Apple:33:3.3:good:Mar
Cherry:4:4:good:Oct
Kiwi:1111:1.1.11:good:Jun
Orange:222:1.1.2:good:Jan
```

## Example 2: numeric sort

This sort via 1st chunk of numeric character according to numeric.

command line:
```text
cat fixtures/fruit.txt | aki-resort -e "[0-9]+" --according-to numeric
```

result output:
```text
Cherry:4:4:good:Oct
Apple:33:3.3:good:Mar
Orange:222:1.1.2:good:Jan
Kiwi:1111:1.1.11:good:Jun
```

## Example 3: version sort

This sort via 1st capture of version character according to version.

command line:
```text
cat fixtures/fruit.txt | aki-resort -e "^[^:]+:[^:]+:([^:]+)" --according-to version
```

result output:
```text
Orange:222:1.1.2:good:Jan
Kiwi:1111:1.1.11:good:Jun
Apple:33:3.3:good:Mar
Cherry:4:4:good:Oct
```

## Example 4: month sort

This sort via 1st capture of month character according to month.

command line:
```text
cat fixtures/fruit.txt | aki-resort -e ":([^:]+)$" --according-to month
```

result output:
```text
Orange:222:1.1.2:good:Jan
Apple:33:3.3:good:Mar
Kiwi:1111:1.1.11:good:Jun
Cherry:4:4:good:Oct
```

## Example 5: time sort

This sort via 1st capture of numeric time character according to time.

command line:
```text
cat fixtures/fruit.txt | aki-resort -e "([0-9]+:([0-9]+:)?[0-9]+(.[0-9]+)?)" --according-to time
```

result output:
```text
Cherry:4:4:good:Oct
Apple:33:3.3:good:Mar
Orange:222:1.1.2:good:Jan
Kiwi:1111:1.1.11:good:Jun
```

## Example 6: numeric sort with the header

This sort via 1st chunk of numeric character according to numeric.
And the 1st line is the fixed header.

command line:
```text
cat fixtures/fruit_header.txt | aki-resort -e "[0-9]+" --according-to numeric -h 1
```

result output:
```text
name:number:version:nice:month
Cherry:4:4:good:Oct
Apple:33:3.3:good:Mar
Orange:222:1.1.2:good:Jan
Kiwi:1111:1.1.11:good:Jun
```

# Library example

See [`fn execute()`] for this library examples.

[`fn execute()`]: crate::execute
*/
#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate anyhow;

pub mod conf;
mod run;
mod sort;

#[macro_use]
mod util;

use flood_tide::HelpVersion;
use runnel::RunnelIoe;

const TRY_HELP_MSG: &str = "Try --help for help.";

/// execute resort
///
/// params:
///   - sioe: stream in/out/err
///   - program: program name. etc. "resort"
///   - args: parameter arguments.
///
/// return:
///   - ok: ()
///   - err: anyhow
///
/// # Examples
///
/// ## Example 1: simple sort
///
/// This sort via utf-8 code.
///
/// ```rust
/// use runnel::RunnelIoeBuilder;
///
/// let r = libaki_resort::execute(&RunnelIoeBuilder::new().build(),
///     "resort", &[] as &[&str]);
/// ```
///
/// ## Example 2: numeric sort
///
/// This sort via 1st chunk of numeric character according to numeric.
///
/// ```rust
/// use runnel::RunnelIoeBuilder;
///
/// let r = libaki_resort::execute(&RunnelIoeBuilder::new().build(),
///     "resort", ["-e", r"[0-9]+", "--according-to", "numeric"]);
/// ```
///
pub fn execute<I, S>(sioe: &RunnelIoe, prog_name: &str, args: I) -> anyhow::Result<()>
where
    I: IntoIterator<Item = S>,
    S: AsRef<std::ffi::OsStr>,
{
    execute_with_env(sioe, prog_name, args, vec![("", "")])
}

/// execute resort with environments
///
/// params:
///   - sioe: stream in/out/err
///   - program: program name. etc. "resort"
///   - env: environments array.
///
/// return:
///   - ok: ()
///   - err: anyhow
///
/// # Examples
///
/// ## Example 1: simple sort
///
/// This sort via utf-8 code.
///
/// ```rust
/// use runnel::RunnelIoeBuilder;
///
/// let r = libaki_resort::execute_with_env(&RunnelIoeBuilder::new().build(),
///     "resort",
///     ["--color", "always"],
///     vec![
///         ("AKI_RESORT_COLOR_SEQ_ST", "<S>"),
///         ("AKI_RESORT_COLOR_SEQ_ED", "<E>"),
///     ]
/// );
/// ```
///
/// ## Example 2: numeric sort
///
/// This sort via 1st chunk of numeric character according to numeric.
///
/// ```rust
/// use runnel::RunnelIoeBuilder;
///
/// let r = libaki_resort::execute_with_env(&RunnelIoeBuilder::new().build(),
///     "resort",
///     [
///         "-e", r"[0-9]+",
///         "--according-to", "numeric",
///         "--color", "always"
///     ],
///     vec![
///         ("AKI_RESORT_COLOR_SEQ_ST", "<S>"),
///         ("AKI_RESORT_COLOR_SEQ_ED", "<E>"),
///     ]
/// );
/// ```
///
pub fn execute_with_env<I, S, IKV, K, V>(
    sioe: &RunnelIoe,
    prog_name: &str,
    args: I,
    env: IKV,
) -> anyhow::Result<()>
where
    I: IntoIterator<Item = S>,
    S: AsRef<std::ffi::OsStr>,
    IKV: IntoIterator<Item = (K, V)>,
    K: AsRef<std::ffi::OsStr>,
    V: AsRef<std::ffi::OsStr>,
{
    let args: Vec<String> = args
        .into_iter()
        .map(|s| s.as_ref().to_string_lossy().into_owned())
        .collect();
    let args_str: Vec<&str> = args.iter().map(|s| s.as_str()).collect();
    let env_cnf: conf::EnvConf = env.into();
    //
    match conf::parse_cmdopts(prog_name, &args_str) {
        Ok(conf) => run::run(sioe, &conf, &env_cnf),
        Err(errs) => {
            if let Some(err) = errs.iter().find(|e| e.is_help() || e.is_version()) {
                sioe.pg_out().write_line(err.to_string())?;
                Ok(())
            } else {
                Err(anyhow!("{errs}\n{TRY_HELP_MSG}"))
            }
        }
    }
}
