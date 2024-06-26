use super::{KeyColumns, KeyLine, SortLinesBuffer};
use anyhow::Context;
use semver::{Error, Version};
use std::cmp::Ordering;

#[derive(Debug)]
pub struct SortLinesBufferVersion {
    buf_lines: Vec<SortLine>,
    reverse: bool,
}
impl SortLinesBufferVersion {
    pub fn new(a_reverse: bool) -> Self {
        Self {
            buf_lines: Vec::new(),
            reverse: a_reverse,
        }
    }
}
impl SortLinesBuffer for SortLinesBufferVersion {
    fn push_line(&mut self, key: KeyColumns, line: String) -> anyhow::Result<()> {
        let sort_line = SortLine::new(self.buf_lines.len(), key, line)?;
        self.buf_lines.push(sort_line);
        Ok(())
    }
    fn into_sorted_vec(mut self) -> Vec<KeyLine> {
        use rayon::slice::ParallelSliceMut;
        if !self.reverse {
            self.buf_lines.par_sort_unstable_by(|a, b| a.cmp(b));
        } else {
            self.buf_lines.par_sort_unstable_by(|a, b| b.cmp(a));
        }
        let mut ret_vec = Vec::with_capacity(self.buf_lines.len());
        for sort_line in self.buf_lines.into_iter() {
            ret_vec.push(sort_line.key_line);
        }
        ret_vec
    }
}

#[derive(Debug)]
struct SortLine {
    num: usize,
    key: Version,
    key_line: KeyLine,
}

impl SortLine {
    fn new(a_num: usize, a_key: KeyColumns, a_line: String) -> anyhow::Result<Self> {
        let key_ver = make_version(&a_line[a_key.st..a_key.ed])
            .with_context(|| format!("({},{}):'{}'", a_key.st, a_key.ed, a_line))?;
        Ok(Self {
            num: a_num,
            key: key_ver,
            key_line: KeyLine::new(a_key, a_line),
        })
    }
}

fn make_version(s: &str) -> Result<Version, Error> {
    match Version::parse(s) {
        Ok(ver) => Ok(ver),
        Err(err) => match err.to_string().as_str() {
            "unexpected end of input while parsing major version number" => {
                make_version((s.to_string() + ".0.0").as_str())
            }
            "unexpected end of input while parsing minor version number" => {
                make_version((s.to_string() + ".0").as_str())
            }
            _ => Err(err),
        },
    }
}

impl PartialOrd for SortLine {
    #[inline]
    fn partial_cmp(&self, other: &SortLine) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for SortLine {
    #[inline]
    fn cmp(&self, other: &SortLine) -> Ordering {
        let r = self.key.cmp(&other.key);
        match r {
            Ordering::Equal => self.num.cmp(&other.num),
            _ => r,
        }
    }
}

impl PartialEq for SortLine {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        self.key == other.key
    }
}

impl Eq for SortLine {}

#[cfg(test)]
mod debug {
    use super::*;
    #[cfg(target_pointer_width = "64")]
    #[test]
    fn size_of() {
        assert_eq!(std::mem::size_of::<SortLinesBufferVersion>(), 32);
        assert_eq!(std::mem::size_of::<SortLine>(), 88);
    }
    #[cfg(target_pointer_width = "32")]
    #[test]
    fn size_of() {
        assert_eq!(std::mem::size_of::<SortLinesBufferVersion>(), 16);
        assert_eq!(std::mem::size_of::<SortLine>(), 64);
    }
}
