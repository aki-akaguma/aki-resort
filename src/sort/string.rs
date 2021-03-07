use super::{KeyColumns, SortLinesBuffer};
use std::cmp::Ordering;

pub struct SortLinesBufferString {
    buf_lines: Vec<SortLine>,
    reverse: bool,
}
impl SortLinesBufferString {
    pub fn new(a_reverse: bool) -> Self {
        Self {
            buf_lines: Vec::new(),
            reverse: a_reverse,
        }
    }
}
impl SortLinesBuffer for SortLinesBufferString {
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
    key: KeyColumns,
    line: String,
}

impl SortLine {
    #[allow(clippy::unnecessary_wraps)]
    fn new(a_num: usize, a_key: KeyColumns, a_line: String) -> anyhow::Result<Self> {
        Ok(Self {
            num: a_num,
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
        let r = one.cmp(&two);
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
        let one = self.key_str();
        let two = other.key_str();
        let r = one.cmp(&two);
        match r {
            Ordering::Equal => self.num.cmp(&other.num),
            _ => r,
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
        assert_eq!(std::mem::size_of::<SortLinesBufferString>(), 32);
        assert_eq!(std::mem::size_of::<SortLine>(), 48);
    }
}
