use super::{KeyColumns, KeyLine, SortLinesBuffer};
use anyhow::Context;
use std::cmp::Ordering;
use std::time::Duration;

#[derive(Debug)]
pub struct SortLinesBufferTime {
    buf_lines: Vec<SortLine>,
    reverse: bool,
}
impl SortLinesBufferTime {
    pub fn new(a_reverse: bool) -> Self {
        Self {
            buf_lines: Vec::new(),
            reverse: a_reverse,
        }
    }
}
impl SortLinesBuffer for SortLinesBufferTime {
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
    key: Duration,
    key_line: KeyLine,
}

impl SortLine {
    fn new(a_num: usize, a_key: KeyColumns, a_line: String) -> anyhow::Result<Self> {
        let key_ver = make_time(&a_line[a_key.st..a_key.ed])
            .with_context(|| format!("({},{}):'{}'", a_key.st, a_key.ed, a_line))?;
        Ok(Self {
            num: a_num,
            key: key_ver,
            key_line: KeyLine::new(a_key, a_line),
        })
    }
}

fn make_time(s: &str) -> anyhow::Result<Duration> {
    for c in s.chars() {
        match c {
            '0'..='9' | ':' | '.' => (),
            _ => {
                return Err(anyhow!("unexpected character '{}' while parsing time", c));
            }
        }
    }
    //
    let key_s = s;
    let (millis, idx) = if key_s.is_empty() {
        (0, 0)
    } else {
        match key_s.rfind('.') {
            Some(idx) => {
                let num = &key_s[(idx + 1)..]
                    .parse::<u64>()
                    .with_context(|| format!("can not parse millis: '{}'", &key_s[idx..]))?;
                (*num, idx)
            }
            None => (0, key_s.len()),
        }
    };
    let key_s = &key_s[..idx];
    let (seconds, idx) = if key_s.is_empty() {
        (0, 0)
    } else {
        match key_s.rfind(':') {
            Some(idx) => {
                let num = &key_s[(idx + 1)..].parse::<u64>().with_context(|| {
                    format!(
                        "can not parse seconds: '{}', already: {millis}ms",
                        &key_s[idx..]
                    )
                })?;
                (*num, idx)
            }
            None => (0, key_s.len()),
        }
    };
    let key_s = &key_s[..idx];
    let (minutes, idx) = if key_s.is_empty() {
        (0, 0)
    } else {
        let (kk, ii) = match key_s.rfind(':') {
            Some(idx) => (&key_s[(idx + 1)..], idx),
            None => (key_s, 0),
        };
        let num = kk.parse::<u64>().with_context(|| {
            format!("can not parse minutes: '{kk}', already: {seconds}.{millis}")
        })?;
        (num, ii)
    };
    let key_s = &key_s[..idx];
    let (hours, _idx) = if key_s.is_empty() {
        (0, 0)
    } else {
        let num = key_s.parse::<u64>().with_context(|| {
            format!("can not parse hours: '{key_s}', already: {minutes}:{seconds}.{millis}")
        })?;
        (num, key_s.len())
    };
    //
    //eprintln!("AAA: {hours}:{minutes}:{seconds}.{millis}");
    let dur_sec = Duration::from_secs(hours * 60 * 60 + minutes * 60 + seconds);
    let dur_milli = Duration::from_millis(millis);
    Ok(dur_sec + dur_milli)
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
        assert_eq!(std::mem::size_of::<SortLinesBufferTime>(), 32);
        assert_eq!(std::mem::size_of::<SortLine>(), 64);
    }
    #[cfg(target_pointer_width = "32")]
    #[test]
    fn size_of() {
        assert_eq!(std::mem::size_of::<SortLinesBufferTime>(), 16);
        #[cfg(not(any(target_arch = "arm", target_arch = "mips")))]
        assert_eq!(std::mem::size_of::<SortLine>(), 36);
        #[cfg(any(target_arch = "arm", target_arch = "mips"))]
        assert_eq!(std::mem::size_of::<SortLine>(), 40);
    }
}
