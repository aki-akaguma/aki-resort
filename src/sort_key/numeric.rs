use super::{KeyColumns, SortLinesBuffer};
use anyhow::Context;
use std::cmp::Ordering;
use std::collections::BinaryHeap;

pub struct SortLinesBufferNumeric {
    buf_lines: BinaryHeap<SortLine>,
}
impl SortLinesBufferNumeric {
    pub fn new() -> Self {
        Self {
            buf_lines: BinaryHeap::new(),
        }
    }
}
impl SortLinesBuffer for SortLinesBufferNumeric {
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
    key: i64,
    line: String,
}

impl SortLine {
    fn new(a_reverse: bool, a_key: KeyColumns, a_line: String) -> anyhow::Result<Self> {
        let key_num = a_line[a_key.0..a_key.1]
            .parse::<i64>()
            .with_context(|| format!("({},{}):'{}'", a_key.0, a_key.1, a_line))?;
        Ok(Self {
            reverse: a_reverse,
            key: key_num,
            line: a_line,
        })
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
        assert_eq!(std::mem::size_of::<SortLinesBufferNumeric>(), 24);
        assert_eq!(std::mem::size_of::<SortLine>(), 40);
    }
}
