//{{{ OptColorWhen
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OptColorWhen {
    Always,
    Never,
    Auto,
}

impl Default for OptColorWhen {
    fn default() -> OptColorWhen {
        OptColorWhen::Auto
    }
}

impl ::std::str::FromStr for OptColorWhen {
    type Err = OptColorWhenParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let oc = match s {
            "always" => OptColorWhen::Always,
            "never" => OptColorWhen::Never,
            "auto" => OptColorWhen::Auto,
            _ => {
                let s = format!("can not parse '{}'", s);
                return Err(OptColorWhenParseError::new(s));
            }
        };
        Ok(oc)
    }
}

impl ::std::fmt::Display for OptColorWhen {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        let s = match *self {
            OptColorWhen::Always => "always",
            OptColorWhen::Never => "never",
            OptColorWhen::Auto => "auto",
        };
        write!(f, "{}", s)
    }
}
//}}} OptColorWhen

//{{{ OptColorWhenParseError
#[derive(Debug)]
pub struct OptColorWhenParseError {
    desc: String,
}

impl OptColorWhenParseError {
    fn new(s: String) -> OptColorWhenParseError {
        OptColorWhenParseError { desc: s }
    }
}

impl ::std::fmt::Display for OptColorWhenParseError {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        self.desc.fmt(f)
    }
}

impl ::std::error::Error for OptColorWhenParseError {
    fn description(&self) -> &str {
        self.desc.as_str()
    }
}
//}}} OptColorWhenParseError

#[cfg(test)]
mod tests {
    //use std::error::Error;
    use std::str::FromStr;

    use super::*;

    #[test]
    fn test_display_always() {
        let col = OptColorWhen::Always;
        assert_eq!(format!("{}", col), "always");
    }
    #[test]
    fn test_display_never() {
        let col = OptColorWhen::Never;
        assert_eq!(format!("{}", col), "never");
    }
    #[test]
    fn test_display_auto() {
        let col = OptColorWhen::Auto;
        assert_eq!(format!("{}", col), "auto");
    }
    #[test]
    fn test_from_str_always() {
        let col: OptColorWhen = match FromStr::from_str("always") {
            Ok(c) => c,
            Err(_) => {
                unreachable!();
            }
        };
        assert_eq!(col, OptColorWhen::Always);
    }
    #[test]
    fn test_from_str_never() {
        let col: OptColorWhen = match FromStr::from_str("never") {
            Ok(c) => c,
            Err(_) => {
                unreachable!();
            }
        };
        assert_eq!(col, OptColorWhen::Never);
    }
    #[test]
    fn test_from_str_auto() {
        let col: OptColorWhen = match FromStr::from_str("auto") {
            Ok(c) => c,
            Err(_) => {
                unreachable!();
            }
        };
        assert_eq!(col, OptColorWhen::Auto);
    }
    #[test]
    fn test_from_str_invalid() {
        let _col: OptColorWhen = match FromStr::from_str("other") {
            Ok(_c) => _c,
            Err(e) => {
                assert_eq!(e.to_string(), "can not parse \'other\'");
                return;
            }
        };
        unreachable!();
    }
}
