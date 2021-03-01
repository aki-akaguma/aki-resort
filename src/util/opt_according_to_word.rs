//{{{ OptAccordingToWord
#[derive(Debug, PartialEq, Clone, Copy)]
pub enum OptAccordingToWord {
    String,
    Numeric,
    Version,
    Month,
}

impl Default for OptAccordingToWord {
    fn default() -> OptAccordingToWord {
        OptAccordingToWord::String
    }
}

impl ::std::str::FromStr for OptAccordingToWord {
    type Err = OptAccordingToWordParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let oc = match s {
            "string" => OptAccordingToWord::String,
            "numeric" => OptAccordingToWord::Numeric,
            "version" => OptAccordingToWord::Version,
            "month" => OptAccordingToWord::Month,
            _ => {
                let s = format!("can not parse '{}'", s);
                return Err(OptAccordingToWordParseError::new(s));
            }
        };
        Ok(oc)
    }
}

impl ::std::fmt::Display for OptAccordingToWord {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        let s = match *self {
            OptAccordingToWord::String => "string",
            OptAccordingToWord::Numeric => "numeric",
            OptAccordingToWord::Version => "version",
            OptAccordingToWord::Month => "month",
        };
        write!(f, "{}", s)
    }
}
//}}} OptAccordingToWord

//{{{ OptAccordingToWordParseError
#[derive(Debug)]
pub struct OptAccordingToWordParseError {
    desc: String,
}

impl OptAccordingToWordParseError {
    fn new(s: String) -> OptAccordingToWordParseError {
        OptAccordingToWordParseError { desc: s }
    }
}

impl ::std::fmt::Display for OptAccordingToWordParseError {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        self.desc.fmt(f)
    }
}

impl ::std::error::Error for OptAccordingToWordParseError {
    fn description(&self) -> &str {
        self.desc.as_str()
    }
}
//}}} OptAccordingToWordParseError

#[cfg(test)]
mod tests {
    //use std::error::Error;
    use std::str::FromStr;

    use super::*;

    #[test]
    fn test_display_string() {
        let col = OptAccordingToWord::String;
        assert_eq!(format!("{}", col), "string");
    }
    #[test]
    fn test_display_numeric() {
        let col = OptAccordingToWord::Numeric;
        assert_eq!(format!("{}", col), "numeric");
    }
    #[test]
    fn test_display_version() {
        let col = OptAccordingToWord::Version;
        assert_eq!(format!("{}", col), "version");
    }
    #[test]
    fn test_display_month() {
        let col = OptAccordingToWord::Month;
        assert_eq!(format!("{}", col), "month");
    }
    #[test]
    fn test_from_str_string() {
        let col: OptAccordingToWord = match FromStr::from_str("string") {
            Ok(c) => c,
            Err(_) => {
                unreachable!();
            }
        };
        assert_eq!(col, OptAccordingToWord::String);
    }
    #[test]
    fn test_from_str_numeric() {
        let col: OptAccordingToWord = match FromStr::from_str("numeric") {
            Ok(c) => c,
            Err(_) => {
                unreachable!();
            }
        };
        assert_eq!(col, OptAccordingToWord::Numeric);
    }
    #[test]
    fn test_from_str_version() {
        let col: OptAccordingToWord = match FromStr::from_str("version") {
            Ok(c) => c,
            Err(_) => {
                unreachable!();
            }
        };
        assert_eq!(col, OptAccordingToWord::Version);
    }
    #[test]
    fn test_from_str_month() {
        let col: OptAccordingToWord = match FromStr::from_str("month") {
            Ok(c) => c,
            Err(_) => {
                unreachable!();
            }
        };
        assert_eq!(col, OptAccordingToWord::Month);
    }
    #[test]
    fn test_from_str_invalid() {
        let _col: OptAccordingToWord = match FromStr::from_str("other") {
            Ok(_c) => _c,
            Err(e) => {
                assert_eq!(e.to_string(), "can not parse \'other\'");
                return;
            }
        };
        unreachable!();
    }
}
