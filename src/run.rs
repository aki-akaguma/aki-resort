use crate::conf::CmdOptConf;
use crate::sort::KeyColumns;
use crate::sort::{
    SortLinesBufferMonth, SortLinesBufferNumeric, SortLinesBufferString, SortLinesBufferVersion,
};
use crate::util::err::BrokenPipeError;
use crate::util::OptAccordingToWord;
use regex::Regex;
use runnel::RunnelIoe;
use std::io::{BufRead, Write};

pub fn run(sioe: &RunnelIoe, conf: &CmdOptConf) -> anyhow::Result<()> {
    let re = if !conf.opt_exp.is_empty() {
        let re = Regex::new(conf.opt_exp.as_str())?;
        Some(re)
    } else {
        None
    };
    let r = run_0(sioe, conf, re);
    if r.is_broken_pipe() {
        return Ok(());
    }
    r
}

fn lines_loop<T>(
    sioe: &RunnelIoe,
    conf: &CmdOptConf,
    re: Option<Regex>,
    mut buf_lines: T,
) -> anyhow::Result<Vec<String>>
where
    T: crate::sort::SortLinesBuffer,
{
    let mut curr_sz: usize = 0;
    //
    // read all lines
    for line in sioe.pin().lock().lines() {
        let line_s = line?;
        let line_ss = line_s.as_str();
        let line_len: usize = line_ss.len();
        //
        curr_sz += line_len;
        if !conf.opt_max_buffer.is_ok(curr_sz) {
            return Err(anyhow!("over max buffer size: {}", conf.opt_max_buffer));
        }
        //
        let key = if let Some(ref re) = re {
            if let Some(caps) = re.captures(line_ss) {
                if let Some(mat) = caps.get(1) {
                    KeyColumns::new(mat.start(), mat.end())
                } else if let Some(mat) = caps.get(0) {
                    KeyColumns::new(mat.start(), mat.end())
                } else {
                    unreachable!();
                }
            } else {
                KeyColumns::new(0, line_len)
            }
        } else {
            KeyColumns::new(0, line_len)
        };
        //
        buf_lines.push_line(conf.flg_reverse, key, line_s)?;
    }
    // all lines
    Ok(buf_lines.into_sorted_vec())
}

fn run_0(sioe: &RunnelIoe, conf: &CmdOptConf, re: Option<Regex>) -> anyhow::Result<()> {
    let v = match conf.opt_according_to {
        OptAccordingToWord::String => lines_loop(sioe, conf, re, SortLinesBufferString::new())?,
        OptAccordingToWord::Numeric => lines_loop(sioe, conf, re, SortLinesBufferNumeric::new())?,
        OptAccordingToWord::Version => lines_loop(sioe, conf, re, SortLinesBufferVersion::new())?,
        OptAccordingToWord::Month => lines_loop(sioe, conf, re, SortLinesBufferMonth::new())?,
    };
    if !conf.flg_unique {
        for line in v {
            #[rustfmt::skip]
            sioe.pout().lock().write_fmt(format_args!("{}\n", line))?;
        }
    } else {
        let mut pre_line = String::new();
        for line in v {
            if pre_line != line {
                #[rustfmt::skip]
                sioe.pout().lock().write_fmt(format_args!("{}\n", line))?;
                pre_line = line;
            }
        }
    }
    //
    sioe.pout().lock().flush()?;
    //
    Ok(())
}
