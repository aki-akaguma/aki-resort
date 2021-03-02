const TARGET_EXE_PATH: &'static str = "target/debug/aki-resort";

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
            "  -k, --key <keydef>            sort via a key. keydef gives location.\n",
            "      --field-separator <sep>   use <sep> instead of non-blank to blank transition\n",
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

mod helper;

mod test_0 {
    use crate::helper::exec_target;
    //use exec_target::args_from;
    const TARGET_EXE_PATH: &'static str = super::TARGET_EXE_PATH;
    //
    #[test]
    fn test_help() {
        let oup = exec_target(TARGET_EXE_PATH, &["-H"]);
        assert_eq!(oup.stderr, "");
        assert_eq!(oup.stdout, help_msg!());
        assert_eq!(oup.status.success(), true);
    }
    #[test]
    fn test_help_long() {
        let oup = exec_target(TARGET_EXE_PATH, &["--help"]);
        assert_eq!(oup.stderr, "");
        assert_eq!(oup.stdout, help_msg!());
        assert_eq!(oup.status.success(), true);
    }
    #[test]
    fn test_version() {
        let oup = exec_target(TARGET_EXE_PATH, &["-V"]);
        assert_eq!(oup.stderr, "");
        assert_eq!(oup.stdout, version_msg!());
        assert_eq!(oup.status.success(), true);
    }
    #[test]
    fn test_version_long() {
        let oup = exec_target(TARGET_EXE_PATH, &["--version"]);
        assert_eq!(oup.stderr, "");
        assert_eq!(oup.stdout, version_msg!());
        assert_eq!(oup.status.success(), true);
    }
    /*
    #[test]
    fn test_non_option() {
        let oup = exec_target(TARGET_EXE_PATH, &[] as &[&str]);
        assert_eq!(
            oup.stderr,
            concat!(
                program_name!(),
                ": ",
                "Missing option: e\n",
                "Unexpected argument: \n",
                try_help_msg!()
            )
        );
        assert_eq!(oup.stdout, "");
        assert_eq!(oup.status.success(), false);
    }
    */
} // mod test_0

mod test_string {
    use crate::helper::exec_target_with_in;
    const TARGET_EXE_PATH: &'static str = super::TARGET_EXE_PATH;
    //
    #[test]
    fn test_t1() {
        let oup = exec_target_with_in(
            TARGET_EXE_PATH,
            &[] as &[&str],
            concat!(
                "ABCDEFG:33:abc\n",
                "OPQRSTU:222:opq\n",
                "VWXYZ:4:vwx\n",
                "HIJKLMN:1111:hij\n",
            )
            .as_bytes(),
        );
        assert_eq!(oup.stderr, "");
        assert_eq!(
            oup.stdout,
            concat!(
                "ABCDEFG:33:abc\n",
                "HIJKLMN:1111:hij\n",
                "OPQRSTU:222:opq\n",
                "VWXYZ:4:vwx\n",
            )
        );
        assert_eq!(oup.status.success(), true);
    }
    //
    #[test]
    fn test_t2() {
        let oup = exec_target_with_in(
            TARGET_EXE_PATH,
            &["-r"],
            concat!(
                "ABCDEFG:33:abc\n",
                "OPQRSTU:222:opq\n",
                "VWXYZ:4:vwx\n",
                "HIJKLMN:1111:hij\n",
            )
            .as_bytes(),
        );
        assert_eq!(oup.stderr, "");
        assert_eq!(
            oup.stdout,
            concat!(
                "VWXYZ:4:vwx\n",
                "OPQRSTU:222:opq\n",
                "HIJKLMN:1111:hij\n",
                "ABCDEFG:33:abc\n",
            )
        );
        assert_eq!(oup.status.success(), true);
    }
    //
    #[test]
    fn test_t3() {
        let oup = exec_target_with_in(
            TARGET_EXE_PATH,
            &["-e", "[0-9]+"],
            concat!(
                "ABCDEFG:33:abc\n",
                "OPQRSTU:222:opq\n",
                "VWXYZ:4:vwx\n",
                "HIJKLMN:1111:hij\n",
            )
            .as_bytes(),
        );
        assert_eq!(oup.stderr, "");
        assert_eq!(
            oup.stdout,
            concat!(
                "HIJKLMN:1111:hij\n",
                "OPQRSTU:222:opq\n",
                "ABCDEFG:33:abc\n",
                "VWXYZ:4:vwx\n",
            )
        );
        assert_eq!(oup.status.success(), true);
    }
    //
    #[test]
    fn test_t4() {
        let oup = exec_target_with_in(
            TARGET_EXE_PATH,
            &["-e", "[0-9]+", "-r"],
            concat!(
                "ABCDEFG:33:abc\n",
                "OPQRSTU:222:opq\n",
                "VWXYZ:4:vwx\n",
                "HIJKLMN:1111:hij\n",
            )
            .as_bytes(),
        );
        assert_eq!(oup.stderr, "");
        assert_eq!(
            oup.stdout,
            concat!(
                "VWXYZ:4:vwx\n",
                "ABCDEFG:33:abc\n",
                "OPQRSTU:222:opq\n",
                "HIJKLMN:1111:hij\n",
            )
        );
        assert_eq!(oup.status.success(), true);
    }
    //
    #[test]
    fn test_t5() {
        let oup = exec_target_with_in(
            TARGET_EXE_PATH,
            &["-e", "[0-9]+"],
            concat!(
                "ABCDEFG:33:abc\n",
                "OPQRSTU:222:opq\n",
                "VWXYZ:4:vwx\n",
                "HIJKLMN:1111:hij\n",
                "ABCDEFG:33:abc\n",
                "OPQRSTU:222:opq\n",
                "VWXYZ:4:vwx\n",
                "HIJKLMN:1111:hij\n",
            )
            .as_bytes(),
        );
        assert_eq!(oup.stderr, "");
        assert_eq!(
            oup.stdout,
            concat!(
                "HIJKLMN:1111:hij\n",
                "HIJKLMN:1111:hij\n",
                "OPQRSTU:222:opq\n",
                "OPQRSTU:222:opq\n",
                "ABCDEFG:33:abc\n",
                "ABCDEFG:33:abc\n",
                "VWXYZ:4:vwx\n",
                "VWXYZ:4:vwx\n",
            )
        );
        assert_eq!(oup.status.success(), true);
    }
} // mod test_1

mod test_numeric {
    use crate::helper::exec_target_with_in;
    const TARGET_EXE_PATH: &'static str = super::TARGET_EXE_PATH;
    //
    #[test]
    fn test_t1() {
        let oup = exec_target_with_in(
            TARGET_EXE_PATH,
            &["--according-to", "numeric"],
            concat!(
                "ABCDEFG:33:abc\n",
                "OPQRSTU:222:opq\n",
                "VWXYZ:1:vwx\n",
                "HIJKLMN:1111:hij\n",
            )
            .as_bytes(),
        );
        assert_eq!(
            oup.stderr,
            concat!(
                program_name!(),
                ": (0,14):\'ABCDEFG:33:abc\': invalid digit found in string\n",
            )
        );
        assert_eq!(oup.stdout, "");
        assert_eq!(oup.status.success(), false);
    }
    //
    #[test]
    fn test_t2() {
        let oup = exec_target_with_in(
            TARGET_EXE_PATH,
            &["--according-to", "numeric", "-r"],
            concat!(
                "ABCDEFG:33:abc\n",
                "OPQRSTU:222:opq\n",
                "VWXYZ:1:vwx\n",
                "HIJKLMN:1111:hij\n",
            )
            .as_bytes(),
        );
        assert_eq!(
            oup.stderr,
            concat!(
                program_name!(),
                ": (0,14):\'ABCDEFG:33:abc\': invalid digit found in string\n"
            )
        );
        assert_eq!(oup.stdout, "");
        assert_eq!(oup.status.success(), false);
    }
    //
    #[test]
    fn test_t3() {
        let oup = exec_target_with_in(
            TARGET_EXE_PATH,
            &["-e", "[0-9]+", "--according-to", "numeric"],
            concat!(
                "ABCDEFG:33:abc\n",
                "OPQRSTU:222:opq\n",
                "VWXYZ:4:vwx\n",
                "HIJKLMN:1111:hij\n",
            )
            .as_bytes(),
        );
        assert_eq!(oup.stderr, "");
        assert_eq!(
            oup.stdout,
            concat!(
                "VWXYZ:4:vwx\n",
                "ABCDEFG:33:abc\n",
                "OPQRSTU:222:opq\n",
                "HIJKLMN:1111:hij\n",
            )
        );
        assert_eq!(oup.status.success(), true);
    }
    //
    #[test]
    fn test_t4() {
        let oup = exec_target_with_in(
            TARGET_EXE_PATH,
            &["-e", "[0-9]+", "--according-to", "numeric", "-r"],
            concat!(
                "ABCDEFG:33:abc\n",
                "OPQRSTU:222:opq\n",
                "VWXYZ:4:vwx\n",
                "HIJKLMN:1111:hij\n",
            )
            .as_bytes(),
        );
        assert_eq!(oup.stderr, "");
        assert_eq!(
            oup.stdout,
            concat!(
                "HIJKLMN:1111:hij\n",
                "OPQRSTU:222:opq\n",
                "ABCDEFG:33:abc\n",
                "VWXYZ:4:vwx\n",
            )
        );
        assert_eq!(oup.status.success(), true);
    }
    //
    #[test]
    fn test_t5() {
        let oup = exec_target_with_in(
            TARGET_EXE_PATH,
            &["-e", "[0-9]+", "--according-to", "numeric"],
            concat!(
                "ABCDEFG:33:abc\n",
                "OPQRSTU:222:opq\n",
                "VWXYZ:4:vwx\n",
                "HIJKLMN:1111:hij\n",
                "ABCDEFG:33:abc\n",
                "OPQRSTU:222:opq\n",
                "VWXYZ:4:vwx\n",
                "HIJKLMN:1111:hij\n",
            )
            .as_bytes(),
        );
        assert_eq!(oup.stderr, "");
        assert_eq!(
            oup.stdout,
            concat!(
                "VWXYZ:4:vwx\n",
                "VWXYZ:4:vwx\n",
                "ABCDEFG:33:abc\n",
                "ABCDEFG:33:abc\n",
                "OPQRSTU:222:opq\n",
                "OPQRSTU:222:opq\n",
                "HIJKLMN:1111:hij\n",
                "HIJKLMN:1111:hij\n",
            )
        );
        assert_eq!(oup.status.success(), true);
    }
} // mod test_2

mod test_3 {
    use crate::helper::exec_target_with_in;
    const TARGET_EXE_PATH: &'static str = super::TARGET_EXE_PATH;
    //
    #[test]
    fn test_t1() {
        let oup = exec_target_with_in(
            TARGET_EXE_PATH,
            &["--max-buffer", "20"],
            concat!(
                "ABCDEFG:33:abc\n",
                "OPQRSTU:222:opq\n",
                "VWXYZ:1:vwx\n",
                "HIJKLMN:1111:hij\n",
            )
            .as_bytes(),
        );
        assert_eq!(
            oup.stderr,
            concat!(program_name!(), ": over max buffer size: 20\n")
        );
        assert_eq!(oup.stdout, "");
        assert_eq!(oup.status.success(), false);
    }
}
/*
mod test_3 {
    use crate::helper::exec_target;
    const TARGET_EXE_PATH: &'static str = super::TARGET_EXE_PATH;
    //
    #[test]
    fn test_output_broken_pipe() {
        let cmdstr = format!(
            "cat \"{}\" | \"{}\" -e \"A\" -f a | head -n 2",
            fixture_text10k!(),
            TARGET_EXE_PATH
        );
        let oup = exec_target("sh", &["-c", &cmdstr]);
        assert_eq!(oup.stderr, "");
        assert_eq!(oup.stdout, "aBCDEFG\nHIJKLMN\n");
        assert_eq!(oup.status.success(), true);
    }
}
*/
/*
mod test_4 {
    use crate::helper::exec_target_with_in;
    //use exec_target::args_from;
    const TARGET_EXE_PATH: &'static str = super::TARGET_EXE_PATH;

    //
    // [BUG] thread 'main' panicked at 'begin <= end (4 <= 2) when slicing `$2 :: $0`', /checkout/src/libcore/str/mod.rs:2221:4
    // echo "001cea1eef55.softphone.blizoo.bg" | rust-gsub -e "(.*\\.){0,1}([A-Za-z0-9][A-Za-z0-9\\-]{1,61}(\\.[A-Za-z0-9]{2,}){0,1}(\\.[A-Za-z]{2,}){0,1}\\.[A-Za-z]{2,5})$" -f "\$2 :: \$0"
    //
    #[test]
    fn test_fix_bug_1() {
        let oup = exec_target_with_in(TARGET_EXE_PATH,
            &[
                "-e",
                "(.*\\.){0,1}([A-Za-z0-9][A-Za-z0-9\\-]{1,61}(\\.[A-Za-z0-9]{2,}){0,1}(\\.[A-Za-z]{2,}){0,1}\\.[A-Za-z]{2,5})$",
                "-f",
                "$2 :: $0",
            ],
            b"001cea1eef55.softphone.blizoo.bg\n" as &[u8]);
        assert_eq!(oup.stderr, "");
        assert_eq!(
            oup.stdout,
            "blizoo.bg :: 001cea1eef55.softphone.blizoo.bg\n"
        );
        assert_eq!(oup.status.success(), true);
    }
    //
    #[test]
    fn test_fix_bug_2() {
        let oup = exec_target_with_in(
            TARGET_EXE_PATH,
            &["-e", "ICON=\"[^\"]*\"", "-f", ""],
            b"abc ICON=\"ABCDEFG\" defg\n" as &[u8],
        );
        assert_eq!(oup.stderr, "");
        assert_eq!(oup.stdout, "abc  defg\n");
        assert_eq!(oup.status.success(), true);
    }
}
*/
