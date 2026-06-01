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
                return Err(anyhow!("unexpected character '{c}' while parsing time"));
            }
        }
    }
    //
    let key_s = s;
    let (nanos, key_s) = match key_s.rfind('.') {
        Some(idx) => {
            let frac_str = &key_s[(idx + 1)..];
            let mut num = frac_str
                .parse::<u64>()
                .with_context(|| format!("can not parse fractional: '{frac_str}'"))?;
            // Adjust to nanoseconds
            let mut len = frac_str.len();
            while len < 9 {
                num *= 10;
                len += 1;
            }
            while len > 9 {
                num /= 10;
                len -= 1;
            }
            (num, &key_s[..idx])
        }
        None => (0, key_s),
    };
    //
    let parts: Vec<&str> = key_s.split(':').collect();
    let (hours, minutes, seconds) = match parts.len() {
        1 => (
            0,
            0,
            parts[0]
                .parse::<u64>()
                .with_context(|| format!("can not parse seconds: '{}'", parts[0]))?,
        ),
        2 => (
            0,
            parts[0]
                .parse::<u64>()
                .with_context(|| format!("can not parse minutes: '{}'", parts[0]))?,
            parts[1]
                .parse::<u64>()
                .with_context(|| format!("can not parse seconds: '{}'", parts[1]))?,
        ),
        3 => (
            parts[0]
                .parse::<u64>()
                .with_context(|| format!("can not parse hours: '{}'", parts[0]))?,
            parts[1]
                .parse::<u64>()
                .with_context(|| format!("can not parse minutes: '{}'", parts[1]))?,
            parts[2]
                .parse::<u64>()
                .with_context(|| format!("can not parse seconds: '{}'", parts[2]))?,
        ),
        _ => return Err(anyhow!("unexpected time format: '{s}'")),
    };
    //
    let dur_sec = Duration::from_secs(hours * 60 * 60 + minutes * 60 + seconds);
    let dur_nano = Duration::from_nanos(nanos);
    Ok(dur_sec + dur_nano)
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

    #[test]
    fn test_make_time_frac() {
        assert_eq!(make_time("1.5").unwrap(), Duration::from_millis(1500));
        assert_eq!(make_time("1.05").unwrap(), Duration::from_millis(1050));
        assert_eq!(make_time("1.005").unwrap(), Duration::from_millis(1005));

        // New tests for colon-based parsing
        assert_eq!(make_time("20.5").unwrap(), Duration::from_millis(20500));
        assert_eq!(
            make_time("1:20.5").unwrap(),
            Duration::from_secs(80) + Duration::from_millis(500)
        );
        assert_eq!(
            make_time("1:1:20.5").unwrap(),
            Duration::from_secs(3680) + Duration::from_millis(500)
        );
    }
}
