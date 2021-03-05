use super::{KeyColumns, SortLinesBuffer};
use anyhow::Context;
use semver::{SemVerError, Version};
use std::cmp::Ordering;
use std::collections::BinaryHeap;

pub struct SortLinesBufferVersion {
    buf_lines: BinaryHeap<SortLine>,
}
impl SortLinesBufferVersion {
    pub fn new() -> Self {
        Self {
            buf_lines: BinaryHeap::new(),
        }
    }
}
impl SortLinesBuffer for SortLinesBufferVersion {
    fn push_line(&mut self, reverse: bool, key: KeyColumns, line: String) -> anyhow::Result<()> {
        let sort_line = SortLine::new(reverse, key, line)?;
        self.buf_lines.push(sort_line);
        Ok(())
    }
    fn into_sorted_vec(self) -> Vec<String> {
        let sorted_vec: Vec<SortLine> = self.buf_lines.into_sorted_vec();
        let mut ret_vec = Vec::with_capacity(sorted_vec.len());
        for sort_line in sorted_vec.into_iter() {
            ret_vec.push(sort_line.line);
        }
        ret_vec
    }
}

struct SortLine {
    reverse: bool,
    key: Version,
    line: String,
}

impl SortLine {
    fn new(a_reverse: bool, a_key: KeyColumns, a_line: String) -> anyhow::Result<Self> {
        let key_ver = make_version(&a_line[a_key.0..a_key.1])
            .with_context(|| format!("({},{}):'{}'", a_key.0, a_key.1, a_line))?;
        Ok(Self {
            reverse: a_reverse,
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
        if !self.reverse {
            Some(self.key.cmp(&other.key))
        } else {
            Some(other.key.cmp(&self.key))
        }
    }
}

impl Ord for SortLine {
    #[inline]
    fn cmp(&self, other: &SortLine) -> Ordering {
        if !self.reverse {
            self.key.cmp(&other.key)
        } else {
            other.key.cmp(&self.key)
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
        assert_eq!(std::mem::size_of::<SortLinesBufferVersion>(), 24);
        assert_eq!(std::mem::size_of::<SortLine>(), 104);
    }
}
