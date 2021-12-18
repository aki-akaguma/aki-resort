use crate::conf::{CmdOptConf, EnvConf};
use crate::sort::{KeyColumns, KeyLine};
use crate::sort::{
    SortLinesBufferMonth, SortLinesBufferNumeric, SortLinesBufferString, SortLinesBufferTime,
    SortLinesBufferVersion,
};
use crate::util::err::BrokenPipeError;
use crate::util::OptAccordingToWord;
use crate::util::OptColorWhen;
use regex::Regex;
use runnel::RunnelIoe;
use std::io::{BufRead, Write};

pub fn run(sioe: &RunnelIoe, conf: &CmdOptConf, env: &EnvConf) -> anyhow::Result<()> {
    let re = if !conf.opt_exp.is_empty() {
        let re = Regex::new(conf.opt_exp.as_str())?;
        Some(re)
    } else {
        None
    };
    let r = run_0(sioe, conf, env, re);
    if r.is_broken_pipe() {
        return Ok(());
    }
    r
}

fn lines_loop<T>(
    sioe: &RunnelIoe,
    conf: &CmdOptConf,
    re: Option<Regex>,
    mut sort_buf_lines: T,
) -> anyhow::Result<Vec<KeyLine>>
where
    T: crate::sort::SortLinesBuffer,
{
    let mut curr_sz: usize = 0;
    let mut result_buf_lines = Vec::new();
    let mut buf_lines = Vec::new();
    //
    // read all lines
    for line in sioe.pin().lock().lines() {
        let mut line_s = line?;
        line_s.shrink_to_fit();
        let line_ss = line_s.as_str();
        let line_len: usize = line_ss.len();
        //
        curr_sz += line_len;
        if !conf.opt_max_buffer.is_ok(curr_sz) {
            return Err(anyhow!("over max buffer size: {}", conf.opt_max_buffer));
        }
        //
        if let Some(n) = conf.opt_head {
            if result_buf_lines.len() < n {
                let key = KeyColumns::new(0, 0);
                result_buf_lines.push(KeyLine::new(key, line_s));
                continue;
            }
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
        buf_lines.push(KeyLine::new(key, line_s));
    }
    // remove footer
    let mut footer = if let Some(n) = conf.opt_tail {
        if n > 0 {
            let at = buf_lines.len() - n;
            let mut buf = buf_lines.split_off(at);
            for v in buf.iter_mut() {
                v.key = KeyColumns::new(0, 0);
            }
            buf
        } else {
            Vec::new()
        }
    } else {
        Vec::new()
    };
    // sort body
    for key_line in buf_lines {
        sort_buf_lines.push_line(key_line.key, key_line.line)?;
    }
    // append all lines
    let mut body = sort_buf_lines.into_sorted_vec();
    result_buf_lines.append(&mut body);
    result_buf_lines.append(&mut footer);
    //
    Ok(result_buf_lines)
}

fn run_0(
    sioe: &RunnelIoe,
    conf: &CmdOptConf,
    env: &EnvConf,
    re: Option<Regex>,
) -> anyhow::Result<()> {
    let color_start_s = env.color_seq_start.as_str();
    let color_end_s = env.color_seq_end.as_str();
    let color_is_alyways = crate::my_matches!(conf.opt_color, OptColorWhen::Always);
    let flg_r = conf.flg_reverse;
    let v = match conf.opt_according_to {
        OptAccordingToWord::Numeric => {
            lines_loop(sioe, conf, re, SortLinesBufferNumeric::new(flg_r))?
        }
        OptAccordingToWord::Month => lines_loop(sioe, conf, re, SortLinesBufferMonth::new(flg_r))?,
        OptAccordingToWord::String => {
            lines_loop(sioe, conf, re, SortLinesBufferString::new(flg_r))?
        }
        OptAccordingToWord::Time => lines_loop(sioe, conf, re, SortLinesBufferTime::new(flg_r))?,
        OptAccordingToWord::Version => {
            lines_loop(sioe, conf, re, SortLinesBufferVersion::new(flg_r))?
        }
    };
    //
    #[allow(clippy::collapsible_if)]
    if !color_is_alyways {
        if !conf.flg_unique {
            for key_line in v {
                #[rustfmt::skip]
                sioe.pout().lock().write_fmt(format_args!("{}\n", key_line.line))?;
            }
        } else {
            let mut pre_line = String::new();
            for key_line in v {
                if pre_line != key_line.line {
                    #[rustfmt::skip]
                    sioe.pout().lock().write_fmt(format_args!("{}\n", key_line.line))?;
                    pre_line = key_line.line;
                }
            }
        }
    } else if !conf.flg_unique {
        for key_line in v {
            let mut out_s: String = String::new();
            out_s.push_str(&key_line.line[0..key_line.key.st]);
            if key_line.key.st < key_line.key.ed {
                out_s.push_str(color_start_s);
                out_s.push_str(&key_line.line[key_line.key.st..key_line.key.ed]);
                out_s.push_str(color_end_s);
            }
            out_s.push_str(&key_line.line[key_line.key.ed..]);
            #[rustfmt::skip]
            sioe.pout().lock().write_fmt(format_args!("{}\n", out_s))?;
        }
    } else {
        let mut pre_line = String::new();
        for key_line in v {
            if pre_line != key_line.line {
                let mut out_s: String = String::new();
                out_s.push_str(&key_line.line[0..key_line.key.st]);
                if key_line.key.st < key_line.key.ed {
                    out_s.push_str(color_start_s);
                    out_s.push_str(&key_line.line[key_line.key.st..key_line.key.ed]);
                    out_s.push_str(color_end_s);
                }
                out_s.push_str(&key_line.line[key_line.key.ed..]);
                #[rustfmt::skip]
                sioe.pout().lock().write_fmt(format_args!("{}\n", out_s))?;
                pre_line = key_line.line;
            }
        }
    }
    //
    sioe.pout().lock().flush()?;
    //
    Ok(())
}
