//! exec_target is command test helper.
//!
//! ```
//! use exec_target::exec_target_with_env_in;
//!
//! let command = "target/debug/exe-stab-grep";
//! let args = &["--color=always", "-e", "c"];
//! let envs = vec![("GREP_COLORS", "ms=01;32")];
//! let inp = b"abcdefg\n" as &[u8];
//!
//! let oup = exec_target_with_env_in(command, args, envs, inp);
//!
//! assert_eq!(oup.stderr, "");
//! assert_eq!(oup.stdout, "ab\u{1b}[01;32m\u{1b}[Kc\u{1b}[m\u{1b}[Kdefg\n");
//! assert_eq!(oup.status.success(), true);
//! ```
//!
//!
//!

use std::collections::HashMap;
use std::env;
use std::ffi::OsStr;
use std::process::{Command, ExitStatus, Output, Stdio};

// trait
use std::io::Write;
use std::iter::IntoIterator;

//
pub struct OutputString {
    pub status: ExitStatus,
    pub stdout: String,
    pub stderr: String,
}

fn setup_envs<I, K, V>(cmd: &mut Command, vars: I) -> &mut Command
where
    I: IntoIterator<Item = (K, V)>,
    K: AsRef<OsStr>,
    V: AsRef<OsStr>,
{
    let filtered_env: HashMap<String, String> = env::vars()
        .filter(|&(ref k, _)| k == "TERM" || k == "TZ" || k == "PATH")
        .collect();
    cmd.env_clear()
        .envs(filtered_env)
        .envs(vars)
        .env("LANG", "C")
}

pub fn exec_target<I, S>(target_exe: &str, args: I) -> OutputString
where
    I: IntoIterator<Item = S>,
    S: AsRef<OsStr>,
{
    let mut cmd: Command = Command::new(target_exe);
    setup_envs(&mut cmd, Vec::<(&str, &str)>::new())
        .args(args)
        .stdout(Stdio::piped())
        .stderr(Stdio::piped());
    let child = cmd.spawn().expect("failed to execute child");
    let output: Output = child.wait_with_output().expect("failed to wait on child");
    //
    OutputString {
        status: output.status,
        stdout: String::from(String::from_utf8_lossy(&output.stdout)),
        stderr: String::from(String::from_utf8_lossy(&output.stderr)),
    }
}

pub fn exec_target_with_in<I, S>(target_exe: &str, args: I, in_bytes: &[u8]) -> OutputString
where
    I: IntoIterator<Item = S>,
    S: AsRef<OsStr>,
{
    let mut cmd: Command = Command::new(target_exe);
    setup_envs(&mut cmd, Vec::<(&str, &str)>::new())
        .args(args)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped());
    let mut child = cmd.spawn().expect("failed to execute child");
    {
        let stdin = child.stdin.as_mut().expect("failed to get stdin");
        stdin.write_all(in_bytes).expect("failed to write to stdin");
    }
    let output: Output = child.wait_with_output().expect("failed to wait on child");
    //
    OutputString {
        status: output.status,
        stdout: String::from(String::from_utf8_lossy(&output.stdout)),
        stderr: String::from(String::from_utf8_lossy(&output.stderr)),
    }
}

pub fn exec_target_with_env_in<I, S, IKV, K, V>(
    target_exe: &str,
    args: I,
    env: IKV,
    in_bytes: &[u8],
) -> OutputString
where
    I: IntoIterator<Item = S>,
    S: AsRef<OsStr>,
    IKV: IntoIterator<Item = (K, V)>,
    K: AsRef<OsStr>,
    V: AsRef<OsStr>,
{
    let mut cmd: Command = Command::new(target_exe);
    setup_envs(&mut cmd, env)
        .args(args)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped());
    let mut child = cmd.spawn().expect("failed to execute child");
    {
        let stdin = child.stdin.as_mut().expect("failed to get stdin");
        stdin.write_all(in_bytes).expect("failed to write to stdin");
    }
    let output: Output = child.wait_with_output().expect("failed to wait on child");
    //
    OutputString {
        status: output.status,
        stdout: String::from(String::from_utf8_lossy(&output.stdout)),
        stderr: String::from(String::from_utf8_lossy(&output.stderr)),
    }
}
