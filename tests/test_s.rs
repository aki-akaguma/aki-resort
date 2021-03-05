macro_rules! help_msg {
    () => {
        concat!(
            version_msg!(),
            "\n",
            "Usage:\n",
            "  aki-resort [options]\n",
            "\n",
            "sort lines of text.\n",
            "\n",
            "Ordering options:\n",
            "  -r, --reverse                 reverse the result of comparisons\n",
            "      --according-to <word>     sort according to WORD: string, numeric, month, version\n",
            "\n",
            "Other options:\n",
            "  -e, --exp <exp>               regular expression. sort via this match point.\n",
            "  -u, --unique                  output only the first line of an equal.\n",
            "      --max-buffer <size>       max buffer size. if reading size is more than <size>, then it not output, quit and display error message.\n",
            "\n",
            "  -H, --help        display this help and exit\n",
            "  -V, --version     display version information and exit\n",
            "\n",
            "Examples:\n",
            "  This sort via utf-8 code:\n",
            "    cat file1.txt | aki-resort\n",
            "  This sort via 1st chunk of numeric character according to numeric:\n",
            "    cat file1.txt | aki-resort -e \"[0-9]+\" --according-to numeric\n",
            "\n",
        )
    };
}

/*
macro_rules! try_help_msg {
    () => {
        "Try --help for help.\n"
    };
}
*/

macro_rules! program_name {
    () => {
        "aki-resort"
    };
}

macro_rules! version_msg {
    () => {
        concat!(program_name!(), " ", env!("CARGO_PKG_VERSION"), "\n")
    };
}

/*
macro_rules! fixture_text10k {
    () => {
        "fixtures/text10k.txt"
    };
}
*/

macro_rules! do_execute {
    ($args:expr) => {
        do_execute!($args, "")
    };
    ($args:expr, $sin:expr,) => {
        do_execute!($args, $sin)
    };
    ($args:expr, $sin:expr) => {{
        let sioe = RunnelIoe::new(
            Box::new(StringIn::with_str($sin)),
            Box::new(StringOut::default()),
            Box::new(StringErr::default()),
        );
        let program = env!("CARGO_PKG_NAME");
        let r = execute(&sioe, &program, $args);
        match r {
            Ok(_) => {}
            Err(ref err) => {
                let _ = sioe
                    .perr()
                    .lock()
                    .write_fmt(format_args!("{}: {:#}\n", program, err));
            }
        };
        (r, sioe)
    }};
}

macro_rules! buff {
    ($sioe:expr, serr) => {
        $sioe.perr().lock().buffer_str()
    };
    ($sioe:expr, sout) => {
        $sioe.pout().lock().buffer_str()
    };
}

mod test_s0 {
    use libaki_resort::*;
    use runnel::medium::stringio::{StringErr, StringIn, StringOut};
    use runnel::RunnelIoe;
    use std::io::Write;
    //
    #[test]
    fn test_help() {
        let (r, sioe) = do_execute!(&["-H"]);
        assert_eq!(buff!(sioe, serr), "");
        assert_eq!(buff!(sioe, sout), help_msg!());
        assert_eq!(r.is_ok(), true);
    }
    #[test]
    fn test_help_long() {
        let (r, sioe) = do_execute!(&["--help"]);
        assert_eq!(buff!(sioe, serr), "");
        assert_eq!(buff!(sioe, sout), help_msg!());
        assert_eq!(r.is_ok(), true);
    }
    #[test]
    fn test_version() {
        let (r, sioe) = do_execute!(&["-V"]);
        assert_eq!(buff!(sioe, serr), "");
        assert_eq!(buff!(sioe, sout), version_msg!());
        assert_eq!(r.is_ok(), true);
    }
    #[test]
    fn test_version_long() {
        let (r, sioe) = do_execute!(&["--version"]);
        assert_eq!(buff!(sioe, serr), "");
        assert_eq!(buff!(sioe, sout), version_msg!());
        assert_eq!(r.is_ok(), true);
    }
    /*
    #[test]
    fn test_non_option() {
        let (r, sioe) = do_execute!(&[] as &[&str]);
        #[rustfmt::skip]
        assert_eq!(
            buff!(sioe, serr),
            concat!(
                program_name!(), ": ",
                "Missing option: e\n",
                "Unexpected argument: \n",
                try_help_msg!()
            )
        );
        assert_eq!(buff!(sioe, sout), "");
        assert_eq!(r.is_ok(), false);
    }
    */
}

const IN_DAT_FRUIT: &str = "\
Apple:33:3.3:good:Mar
Orange:222:1.1.2:good:Jan
Cherry:4:4:good:Oct
Kiwi:1111:1.1.11:good:Jun
";

mod test_s_string {
    use libaki_resort::*;
    use runnel::medium::stringio::{StringErr, StringIn, StringOut};
    use runnel::RunnelIoe;
    use std::io::Write;
    //
    #[test]
    fn test_t1() {
        let (r, sioe) = do_execute!(&[], super::IN_DAT_FRUIT,);
        assert_eq!(buff!(sioe, serr), "");
        assert_eq!(
            buff!(sioe, sout),
            concat!(
                "Apple:33:3.3:good:Mar\n",
                "Cherry:4:4:good:Oct\n",
                "Kiwi:1111:1.1.11:good:Jun\n",
                "Orange:222:1.1.2:good:Jan\n",
            )
        );
        assert_eq!(r.is_ok(), true);
    }
    //
    #[test]
    fn test_t2() {
        let (r, sioe) = do_execute!(&["-r"], super::IN_DAT_FRUIT,);
        assert_eq!(buff!(sioe, serr), "");
        assert_eq!(
            buff!(sioe, sout),
            concat!(
                "Orange:222:1.1.2:good:Jan\n",
                "Kiwi:1111:1.1.11:good:Jun\n",
                "Cherry:4:4:good:Oct\n",
                "Apple:33:3.3:good:Mar\n",
            )
        );
        assert_eq!(r.is_ok(), true);
    }
    //
    #[test]
    fn test_t3() {
        let (r, sioe) = do_execute!(&["-e", "[0-9]+"], super::IN_DAT_FRUIT,);
        assert_eq!(buff!(sioe, serr), "");
        assert_eq!(
            buff!(sioe, sout),
            concat!(
                "Kiwi:1111:1.1.11:good:Jun\n",
                "Orange:222:1.1.2:good:Jan\n",
                "Apple:33:3.3:good:Mar\n",
                "Cherry:4:4:good:Oct\n",
            )
        );
        assert_eq!(r.is_ok(), true);
    }
    //
    #[test]
    fn test_t4() {
        let (r, sioe) = do_execute!(&["-e", "[0-9]+", "-r"], super::IN_DAT_FRUIT,);
        assert_eq!(buff!(sioe, serr), "");
        assert_eq!(
            buff!(sioe, sout),
            concat!(
                "Cherry:4:4:good:Oct\n",
                "Apple:33:3.3:good:Mar\n",
                "Orange:222:1.1.2:good:Jan\n",
                "Kiwi:1111:1.1.11:good:Jun\n",
            )
        );
        assert_eq!(r.is_ok(), true);
    }
    //
    #[test]
    fn test_t5() {
        let in_w = super::IN_DAT_FRUIT.to_string() + super::IN_DAT_FRUIT;
        let (r, sioe) = do_execute!(&["-e", "[0-9]+"], in_w.as_str(),);
        assert_eq!(buff!(sioe, serr), "");
        assert_eq!(
            buff!(sioe, sout),
            concat!(
                "Kiwi:1111:1.1.11:good:Jun\n",
                "Kiwi:1111:1.1.11:good:Jun\n",
                "Orange:222:1.1.2:good:Jan\n",
                "Orange:222:1.1.2:good:Jan\n",
                "Apple:33:3.3:good:Mar\n",
                "Apple:33:3.3:good:Mar\n",
                "Cherry:4:4:good:Oct\n",
                "Cherry:4:4:good:Oct\n",
            )
        );
        assert_eq!(r.is_ok(), true);
    }
}

mod test_s_numeric {
    use libaki_resort::*;
    use runnel::medium::stringio::{StringErr, StringIn, StringOut};
    use runnel::RunnelIoe;
    use std::io::Write;
    //
    #[test]
    fn test_t1() {
        let (r, sioe) = do_execute!(&["--according-to", "numeric"], super::IN_DAT_FRUIT,);
        assert_eq!(
            buff!(sioe, serr),
            concat!(
                program_name!(),
                ": (0,21):\'Apple:33:3.3:good:Mar\': invalid digit found in string\n"
            )
        );
        assert_eq!(buff!(sioe, sout), "");
        assert_eq!(r.is_ok(), false);
    }
    //
    #[test]
    fn test_t2() {
        let (r, sioe) = do_execute!(&["--according-to", "numeric", "-r"], super::IN_DAT_FRUIT,);
        assert_eq!(
            buff!(sioe, serr),
            concat!(
                program_name!(),
                ": (0,21):\'Apple:33:3.3:good:Mar\': invalid digit found in string\n"
            )
        );
        assert_eq!(buff!(sioe, sout), "");
        assert_eq!(r.is_ok(), false);
    }
    //
    #[test]
    fn test_t3() {
        let (r, sioe) = do_execute!(
            &["-e", "[0-9]+", "--according-to", "numeric"],
            super::IN_DAT_FRUIT,
        );
        assert_eq!(buff!(sioe, serr), "");
        assert_eq!(
            buff!(sioe, sout),
            concat!(
                "Cherry:4:4:good:Oct\n",
                "Apple:33:3.3:good:Mar\n",
                "Orange:222:1.1.2:good:Jan\n",
                "Kiwi:1111:1.1.11:good:Jun\n",
            )
        );
        assert_eq!(r.is_ok(), true);
    }
    //
    #[test]
    fn test_t4() {
        let (r, sioe) = do_execute!(
            &["-e", "[0-9]+", "--according-to", "numeric", "-r"],
            super::IN_DAT_FRUIT,
        );
        assert_eq!(buff!(sioe, serr), "");
        assert_eq!(
            buff!(sioe, sout),
            concat!(
                "Kiwi:1111:1.1.11:good:Jun\n",
                "Orange:222:1.1.2:good:Jan\n",
                "Apple:33:3.3:good:Mar\n",
                "Cherry:4:4:good:Oct\n",
            )
        );
        assert_eq!(r.is_ok(), true);
    }
    //
    #[test]
    fn test_t5() {
        let in_w = super::IN_DAT_FRUIT.to_string() + super::IN_DAT_FRUIT;
        let (r, sioe) = do_execute!(
            &["-e", "[0-9]+", "--according-to", "numeric"],
            in_w.as_str(),
        );
        assert_eq!(buff!(sioe, serr), "");
        assert_eq!(
            buff!(sioe, sout),
            concat!(
                "Cherry:4:4:good:Oct\n",
                "Cherry:4:4:good:Oct\n",
                "Apple:33:3.3:good:Mar\n",
                "Apple:33:3.3:good:Mar\n",
                "Orange:222:1.1.2:good:Jan\n",
                "Orange:222:1.1.2:good:Jan\n",
                "Kiwi:1111:1.1.11:good:Jun\n",
                "Kiwi:1111:1.1.11:good:Jun\n",
            )
        );
        assert_eq!(r.is_ok(), true);
    }
}

mod test_s_version {
    use libaki_resort::*;
    use runnel::medium::stringio::{StringErr, StringIn, StringOut};
    use runnel::RunnelIoe;
    use std::io::Write;
    //
    #[test]
    fn test_t1() {
        let (r, sioe) = do_execute!(&["--according-to", "version"], super::IN_DAT_FRUIT,);
        assert_eq!(
            buff!(sioe, serr),
            concat!(
                program_name!(),
                ": (0,21):\'Apple:33:3.3:good:Mar\': lexer error: UnexpectedChar(\':\')\n",
            )
        );
        assert_eq!(buff!(sioe, sout), "");
        assert_eq!(r.is_ok(), false);
    }
    //
    #[test]
    fn test_t2() {
        let (r, sioe) = do_execute!(&["--according-to", "version", "-r"], super::IN_DAT_FRUIT,);
        assert_eq!(
            buff!(sioe, serr),
            concat!(
                program_name!(),
                ": (0,21):\'Apple:33:3.3:good:Mar\': lexer error: UnexpectedChar(\':\')\n",
            )
        );
        assert_eq!(buff!(sioe, sout), "");
        assert_eq!(r.is_ok(), false);
    }
    //
    #[test]
    fn test_t3() {
        let (r, sioe) = do_execute!(
            &["-e", "[^:]+:[^:]+:([0-9.]+):", "--according-to", "version"],
            super::IN_DAT_FRUIT,
        );
        assert_eq!(buff!(sioe, serr), "");
        assert_eq!(
            buff!(sioe, sout),
            concat!(
                "Orange:222:1.1.2:good:Jan\n",
                "Kiwi:1111:1.1.11:good:Jun\n",
                "Apple:33:3.3:good:Mar\n",
                "Cherry:4:4:good:Oct\n",
            )
        );
        assert_eq!(r.is_ok(), true);
    }
    //
    #[test]
    fn test_t4() {
        let (r, sioe) = do_execute!(
            &[
                "-e",
                "[^:]+:[^:]+:([0-9.]+):",
                "--according-to",
                "version",
                "-r",
            ],
            super::IN_DAT_FRUIT,
        );
        assert_eq!(buff!(sioe, serr), "");
        assert_eq!(
            buff!(sioe, sout),
            concat!(
                "Cherry:4:4:good:Oct\n",
                "Apple:33:3.3:good:Mar\n",
                "Kiwi:1111:1.1.11:good:Jun\n",
                "Orange:222:1.1.2:good:Jan\n",
            )
        );
        assert_eq!(r.is_ok(), true);
    }
    //
    #[test]
    fn test_t5() {
        let in_w = super::IN_DAT_FRUIT.to_string() + super::IN_DAT_FRUIT;
        let (r, sioe) = do_execute!(
            &["-e", "[^:]+:[^:]+:([0-9.]+):", "--according-to", "version"],
            in_w.as_str(),
        );
        assert_eq!(buff!(sioe, serr), "");
        assert_eq!(
            buff!(sioe, sout),
            concat!(
                "Orange:222:1.1.2:good:Jan\n",
                "Orange:222:1.1.2:good:Jan\n",
                "Kiwi:1111:1.1.11:good:Jun\n",
                "Kiwi:1111:1.1.11:good:Jun\n",
                "Apple:33:3.3:good:Mar\n",
                "Apple:33:3.3:good:Mar\n",
                "Cherry:4:4:good:Oct\n",
                "Cherry:4:4:good:Oct\n",
            )
        );
        assert_eq!(r.is_ok(), true);
    }
}

mod test_s_month {
    use libaki_resort::*;
    use runnel::medium::stringio::{StringErr, StringIn, StringOut};
    use runnel::RunnelIoe;
    use std::io::Write;
    //
    #[test]
    fn test_t1() {
        let (r, sioe) = do_execute!(&["--according-to", "month"], super::IN_DAT_FRUIT,);
        assert_eq!(
            buff!(sioe, serr),
            concat!(
                program_name!(),
                ": (0,21):\'Apple:33:3.3:good:Mar\': invalid month strings\n",
            )
        );
        assert_eq!(buff!(sioe, sout), "");
        assert_eq!(r.is_ok(), false);
    }
    //
    #[test]
    fn test_t2() {
        let (r, sioe) = do_execute!(&["--according-to", "month", "-r"], super::IN_DAT_FRUIT,);
        assert_eq!(
            buff!(sioe, serr),
            concat!(
                program_name!(),
                ": (0,21):\'Apple:33:3.3:good:Mar\': invalid month strings\n",
            )
        );
        assert_eq!(buff!(sioe, sout), "");
        assert_eq!(r.is_ok(), false);
    }
    //
    #[test]
    fn test_t3() {
        let (r, sioe) = do_execute!(
            &["-e", ":([^:]+)$", "--according-to", "month"],
            super::IN_DAT_FRUIT,
        );
        assert_eq!(buff!(sioe, serr), "");
        assert_eq!(
            buff!(sioe, sout),
            concat!(
                "Orange:222:1.1.2:good:Jan\n",
                "Apple:33:3.3:good:Mar\n",
                "Kiwi:1111:1.1.11:good:Jun\n",
                "Cherry:4:4:good:Oct\n",
            )
        );
        assert_eq!(r.is_ok(), true);
    }
    //
    #[test]
    fn test_t4() {
        let (r, sioe) = do_execute!(
            &["-e", ":([^:]+)$", "--according-to", "month", "-r"],
            super::IN_DAT_FRUIT,
        );
        assert_eq!(buff!(sioe, serr), "");
        assert_eq!(
            buff!(sioe, sout),
            concat!(
                "Cherry:4:4:good:Oct\n",
                "Kiwi:1111:1.1.11:good:Jun\n",
                "Apple:33:3.3:good:Mar\n",
                "Orange:222:1.1.2:good:Jan\n",
            )
        );
        assert_eq!(r.is_ok(), true);
    }
    //
    #[test]
    fn test_t5() {
        let in_w = super::IN_DAT_FRUIT.to_string() + super::IN_DAT_FRUIT;
        let (r, sioe) = do_execute!(
            &["-e", ":([^:]+)$", "--according-to", "month"],
            in_w.as_str(),
        );
        assert_eq!(buff!(sioe, serr), "");
        assert_eq!(
            buff!(sioe, sout),
            concat!(
                "Orange:222:1.1.2:good:Jan\n",
                "Orange:222:1.1.2:good:Jan\n",
                "Apple:33:3.3:good:Mar\n",
                "Apple:33:3.3:good:Mar\n",
                "Kiwi:1111:1.1.11:good:Jun\n",
                "Kiwi:1111:1.1.11:good:Jun\n",
                "Cherry:4:4:good:Oct\n",
                "Cherry:4:4:good:Oct\n",
            )
        );
        assert_eq!(r.is_ok(), true);
    }
}

mod test_s_2 {
    use libaki_resort::*;
    use runnel::medium::stringio::{StringErr, StringIn, StringOut};
    use runnel::RunnelIoe;
    use std::io::Write;
    //
    #[test]
    fn test_max_buffer() {
        let (r, sioe) = do_execute!(&["--max-buffer", "20"], super::IN_DAT_FRUIT,);
        assert_eq!(
            buff!(sioe, serr),
            concat!(program_name!(), ": over max buffer size: 20\n")
        );
        assert_eq!(buff!(sioe, sout), "");
        assert_eq!(r.is_ok(), false);
    }
    //
    #[test]
    fn test_uniq() {
        let in_w = super::IN_DAT_FRUIT.to_string() + super::IN_DAT_FRUIT;
        let (r, sioe) = do_execute!(&["-u"], in_w.as_str(),);
        assert_eq!(buff!(sioe, serr), "");
        assert_eq!(
            buff!(sioe, sout),
            concat!(
                "Apple:33:3.3:good:Mar\n",
                "Cherry:4:4:good:Oct\n",
                "Kiwi:1111:1.1.11:good:Jun\n",
                "Orange:222:1.1.2:good:Jan\n",
            )
        );
        assert_eq!(r.is_ok(), true);
    }
}
/*
mod test_s3 {
    use libaki_resort::*;
    use runnel::RunnelIoe;
    use runnel::medium::stringio::{StringIn, StringOut, StringErr};
    use std::io::Write;
    //
     * can NOT test
    #[test]
    fn test_output_broken_pipe() {
    }
}
*/
/*
mod test_s4 {
    use libaki_resort::*;
    use runnel::medium::stringio::{StringErr, StringIn, StringOut};
    use runnel::RunnelIoe;
    use std::io::Write;
    //
    //
    // [BUG] thread 'main' panicked at 'begin <= end (4 <= 2) when slicing `$2 :: $0`', /checkout/src/libcore/str/mod.rs:2221:4
    // echo "001cea1eef55.softphone.blizoo.bg" | rust-gsub -e "(.*\\.){0,1}([A-Za-z0-9][A-Za-z0-9\\-]{1,61}(\\.[A-Za-z0-9]{2,}){0,1}(\\.[A-Za-z]{2,}){0,1}\\.[A-Za-z]{2,5})$" -f "\$2 :: \$0"
    //
    #[test]
    fn test_fix_bug_1() {
        let (r, sioe) = do_execute!(&[
                "-e",
                "(.*\\.){0,1}([A-Za-z0-9][A-Za-z0-9\\-]{1,61}(\\.[A-Za-z0-9]{2,}){0,1}(\\.[A-Za-z]{2,}){0,1}\\.[A-Za-z]{2,5})$",
                "-f",
                "$2 :: $0",
            ],
            "001cea1eef55.softphone.blizoo.bg\n");
        assert_eq!(buff!(sioe, serr), "");
        assert_eq!(
            buff!(sioe, sout),
            "blizoo.bg :: 001cea1eef55.softphone.blizoo.bg\n"
        );
        assert_eq!(r.is_ok(), true);
    }
    //
    #[test]
    fn test_fix_bug_2() {
        let (r, sioe) = do_execute!(
            &["-e", "ICON=\"[^\"]*\"", "-f", ""],
            "abc ICON=\"ABCDEFG\" defg\n"
        );
        assert_eq!(buff!(sioe, serr), "");
        assert_eq!(buff!(sioe, sout), "abc  defg\n");
        assert_eq!(r.is_ok(), true);
    }
}
*/
