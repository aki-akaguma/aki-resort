//! sort lines of text. You can use regex to specify the KEY.
//!
//! ```text
//! Usage:
//!   aki-resort [options]
//!
//! sort lines of text.
//!
//! Ordering options:
//!   -r, --reverse                 reverse the result of comparisons
//!       --according-to <word>     sort according to WORD: string, numeric, month, version
//!
//! Other options:
//!   -e, --exp <exp>               regular expression. sort via this match point.
//!   -u, --unique                  output only the first line of an equal.
//!       --max-buffer <size>       max buffer size. if reading size is more than <size>, then it not output, quit and display error message.
//!
//!   -H, --help        display this help and exit
//!   -V, --version     display version information and exit
//!
//! Examples:
//!   This sort via utf-8 code:
//!     cat file1.txt | aki-resort
//!   This sort via 1st chunk of numeric character according to numeric:
//!     cat file1.txt | aki-resort -e "[0-9]+" --according-to numeric
//! ```
//!
//! # Examples
//!
//! The input data used in this example looks like this:
//!
//! ```text
//! cat fixtures/fruit.txt
//! ```
//!
//! result output:
//! ```text
//! Apple:33:3.3:good:Mar
//! Orange:222:1.1.2:good:Jan
//! Cherry:4:4:good:Oct
//! Kiwi:1111:1.1.11:good:Jun
//! ```
//!
//! ## Example 1: simple sort
//!
//! This sort via utf-8 code.
//!
//! command line:
//! ```text
//! cat fixtures/fruit.txt | aki-resort
//! ```
//!
//! result output:
//! ```text
//! Apple:33:3.3:good:Mar
//! Cherry:4:4:good:Oct
//! Kiwi:1111:1.1.11:good:Jun
//! Orange:222:1.1.2:good:Jan
//! ```
//!
//! ## Example 2: numeric sort
//!
//! This sort via 1st chunk of numeric character according to numeric.
//!
//! command line:
//! ```text
//! cat fixtures/fruit.txt | aki-resort -e "[0-9]+" --according-to numeric
//! ```
//!
//! result output:
//! ```text
//! Cherry:4:4:good:Oct
//! Apple:33:3.3:good:Mar
//! Orange:222:1.1.2:good:Jan
//! Kiwi:1111:1.1.11:good:Jun
//! ```
//!
//! ## Example 3: version sort
//!
//! This sort via 1st capture of version character according to version.
//!
//! command line:
//! ```text
//! cat fixtures/fruit.txt | aki-resort -e "^[^:]+:[^:]+:([^:]+)" --according-to version
//! ```
//!
//! result output:
//! ```text
//! Orange:222:1.1.2:good:Jan
//! Kiwi:1111:1.1.11:good:Jun
//! Apple:33:3.3:good:Mar
//! Cherry:4:4:good:Oct
//! ```
//!
//! ## Example 4: month sort
//!
//! This sort via 1st capture of month character according to month.
//!
//! command line:
//! ```text
//! cat fixtures/fruit.txt | aki-resort -e ":([^:]+)$" --according-to month
//! ```
//!
//! result output:
//! ```text
//! Orange:222:1.1.2:good:Jan
//! Apple:33:3.3:good:Mar
//! Kiwi:1111:1.1.11:good:Jun
//! Cherry:4:4:good:Oct
//! ```
//!
//! # Library example
//!
//! See [`fn execute()`] for this library examples.
//!
//! [`fn execute()`]: crate::execute

#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate anyhow;

mod conf;
mod run;
mod sort;
mod util;

use flood_tide::HelpVersion;
use runnel::RunnelIoe;
use std::io::Write;

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
///     "resort", &[]);
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
///     "resort", &["-e", r"[0-9]+", "--according-to", "numeric"]);
/// ```
///
pub fn execute(sioe: &RunnelIoe, prog_name: &str, args: &[&str]) -> anyhow::Result<()> {
    let conf = match conf::parse_cmdopts(prog_name, args) {
        Ok(conf) => conf,
        Err(errs) => {
            for err in errs.iter().take(1) {
                if err.is_help() || err.is_version() {
                    let _r = sioe.pout().lock().write_fmt(format_args!("{}\n", err));
                    return Ok(());
                }
            }
            return Err(anyhow!("{}\n{}", errs, TRY_HELP_MSG));
        }
    };
    run::run(sioe, &conf)
}
