//
use crate::util::OptAccordingToWord;
use crate::util::OptMaxBufferSize;
use flood_tide::parse_simple_gnu_style;
use flood_tide::HelpVersion;
use flood_tide::{Arg, NameVal, Opt, OptNum};
use flood_tide::{OptParseError, OptParseErrors};
use std::str::FromStr;

//----------------------------------------------------------------------
include!("cmd.help.rs.txt");

//{{{ TEXT
const DESCRIPTIONS_TEXT: &str = r#"
sort lines of text.
"#;
/*
const ARGUMENTS_TEXT: &str = r#"Argument:
  <url>                     url to getting, protocol is http or ftp
"#;
*/
const EXAMPLES_TEXT: &str = r#"Examples:
  This sort via utf-8 code:
    cat file1.txt | aki-resort
  This sort via 1st chunk of numeric character according to numeric:
    cat file1.txt | aki-resort -e "[0-9]+" --according-to numeric
"#;
//}}} TEXT

//----------------------------------------------------------------------
#[rustfmt::skip]
fn version_message(_program: &str) -> String {
    format!( "{} {}",
        env!("CARGO_PKG_NAME"), env!("CARGO_PKG_VERSION"))
}

#[rustfmt::skip]
fn usage_message(program: &str) -> String {
    format!("Usage:\n  {} {}", program, "[options]")
}

#[rustfmt::skip]
fn help_message(program: &str) -> String {
    let ver = version_message(program);
    let usa = usage_message(env!("CARGO_PKG_NAME"));
    [ &ver, "", &usa, DESCRIPTIONS_TEXT, OPTIONS_TEXT, EXAMPLES_TEXT].join("\n")
}

//----------------------------------------------------------------------
fn value_to_opt_according_to_word(nv: &NameVal<'_>) -> Result<OptAccordingToWord, OptParseError> {
    match nv.val {
        Some(s) => match FromStr::from_str(s) {
            Ok(word) => Ok(word),
            Err(err) => Err(OptParseError::invalid_option_argument(
                &nv.opt.lon,
                &err.to_string(),
            )),
        },
        None => Err(OptParseError::missing_option_argument(&nv.opt.lon)),
    }
}

fn value_to_opt_max_buffer_size(nv: &NameVal<'_>) -> Result<OptMaxBufferSize, OptParseError> {
    match nv.val {
        Some(s) => match FromStr::from_str(s) {
            Ok(word) => Ok(word),
            Err(err) => Err(OptParseError::invalid_option_argument(
                &nv.opt.lon,
                &err.to_string(),
            )),
        },
        None => Err(OptParseError::missing_option_argument(&nv.opt.lon)),
    }
}

//----------------------------------------------------------------------
fn parse_match(conf: &mut CmdOptConf, nv: &NameVal<'_>) -> Result<(), OptParseError> {
    include!("cmd.match.rs.txt");
    Ok(())
}

pub fn parse_cmdopts(a_prog_name: &str, args: &[&str]) -> Result<CmdOptConf, OptParseErrors> {
    //
    let mut conf = CmdOptConf {
        prog_name: a_prog_name.to_string(),
        ..Default::default()
    };
    let (opt_free, r_errs) =
        parse_simple_gnu_style(&mut conf, &OPT_ARY, &OPT_ARY_SHO_IDX, args, parse_match);
    //
    if conf.is_help() {
        let mut errs = OptParseErrors::new();
        errs.push(OptParseError::help_message(&help_message(&conf.prog_name)));
        return Err(errs);
    }
    if conf.is_version() {
        let mut errs = OptParseErrors::new();
        errs.push(OptParseError::version_message(&version_message(
            &conf.prog_name,
        )));
        return Err(errs);
    }
    //
    {
        let mut errs = if let Err(errs) = r_errs {
            errs
        } else {
            OptParseErrors::new()
        };
        //
        /*
        if conf.opt_expression.is_empty() {
            errs.push(OptParseError::missing_option("e"));
        }
        if conf.opt_expression.len() != conf.opt_format.len() {
            errs.push(OptParseError::missing_option("e or f"));
        }
        */
        //
        if let Some(free) = opt_free {
            if !free.is_empty() {
                errs.push(OptParseError::unexpected_argument(&free[0]));
            }
        };
        if !errs.is_empty() {
            return Err(errs);
        }
    }
    //
    Ok(conf)
}
