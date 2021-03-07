use super::{KeyColumns, SortLinesBuffer};
use anyhow::Context;
use semver::{SemVerError, Version};
use std::cmp::Ordering;

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
    fn into_sorted_vec(mut self) -> Vec<String> {
        use rayon::slice::ParallelSliceMut;
        if !self.reverse {
            self.buf_lines.par_sort_unstable_by(|a, b| a.cmp(&b));
        } else {
            self.buf_lines.par_sort_unstable_by(|a, b| b.cmp(&a));
        }
        let mut ret_vec = Vec::with_capacity(self.buf_lines.len());
        for sort_line in self.buf_lines.into_iter() {
            ret_vec.push(sort_line.line);
        }
        ret_vec
    }
}

struct SortLine {
    num: usize,
    key: Version,
    line: String,
}

impl SortLine {
    fn new(a_num: usize, a_key: KeyColumns, a_line: String) -> anyhow::Result<Self> {
        let key_ver = make_version(&a_line[a_key.0..a_key.1])
            .with_context(|| format!("({},{}):'{}'", a_key.0, a_key.1, a_line))?;
        Ok(Self {
            num: a_num,
            key: key_ver,
            line: a_line,
        })
    }
}

fn make_version(s: &str) -> Result<Version, SemVerError> {
    match Version::parse(s) {
        Ok(ver) => Ok(ver),
        Err(SemVerError::ParseError(ref msg)) if msg.as_str() == "expected more input" => {
            make_version((s.to_string() + ".0").as_str())
        }
        Err(err) => Err(err),
    }
}

impl PartialOrd for SortLine {
    #[inline]
    fn partial_cmp(&self, other: &SortLine) -> Option<Ordering> {
        let r = self.key.cmp(&other.key);
        let r = match r {
            Ordering::Equal => self.num.cmp(&other.num),
            _ => r,
        };
        Some(r)
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
    #[test]
    fn size_of() {
        assert_eq!(std::mem::size_of::<SortLinesBufferVersion>(), 32);
        assert_eq!(std::mem::size_of::<SortLine>(), 104);
    }
}
