//{{{ OptUcXParam
#[derive(Debug, PartialEq, Clone)]
pub enum OptUcXParam {
    Void,
    Help,
    PackageVersionInfo,
    BaseDir(String),
}

impl Default for OptUcXParam {
    fn default() -> OptUcXParam {
        OptUcXParam::Void
    }
}

impl ::std::str::FromStr for OptUcXParam {
    type Err = OptUcXParamParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let oc = match s {
            "void" => OptUcXParam::Void,
            "help" => OptUcXParam::Help,
            "package-version-info" => OptUcXParam::PackageVersionInfo,
            _ => {
                let bs = "base_dir=";
                if let Some(stripped) = s.strip_prefix(bs) {
                    OptUcXParam::BaseDir(stripped.to_string())
                } else {
                    let s = format!("can not parse '{}'", s);
                    return Err(OptUcXParamParseError::new(s));
                }
            }
        };
        Ok(oc)
    }
}

impl ::std::fmt::Display for OptUcXParam {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        let s = match *self {
            OptUcXParam::Void => "void",
            OptUcXParam::Help => "help",
            OptUcXParam::PackageVersionInfo => "package-version-info",
            OptUcXParam::BaseDir(_) => "base_dir=",
        };
        write!(f, "{}", s)
    }
}
//}}} OptUcXParam

//{{{ OptUcXParamParseError
#[derive(Debug)]
pub struct OptUcXParamParseError {
    desc: String,
}

impl OptUcXParamParseError {
    fn new(s: String) -> OptUcXParamParseError {
        OptUcXParamParseError { desc: s }
    }
}

impl ::std::fmt::Display for OptUcXParamParseError {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        self.desc.fmt(f)
    }
}

impl ::std::error::Error for OptUcXParamParseError {
    fn description(&self) -> &str {
        self.desc.as_str()
    }
}
//}}} OptUcXParamParseError

/*
#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use super::*;

    #[test]
    fn test_display_empty() {
        let col = OptSortOrder::Empty;
        assert_eq!(format!("{}", col), "empty");
    }
    #[test]
    fn test_display_swap() {
        let col = OptSortOrder::Swap;
        assert_eq!(format!("{}", col), "swap");
    }
    #[test]
    fn test_display_rss() {
        let col = OptSortOrder::Rss;
        assert_eq!(format!("{}", col), "rss");
    }
    #[test]
    fn test_display_total() {
        let col = OptSortOrder::Total;
        assert_eq!(format!("{}", col), "total");
    }
    #[test]
    fn test_from_str_empty() {
        let col: OptSortOrder = match FromStr::from_str("empty") {
            Ok(c) => c,
            Err(_) => {
                unreachable!();
            }
        };
        assert_eq!(col, OptSortOrder::Empty);
    }
    #[test]
    fn test_from_str_swap() {
        let col: OptSortOrder = match FromStr::from_str("swap") {
            Ok(c) => c,
            Err(_) => {
                unreachable!();
            }
        };
        assert_eq!(col, OptSortOrder::Swap);
    }
    #[test]
    fn test_from_str_rss() {
        let col: OptSortOrder = match FromStr::from_str("rss") {
            Ok(c) => c,
            Err(_) => {
                unreachable!();
            }
        };
        assert_eq!(col, OptSortOrder::Rss);
    }
    #[test]
    fn test_from_str_total() {
        let col: OptSortOrder = match FromStr::from_str("total") {
            Ok(c) => c,
            Err(_) => {
                unreachable!();
            }
        };
        assert_eq!(col, OptSortOrder::Total);
    }
    #[test]
    fn test_from_str_invalid() {
        let _col: OptSortOrder = match FromStr::from_str("other") {
            Ok(_c) => _c,
            Err(e) => {
                assert_eq!(e.to_string(), "can not parse \'other\'");
                return;
            }
        };
        unreachable!();
    }
}
*/
