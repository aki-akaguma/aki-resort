pub use self::parse::parse_cmdopts;
use crate::util::OptUcXParam;
pub use parse::CmdOptConf;

mod parse;

impl CmdOptConf {
    pub fn is_opt_uc_x_help(&self) -> bool {
        for o in self.opt_uc_x.iter() {
            if let OptUcXParam::Help = o {
                return true;
            }
        }
        false
    }
    pub fn is_opt_uc_x_package_version_info(&self) -> bool {
        for o in self.opt_uc_x.iter() {
            if let OptUcXParam::RustVersionInfo = o {
                return true;
            }
        }
        false
    }
}

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

impl<IKV, K, V> From<IKV> for EnvConf
where
    IKV: IntoIterator<Item = (K, V)>,
    K: AsRef<std::ffi::OsStr>,
    V: AsRef<std::ffi::OsStr>,
{
    fn from(ary: IKV) -> Self {
        let mut r = Self::new();
        for a in ary {
            match a.0.as_ref().to_string_lossy().to_string().as_str() {
                "AKI_RESORT_COLOR_SEQ_ST" => {
                    r.color_seq_start = a.1.as_ref().to_string_lossy().to_string();
                }
                "AKI_RESORT_COLOR_SEQ_ED" => {
                    r.color_seq_end = a.1.as_ref().to_string_lossy().to_string();
                }
                _ => (),
            }
        }
        r
    }
}
