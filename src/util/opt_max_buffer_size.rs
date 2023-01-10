use regex::Regex;
use std::convert::TryInto;

//{{{ OptMaxBufferSize
#[derive(Default, Debug, Clone, Copy, PartialEq, Eq)]
pub struct OptMaxBufferSize(usize);
impl OptMaxBufferSize {
    pub fn new(v: usize) -> Self {
        Self(v)
    }
    pub fn is_ok(&self, v: usize) -> bool {
        if self.0 == 0 {
            true
        } else {
            v <= self.0
        }
    }
}

impl ::std::str::FromStr for OptMaxBufferSize {
    type Err = OptMaxBufferSizeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        lazy_static! {
            static ref RE: Regex = Regex::new("([0-9]+)([KMGTPkmgtp])?[bB]?").unwrap();
        }
        if let Some(caps) = RE.captures(s) {
            let digit: usize = if let Some(mat) = caps.get(1) {
                match mat.as_str().parse::<usize>() {
                    Ok(digit) => digit,
                    Err(err) => {
                        let s = format!("can not parse '{s}': {err}");
                        return Err(OptMaxBufferSizeParseError::new(s));
                    }
                }
            } else {
                let s = format!("can not parse '{s}'");
                return Err(OptMaxBufferSizeParseError::new(s));
            };
            let unit: usize = if let Some(mat) = caps.get(2) {
                let un: u64 = match mat.as_str() {
                    "K" | "k" => 1024,
                    "M" | "m" => 1024 * 1024,
                    "G" | "g" => 1024 * 1024 * 1024,
                    "T" | "t" => 1024 * 1024 * 1024 * 1024,
                    "P" | "p" => 1024 * 1024 * 1024 * 1024 * 1024,
                    _ => 1,
                };
                if un > usize::MAX.try_into().unwrap()
                    || digit as u64 * un > usize::MAX.try_into().unwrap()
                {
                    let s = format!("can not parse '{s}': overflow");
                    return Err(OptMaxBufferSizeParseError::new(s));
                } else {
                    un.try_into().unwrap()
                }
            } else {
                1
            };
            Ok(OptMaxBufferSize::new(digit * unit))
        } else {
            let s = format!("can not parse '{s}'");
            Err(OptMaxBufferSizeParseError::new(s))
        }
    }
}

impl ::std::fmt::Display for OptMaxBufferSize {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
//}}} OptMaxBufferSize

//{{{ OptMaxBufferSizeParseError
#[derive(Debug)]
pub struct OptMaxBufferSizeParseError {
    desc: String,
}

impl OptMaxBufferSizeParseError {
    fn new(s: String) -> OptMaxBufferSizeParseError {
        OptMaxBufferSizeParseError { desc: s }
    }
}

impl ::std::fmt::Display for OptMaxBufferSizeParseError {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        self.desc.fmt(f)
    }
}

impl ::std::error::Error for OptMaxBufferSizeParseError {
    fn description(&self) -> &str {
        self.desc.as_str()
    }
}
//}}} OptMaxBufferSizeParseError

#[cfg(test)]
mod tests {
    use super::*;
    use std::str::FromStr;

    #[test]
    fn test_display_0() {
        let v = OptMaxBufferSize::new(0);
        assert_eq!(format!("{}", v), "0");
    }
    #[test]
    fn test_display_1024() {
        let v = OptMaxBufferSize::new(1024);
        assert_eq!(format!("{}", v), "1024");
    }
    #[test]
    fn test_from_str_123() {
        let col: OptMaxBufferSize = match FromStr::from_str("123") {
            Ok(c) => c,
            Err(_) => {
                unreachable!();
            }
        };
        assert_eq!(col, OptMaxBufferSize::new(123));
    }
    #[test]
    fn test_from_str_123k() {
        let col: OptMaxBufferSize = match FromStr::from_str("123k") {
            Ok(c) => c,
            Err(_) => {
                unreachable!();
            }
        };
        assert_eq!(col, OptMaxBufferSize::new(123 * 1024));
    }
    #[test]
    fn test_from_str_123m() {
        let col: OptMaxBufferSize = match FromStr::from_str("123m") {
            Ok(c) => c,
            Err(_) => {
                unreachable!();
            }
        };
        assert_eq!(col, OptMaxBufferSize::new(123 * 1024 * 1024));
    }
    #[cfg(target_pointer_width = "64")]
    #[test]
    fn test_from_str_123g() {
        let _col: OptMaxBufferSize = match FromStr::from_str("123g") {
            Ok(c) => c,
            Err(_) => {
                unreachable!();
            }
        };
        #[cfg(target_pointer_width = "64")]
        assert_eq!(_col, OptMaxBufferSize::new(123 * 1024 * 1024 * 1024));
    }
    #[cfg(target_pointer_width = "32")]
    #[test]
    fn test_from_str_123g() {
        match FromStr::from_str("123g") {
            Ok(c) => {
                let _col: OptMaxBufferSize = c;
                unreachable!();
            }
            Err(err) => {
                assert_eq!(format!("{}", err), "can not parse \'123g\': overflow");
            }
        };
    }
    #[cfg(target_pointer_width = "64")]
    #[test]
    fn test_from_str_123t() {
        let _col: OptMaxBufferSize = match FromStr::from_str("123t") {
            Ok(c) => c,
            Err(_) => {
                unreachable!();
            }
        };
        assert_eq!(_col, OptMaxBufferSize::new(123 * 1024 * 1024 * 1024 * 1024));
    }
    #[cfg(target_pointer_width = "32")]
    #[test]
    fn test_from_str_123t() {
        match FromStr::from_str("123t") {
            Ok(c) => {
                let _col: OptMaxBufferSize = c;
                unreachable!();
            }
            Err(err) => {
                assert_eq!(format!("{}", err), "can not parse \'123t\': overflow");
            }
        };
    }
    #[cfg(target_pointer_width = "64")]
    #[test]
    fn test_from_str_123p() {
        let _col: OptMaxBufferSize = match FromStr::from_str("123p") {
            Ok(c) => c,
            Err(_) => {
                unreachable!();
            }
        };
        #[cfg(target_pointer_width = "64")]
        assert_eq!(
            _col,
            OptMaxBufferSize::new(123 * 1024 * 1024 * 1024 * 1024 * 1024)
        );
    }
    #[cfg(target_pointer_width = "32")]
    #[test]
    fn test_from_str_123p() {
        match FromStr::from_str("123p") {
            Ok(c) => {
                let _col: OptMaxBufferSize = c;
                unreachable!();
            }
            Err(err) => {
                assert_eq!(format!("{}", err), "can not parse \'123p\': overflow");
            }
        };
    }
    #[test]
    fn test_from_str_invalid() {
        let _col: OptMaxBufferSize = match FromStr::from_str("other") {
            Ok(_c) => _c,
            Err(e) => {
                assert_eq!(e.to_string(), "can not parse \'other\'");
                return;
            }
        };
        unreachable!();
    }
}
