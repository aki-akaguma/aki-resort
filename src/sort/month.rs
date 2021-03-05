use super::{KeyColumns, SortLinesBuffer};
use std::cmp::Ordering;
use std::collections::BinaryHeap;

pub struct SortLinesBufferMonth {
    buf_lines: BinaryHeap<SortLine>,
}
impl SortLinesBufferMonth {
    pub fn new() -> Self {
        Self {
            buf_lines: BinaryHeap::new(),
        }
    }
}
impl SortLinesBuffer for SortLinesBufferMonth {
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

const DICT_FULL: [(&str, &str, &str); 12] = [
    ("jan", "jan", "january"),
    ("feb", "feb", "february"),
    ("mar", "mar", "march"),
    ("apr", "apr", "april"),
    ("may", "may", "may"),
    ("jun", "june", "june"),
    ("jul", "july", "july"),
    ("aug", "aug", "august"),
    ("sep", "sept", "september"),
    ("oct", "oct", "october"),
    ("nov", "nov", "november"),
    ("dec", "dec", "december"),
];

impl SortLine {
    fn new(a_reverse: bool, a_key: KeyColumns, a_line: String) -> anyhow::Result<Self> {
        let key = a_line[a_key.0..a_key.1].to_ascii_lowercase();
        let idx = match DICT_FULL
            .iter()
            .position(|item| item.0 == key || item.1 == key || item.2 == key)
        {
            Some(idx) => idx,
            None => {
                return Err(anyhow!(
                    "({},{}):'{}': {}",
                    a_key.0,
                    a_key.1,
                    a_line,
                    "invalid month strings"
                ));
            }
        };
        Ok(Self {
            reverse: a_reverse,
            key: idx as i64,
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
        assert_eq!(std::mem::size_of::<SortLinesBufferMonth>(), 24);
        assert_eq!(std::mem::size_of::<SortLine>(), 40);
    }
}
