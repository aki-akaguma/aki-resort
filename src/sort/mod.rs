pub mod month;
pub mod numeric;
pub mod string;
pub mod version;

pub use month::SortLinesBufferMonth;
pub use numeric::SortLinesBufferNumeric;
pub use string::SortLinesBufferString;
pub use version::SortLinesBufferVersion;

#[derive(Debug)]
pub struct KeyColumns(usize, usize);
impl KeyColumns {
    pub fn new(a_st: usize, a_ed: usize) -> Self {
        Self(a_st, a_ed)
    }
}

pub trait SortLinesBuffer {
    fn push_line(&mut self, reverse: bool, key: KeyColumns, line: String) -> anyhow::Result<()>;
    fn into_sorted_vec(self) -> Vec<String>;
}

#[cfg(test)]
mod debug {
    use super::*;
    #[test]
    fn size_of() {
        assert_eq!(std::mem::size_of::<KeyColumns>(), 16);
    }
}
