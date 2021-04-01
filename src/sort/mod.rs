pub mod month;
pub mod numeric;
pub mod string;
pub mod version;

pub use month::SortLinesBufferMonth;
pub use numeric::SortLinesBufferNumeric;
pub use string::SortLinesBufferString;
pub use version::SortLinesBufferVersion;

#[derive(Debug)]
pub(crate) struct KeyColumns {
    pub st: usize,
    pub ed: usize,
}
impl KeyColumns {
    pub fn new(a_st: usize, a_ed: usize) -> Self {
        Self { st: a_st, ed: a_ed }
    }
}

#[derive(Debug)]
pub(crate) struct KeyLine {
    pub key: KeyColumns,
    pub line: String,
}
impl KeyLine {
    pub fn new(a_key: KeyColumns, a_line: String) -> Self {
        Self {
            key: a_key,
            line: a_line,
        }
    }
}

pub(crate) trait SortLinesBuffer {
    fn push_line(&mut self, key: KeyColumns, line: String) -> anyhow::Result<()>;
    fn into_sorted_vec(self) -> Vec<KeyLine>;
}

#[cfg(test)]
mod debug {
    use super::*;
    #[test]
    fn size_of() {
        assert_eq!(std::mem::size_of::<KeyColumns>(), 16);
        assert_eq!(std::mem::size_of::<KeyLine>(), 40);
    }
}
