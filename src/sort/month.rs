use super::{KeyColumns, KeyLine, SortLinesBuffer};
use std::cmp::Ordering;

pub struct SortLinesBufferMonth {
    buf_lines: Vec<SortLine>,
    reverse: bool,
}
impl SortLinesBufferMonth {
    pub fn new(a_reverse: bool) -> Self {
        Self {
            buf_lines: Vec::new(),
            reverse: a_reverse,
        }
    }
}
impl SortLinesBuffer for SortLinesBufferMonth {
    fn push_line(&mut self, key: KeyColumns, line: String) -> anyhow::Result<()> {
        let sort_line = SortLine::new(self.buf_lines.len(), key, line)?;
        self.buf_lines.push(sort_line);
        Ok(())
    }
    fn into_sorted_vec(mut self) -> Vec<KeyLine> {
        use rayon::slice::ParallelSliceMut;
        if !self.reverse {
            self.buf_lines.par_sort_unstable_by(|a, b| a.cmp(&b));
        } else {
            self.buf_lines.par_sort_unstable_by(|a, b| b.cmp(&a));
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
    key: i64,
    key_line: KeyLine,
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
    fn new(a_num: usize, a_key: KeyColumns, a_line: String) -> anyhow::Result<Self> {
        let key = a_line[a_key.st..a_key.ed].to_ascii_lowercase();
        let idx = match DICT_FULL
            .iter()
            .position(|item| item.0 == key || item.1 == key || item.2 == key)
        {
            Some(idx) => idx,
            None => {
                return Err(anyhow!(
                    "({},{}):'{}': {}",
                    a_key.st,
                    a_key.ed,
                    a_line,
                    "invalid month strings"
                ));
            }
        };
        Ok(Self {
            num: a_num,
            key: idx as i64,
            key_line: KeyLine::new(a_key, a_line),
        })
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
        assert_eq!(std::mem::size_of::<SortLinesBufferMonth>(), 32);
        assert_eq!(std::mem::size_of::<SortLine>(), 56);
    }
}
