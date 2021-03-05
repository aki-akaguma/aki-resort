use super::{KeyColumns, SortLinesBuffer};
use std::cmp::Ordering;
use std::collections::BinaryHeap;

pub struct SortLinesBufferString {
    buf_lines: BinaryHeap<SortLine>,
}
impl SortLinesBufferString {
    pub fn new() -> Self {
        Self {
            buf_lines: BinaryHeap::new(),
        }
    }
}
impl SortLinesBuffer for SortLinesBufferString {
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
    key: KeyColumns,
    line: String,
}

impl SortLine {
    #[allow(clippy::unnecessary_wraps)]
    fn new(a_reverse: bool, a_key: KeyColumns, a_line: String) -> anyhow::Result<Self> {
        Ok(Self {
            reverse: a_reverse,
            key: a_key,
            line: a_line,
        })
    }
    #[inline]
    fn key_str(&self) -> &str {
        &self.line[self.key.0..self.key.1]
    }
}

impl PartialOrd for SortLine {
    #[inline]
    fn partial_cmp(&self, other: &SortLine) -> Option<Ordering> {
        let one = self.key_str();
        let two = other.key_str();
        if !self.reverse {
            Some(one.cmp(&two))
        } else {
            Some(two.cmp(&one))
        }
    }
}

impl Ord for SortLine {
    #[inline]
    fn cmp(&self, other: &SortLine) -> Ordering {
        let one = self.key_str();
        let two = other.key_str();
        if !self.reverse {
            one.cmp(&two)
        } else {
            two.cmp(&one)
        }
    }
}

impl PartialEq for SortLine {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        let one = self.key_str();
        let two = other.key_str();
        one == two
    }
}

impl Eq for SortLine {}

#[cfg(test)]
mod debug {
    use super::*;
    #[test]
    fn size_of() {
        assert_eq!(std::mem::size_of::<String>(), 24);
        assert_eq!(std::mem::size_of::<SortLinesBufferString>(), 24);
        assert_eq!(std::mem::size_of::<SortLine>(), 48);
    }
}
