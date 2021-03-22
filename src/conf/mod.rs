pub use self::parse::parse_cmdopts;
pub use parse::CmdOptConf;

mod parse;

use std::env;

//
// ref.) 3-bit and 4-bit color sequence
//   https://en.wikipedia.org/wiki/ANSI_escape_code
// * black letters on white background use: ESC[30;47m
// * red use: ESC[31m
// * bright red use: ESC[1;31m
// * reset colors to their defaults: ESC[39;49m (not supported on some terminals)
// * reset all attributes: ESC[0m
//
static COLOR_START: &str = "\u{1B}[1;31m";
static COLOR_END: &str = "\u{1B}[0m";

#[derive(Debug)]
pub struct EnvConf {
    pub color_seq_start: String,
    pub color_seq_end: String,
}
impl EnvConf {
    pub fn new() -> Self {
        //
        let a_color_seq_start = match env::var("AKI_RESORT_COLOR_SEQ_ST") {
            Ok(v) => v,
            Err(_) => String::from(COLOR_START),
        };
        let a_color_seq_end = match env::var("AKI_RESORT_COLOR_SEQ_ED") {
            Ok(v) => v,
            Err(_) => String::from(COLOR_END),
        };
        //
        Self {
            color_seq_start: a_color_seq_start,
            color_seq_end: a_color_seq_end,
        }
    }
}
impl std::default::Default for EnvConf {
    fn default() -> EnvConf {
        EnvConf::new()
    }
}
