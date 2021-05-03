const TARGET_EXE_PATH: &'static str = env!("CARGO_BIN_EXE_aki-resort");

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
            "      --according-to <word>     sort according to <word>\n",
            "  -h, --head <num>              unsort the first <num> lines.\n",
            "  -t, --tail <num>              unsort the last <num> lines.\n",
            "\n",
            "Other options:\n",
            "      --color <when>            use markers to highlight the matching strings\n",
            "  -e, --exp <exp>               regular expression. sort by the entires match\n",
            "  -u, --unique                  output only the first line of an equal\n",
            "      --max-buffer <size>       max buffer size\n",
            "\n",
            "  -H, --help        display this help and exit\n",
            "  -V, --version     display version information and exit\n",
            "  -X <x-options>    x options. try -X help\n",
            "\n",
            "Option Parameters:\n",
            "  <word>    'string', 'numeric', 'month', 'version'\n",
            "  <when>    'always', 'never', or 'auto'\n",
            "  <exp>     regular expression, sort by the entires match.\n",
            "  <size>    if a reading size is more than <size>, then it is not output,\n",
            "            quit and display error message.\n",
            "\n",
            "Environments:\n",
            "  AKI_RESORT_COLOR_SEQ_ST   color start sequence specified by ansi\n",
            "  AKI_RESORT_COLOR_SEQ_ED   color end sequence specified by ansi\n",
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

//
macro_rules! color_start {
    //() => { "\u{1B}[01;31m" }
    () => {
        "<S>"
    };
}
macro_rules! color_end {
    //() => {"\u{1B}[0m"}
    () => {
        "<E>"
    };
}
macro_rules! env_1 {
    () => {{
        let mut env = std::collections::HashMap::new();
        env.insert(
            "AKI_RESORT_COLOR_SEQ_ST".to_string(),
            color_start!().to_string(),
        );
        env.insert(
            "AKI_RESORT_COLOR_SEQ_ED".to_string(),
            color_end!().to_string(),
        );
        env
    }};
}

const IN_DAT_FRUIT_HEADER: &str = "\
name:number:version:nice:month
";
const IN_DAT_FRUIT_FOOTER: &str = "\
This is footer line. 1
";
const IN_DAT_FRUIT: &str = "\
Apple:33:3.3:good:Mar
Orange:222:1.1.2:good:Jan
Cherry:4:4:good:Oct
Kiwi:1111:1.1.11:good:Jun
";

mod helper;

mod test_0 {
    use crate::helper::exec_target;
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
}

mod test_string {
    use crate::helper::exec_target_with_in;
    const TARGET_EXE_PATH: &'static str = super::TARGET_EXE_PATH;
    //
    #[test]
    fn test_t1() {
        let oup = exec_target_with_in(
            TARGET_EXE_PATH,
            &[] as &[&str],
            super::IN_DAT_FRUIT.as_bytes(),
        );
        assert_eq!(oup.stderr, "");
        assert_eq!(
            oup.stdout,
            concat!(
                "Apple:33:3.3:good:Mar\n",
                "Cherry:4:4:good:Oct\n",
                "Kiwi:1111:1.1.11:good:Jun\n",
                "Orange:222:1.1.2:good:Jan\n",
            )
        );
        assert_eq!(oup.status.success(), true);
    }
    //
    #[test]
    fn test_t2() {
        let oup = exec_target_with_in(TARGET_EXE_PATH, &["-r"], super::IN_DAT_FRUIT.as_bytes());
        assert_eq!(oup.stderr, "");
        assert_eq!(
            oup.stdout,
            concat!(
                "Orange:222:1.1.2:good:Jan\n",
                "Kiwi:1111:1.1.11:good:Jun\n",
                "Cherry:4:4:good:Oct\n",
                "Apple:33:3.3:good:Mar\n",
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
            super::IN_DAT_FRUIT.as_bytes(),
        );
        assert_eq!(oup.stderr, "");
        assert_eq!(
            oup.stdout,
            concat!(
                "Kiwi:1111:1.1.11:good:Jun\n",
                "Orange:222:1.1.2:good:Jan\n",
                "Apple:33:3.3:good:Mar\n",
                "Cherry:4:4:good:Oct\n",
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
            super::IN_DAT_FRUIT.as_bytes(),
        );
        assert_eq!(oup.stderr, "");
        assert_eq!(
            oup.stdout,
            concat!(
                "Cherry:4:4:good:Oct\n",
                "Apple:33:3.3:good:Mar\n",
                "Orange:222:1.1.2:good:Jan\n",
                "Kiwi:1111:1.1.11:good:Jun\n",
            )
        );
        assert_eq!(oup.status.success(), true);
    }
    //
    #[test]
    fn test_t5() {
        let in_w = super::IN_DAT_FRUIT.to_string() + super::IN_DAT_FRUIT;
        let oup = exec_target_with_in(TARGET_EXE_PATH, &["-e", "[0-9]+"], in_w.as_bytes());
        assert_eq!(oup.stderr, "");
        assert_eq!(
            oup.stdout,
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
        assert_eq!(oup.status.success(), true);
    }
    //
    #[test]
    fn test_t6_header() {
        let in_w = super::IN_DAT_FRUIT_HEADER.to_string() + super::IN_DAT_FRUIT;
        let oup = exec_target_with_in(
            TARGET_EXE_PATH,
            &["-e", "[0-9]+", "-h", "1"],
            in_w.as_bytes(),
        );
        assert_eq!(oup.stderr, "");
        assert_eq!(
            oup.stdout,
            concat!(
                "name:number:version:nice:month\n",
                "Kiwi:1111:1.1.11:good:Jun\n",
                "Orange:222:1.1.2:good:Jan\n",
                "Apple:33:3.3:good:Mar\n",
                "Cherry:4:4:good:Oct\n",
            )
        );
        assert_eq!(oup.status.success(), true);
    }
    //
    #[test]
    fn test_t6_footer() {
        let in_w = super::IN_DAT_FRUIT.to_string() + super::IN_DAT_FRUIT_FOOTER;
        let oup = exec_target_with_in(
            TARGET_EXE_PATH,
            &["-e", "[0-9]+", "-t", "1"],
            in_w.as_bytes(),
        );
        assert_eq!(oup.stderr, "");
        assert_eq!(
            oup.stdout,
            concat!(
                "Kiwi:1111:1.1.11:good:Jun\n",
                "Orange:222:1.1.2:good:Jan\n",
                "Apple:33:3.3:good:Mar\n",
                "Cherry:4:4:good:Oct\n",
                "This is footer line. 1\n",
            )
        );
        assert_eq!(oup.status.success(), true);
    }
}

mod test_string_color {
    use crate::helper::exec_target_with_env_in;
    const TARGET_EXE_PATH: &'static str = super::TARGET_EXE_PATH;
    //
    #[test]
    fn test_t1() {
        let env = env_1!();
        let oup = exec_target_with_env_in(
            TARGET_EXE_PATH,
            &["--color", "always"],
            env,
            super::IN_DAT_FRUIT.as_bytes(),
        );
        assert_eq!(oup.stderr, "");
        assert_eq!(
            oup.stdout,
            concat!(
                "<S>Apple:33:3.3:good:Mar<E>\n",
                "<S>Cherry:4:4:good:Oct<E>\n",
                "<S>Kiwi:1111:1.1.11:good:Jun<E>\n",
                "<S>Orange:222:1.1.2:good:Jan<E>\n",
            )
        );
        assert_eq!(oup.status.success(), true);
    }
    //
    #[test]
    fn test_t2() {
        let env = env_1!();
        let oup = exec_target_with_env_in(
            TARGET_EXE_PATH,
            &["-r", "--color", "always"],
            env,
            super::IN_DAT_FRUIT.as_bytes(),
        );
        assert_eq!(oup.stderr, "");
        assert_eq!(
            oup.stdout,
            concat!(
                "<S>Orange:222:1.1.2:good:Jan<E>\n",
                "<S>Kiwi:1111:1.1.11:good:Jun<E>\n",
                "<S>Cherry:4:4:good:Oct<E>\n",
                "<S>Apple:33:3.3:good:Mar<E>\n",
            )
        );
        assert_eq!(oup.status.success(), true);
    }
    //
    #[test]
    fn test_t3() {
        let env = env_1!();
        let oup = exec_target_with_env_in(
            TARGET_EXE_PATH,
            &["-e", "[0-9]+", "--color", "always"],
            env,
            super::IN_DAT_FRUIT.as_bytes(),
        );
        assert_eq!(oup.stderr, "");
        assert_eq!(
            oup.stdout,
            concat!(
                "Kiwi:<S>1111<E>:1.1.11:good:Jun\n",
                "Orange:<S>222<E>:1.1.2:good:Jan\n",
                "Apple:<S>33<E>:3.3:good:Mar\n",
                "Cherry:<S>4<E>:4:good:Oct\n",
            )
        );
        assert_eq!(oup.status.success(), true);
    }
    //
    #[test]
    fn test_t4() {
        let env = env_1!();
        let oup = exec_target_with_env_in(
            TARGET_EXE_PATH,
            &["-e", "[0-9]+", "-r", "--color", "always"],
            env,
            super::IN_DAT_FRUIT.as_bytes(),
        );
        assert_eq!(oup.stderr, "");
        assert_eq!(
            oup.stdout,
            concat!(
                "Cherry:<S>4<E>:4:good:Oct\n",
                "Apple:<S>33<E>:3.3:good:Mar\n",
                "Orange:<S>222<E>:1.1.2:good:Jan\n",
                "Kiwi:<S>1111<E>:1.1.11:good:Jun\n",
            )
        );
        assert_eq!(oup.status.success(), true);
    }
    //
    #[test]
    fn test_t5() {
        let env = env_1!();
        let in_w = super::IN_DAT_FRUIT.to_string() + super::IN_DAT_FRUIT;
        let oup = exec_target_with_env_in(
            TARGET_EXE_PATH,
            &["-e", "[0-9]+", "--color", "always"],
            env,
            in_w.as_bytes(),
        );
        assert_eq!(oup.stderr, "");
        assert_eq!(
            oup.stdout,
            concat!(
                "Kiwi:<S>1111<E>:1.1.11:good:Jun\n",
                "Kiwi:<S>1111<E>:1.1.11:good:Jun\n",
                "Orange:<S>222<E>:1.1.2:good:Jan\n",
                "Orange:<S>222<E>:1.1.2:good:Jan\n",
                "Apple:<S>33<E>:3.3:good:Mar\n",
                "Apple:<S>33<E>:3.3:good:Mar\n",
                "Cherry:<S>4<E>:4:good:Oct\n",
                "Cherry:<S>4<E>:4:good:Oct\n",
            )
        );
        assert_eq!(oup.status.success(), true);
    }
    //
    #[test]
    fn test_t6_header() {
        let env = env_1!();
        let in_w = super::IN_DAT_FRUIT_HEADER.to_string() + super::IN_DAT_FRUIT;
        let oup = exec_target_with_env_in(
            TARGET_EXE_PATH,
            &["-e", "[0-9]+", "-h", "1", "--color", "always"],
            env,
            in_w.as_bytes(),
        );
        assert_eq!(oup.stderr, "");
        assert_eq!(
            oup.stdout,
            concat!(
                "name:number:version:nice:month\n",
                "Kiwi:<S>1111<E>:1.1.11:good:Jun\n",
                "Orange:<S>222<E>:1.1.2:good:Jan\n",
                "Apple:<S>33<E>:3.3:good:Mar\n",
                "Cherry:<S>4<E>:4:good:Oct\n",
            )
        );
        assert_eq!(oup.status.success(), true);
    }
    //
    #[test]
    fn test_t6_footer() {
        let env = env_1!();
        let in_w = super::IN_DAT_FRUIT.to_string() + super::IN_DAT_FRUIT_FOOTER;
        let oup = exec_target_with_env_in(
            TARGET_EXE_PATH,
            &["-e", "[0-9]+", "-t", "1", "--color", "always"],
            env,
            in_w.as_bytes(),
        );
        assert_eq!(oup.stderr, "");
        assert_eq!(
            oup.stdout,
            concat!(
                "Kiwi:<S>1111<E>:1.1.11:good:Jun\n",
                "Orange:<S>222<E>:1.1.2:good:Jan\n",
                "Apple:<S>33<E>:3.3:good:Mar\n",
                "Cherry:<S>4<E>:4:good:Oct\n",
                "This is footer line. 1\n",
            )
        );
        assert_eq!(oup.status.success(), true);
    }
}

mod test_numeric {
    use crate::helper::exec_target_with_in;
    const TARGET_EXE_PATH: &'static str = super::TARGET_EXE_PATH;
    //
    #[test]
    fn test_t1() {
        let oup = exec_target_with_in(
            TARGET_EXE_PATH,
            &["--according-to", "numeric"],
            super::IN_DAT_FRUIT.as_bytes(),
        );
        assert_eq!(
            oup.stderr,
            concat!(
                program_name!(),
                ": (0,21):\'Apple:33:3.3:good:Mar\': invalid digit found in string\n"
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
            super::IN_DAT_FRUIT.as_bytes(),
        );
        assert_eq!(
            oup.stderr,
            concat!(
                program_name!(),
                ": (0,21):\'Apple:33:3.3:good:Mar\': invalid digit found in string\n"
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
            super::IN_DAT_FRUIT.as_bytes(),
        );
        assert_eq!(oup.stderr, "");
        assert_eq!(
            oup.stdout,
            concat!(
                "Cherry:4:4:good:Oct\n",
                "Apple:33:3.3:good:Mar\n",
                "Orange:222:1.1.2:good:Jan\n",
                "Kiwi:1111:1.1.11:good:Jun\n",
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
            super::IN_DAT_FRUIT.as_bytes(),
        );
        assert_eq!(oup.stderr, "");
        assert_eq!(
            oup.stdout,
            concat!(
                "Kiwi:1111:1.1.11:good:Jun\n",
                "Orange:222:1.1.2:good:Jan\n",
                "Apple:33:3.3:good:Mar\n",
                "Cherry:4:4:good:Oct\n",
            )
        );
        assert_eq!(oup.status.success(), true);
    }
    //
    #[test]
    fn test_t5() {
        let in_w = super::IN_DAT_FRUIT.to_string() + super::IN_DAT_FRUIT;
        let oup = exec_target_with_in(
            TARGET_EXE_PATH,
            &["-e", "[0-9]+", "--according-to", "numeric"],
            in_w.as_bytes(),
        );
        assert_eq!(oup.stderr, "");
        assert_eq!(
            oup.stdout,
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
        assert_eq!(oup.status.success(), true);
    }
    //
    #[test]
    fn test_t6_header() {
        let in_w = super::IN_DAT_FRUIT_HEADER.to_string() + super::IN_DAT_FRUIT;
        let oup = exec_target_with_in(
            TARGET_EXE_PATH,
            &["-e", "[0-9]+", "--according-to", "numeric", "-h", "1"],
            in_w.as_bytes(),
        );
        assert_eq!(oup.stderr, "");
        assert_eq!(
            oup.stdout,
            concat!(
                "name:number:version:nice:month\n",
                "Cherry:4:4:good:Oct\n",
                "Apple:33:3.3:good:Mar\n",
                "Orange:222:1.1.2:good:Jan\n",
                "Kiwi:1111:1.1.11:good:Jun\n",
            )
        );
        assert_eq!(oup.status.success(), true);
    }
    //
    #[test]
    fn test_t6_footer() {
        let in_w = super::IN_DAT_FRUIT.to_string() + super::IN_DAT_FRUIT_FOOTER;
        let oup = exec_target_with_in(
            TARGET_EXE_PATH,
            &["-e", "[0-9]+", "--according-to", "numeric", "-t", "1"],
            in_w.as_bytes(),
        );
        assert_eq!(oup.stderr, "");
        assert_eq!(
            oup.stdout,
            concat!(
                "Cherry:4:4:good:Oct\n",
                "Apple:33:3.3:good:Mar\n",
                "Orange:222:1.1.2:good:Jan\n",
                "Kiwi:1111:1.1.11:good:Jun\n",
                "This is footer line. 1\n",
            )
        );
        assert_eq!(oup.status.success(), true);
    }
}

mod test_numeric_color {
    use crate::helper::exec_target_with_env_in;
    const TARGET_EXE_PATH: &'static str = super::TARGET_EXE_PATH;
    //
    #[test]
    fn test_t1() {
        let env = env_1!();
        let oup = exec_target_with_env_in(
            TARGET_EXE_PATH,
            &["--according-to", "numeric", "--color", "always"],
            env,
            super::IN_DAT_FRUIT.as_bytes(),
        );
        assert_eq!(
            oup.stderr,
            concat!(
                program_name!(),
                ": (0,21):\'Apple:33:3.3:good:Mar\': invalid digit found in string\n"
            )
        );
        assert_eq!(oup.stdout, "");
        assert_eq!(oup.status.success(), false);
    }
    //
    #[test]
    fn test_t2() {
        let env = env_1!();
        let oup = exec_target_with_env_in(
            TARGET_EXE_PATH,
            &["--according-to", "numeric", "-r", "--color", "always"],
            env,
            super::IN_DAT_FRUIT.as_bytes(),
        );
        assert_eq!(
            oup.stderr,
            concat!(
                program_name!(),
                ": (0,21):\'Apple:33:3.3:good:Mar\': invalid digit found in string\n"
            )
        );
        assert_eq!(oup.stdout, "");
        assert_eq!(oup.status.success(), false);
    }
    //
    #[test]
    fn test_t3() {
        let env = env_1!();
        let oup = exec_target_with_env_in(
            TARGET_EXE_PATH,
            &[
                "-e",
                "[0-9]+",
                "--according-to",
                "numeric",
                "--color",
                "always",
            ],
            env,
            super::IN_DAT_FRUIT.as_bytes(),
        );
        assert_eq!(oup.stderr, "");
        assert_eq!(
            oup.stdout,
            concat!(
                "Cherry:<S>4<E>:4:good:Oct\n",
                "Apple:<S>33<E>:3.3:good:Mar\n",
                "Orange:<S>222<E>:1.1.2:good:Jan\n",
                "Kiwi:<S>1111<E>:1.1.11:good:Jun\n",
            )
        );
        assert_eq!(oup.status.success(), true);
    }
    //
    #[test]
    fn test_t4() {
        let env = env_1!();
        let oup = exec_target_with_env_in(
            TARGET_EXE_PATH,
            &[
                "-e",
                "[0-9]+",
                "--according-to",
                "numeric",
                "-r",
                "--color",
                "always",
            ],
            env,
            super::IN_DAT_FRUIT.as_bytes(),
        );
        assert_eq!(oup.stderr, "");
        assert_eq!(
            oup.stdout,
            concat!(
                "Kiwi:<S>1111<E>:1.1.11:good:Jun\n",
                "Orange:<S>222<E>:1.1.2:good:Jan\n",
                "Apple:<S>33<E>:3.3:good:Mar\n",
                "Cherry:<S>4<E>:4:good:Oct\n",
            )
        );
        assert_eq!(oup.status.success(), true);
    }
    //
    #[test]
    fn test_t5() {
        let env = env_1!();
        let in_w = super::IN_DAT_FRUIT.to_string() + super::IN_DAT_FRUIT;
        let oup = exec_target_with_env_in(
            TARGET_EXE_PATH,
            &[
                "-e",
                "[0-9]+",
                "--according-to",
                "numeric",
                "--color",
                "always",
            ],
            env,
            in_w.as_bytes(),
        );
        assert_eq!(oup.stderr, "");
        assert_eq!(
            oup.stdout,
            concat!(
                "Cherry:<S>4<E>:4:good:Oct\n",
                "Cherry:<S>4<E>:4:good:Oct\n",
                "Apple:<S>33<E>:3.3:good:Mar\n",
                "Apple:<S>33<E>:3.3:good:Mar\n",
                "Orange:<S>222<E>:1.1.2:good:Jan\n",
                "Orange:<S>222<E>:1.1.2:good:Jan\n",
                "Kiwi:<S>1111<E>:1.1.11:good:Jun\n",
                "Kiwi:<S>1111<E>:1.1.11:good:Jun\n",
            )
        );
        assert_eq!(oup.status.success(), true);
    }
    //
    #[test]
    fn test_t6_header() {
        let env = env_1!();
        let in_w = super::IN_DAT_FRUIT_HEADER.to_string() + super::IN_DAT_FRUIT;
        let oup = exec_target_with_env_in(
            TARGET_EXE_PATH,
            &[
                "-e",
                "[0-9]+",
                "--according-to",
                "numeric",
                "-h",
                "1",
                "--color",
                "always",
            ],
            env,
            in_w.as_bytes(),
        );
        assert_eq!(oup.stderr, "");
        assert_eq!(
            oup.stdout,
            concat!(
                "name:number:version:nice:month\n",
                "Cherry:<S>4<E>:4:good:Oct\n",
                "Apple:<S>33<E>:3.3:good:Mar\n",
                "Orange:<S>222<E>:1.1.2:good:Jan\n",
                "Kiwi:<S>1111<E>:1.1.11:good:Jun\n",
            )
        );
        assert_eq!(oup.status.success(), true);
    }
    //
    #[test]
    fn test_t6_footer() {
        let env = env_1!();
        let in_w = super::IN_DAT_FRUIT.to_string() + super::IN_DAT_FRUIT_FOOTER;
        let oup = exec_target_with_env_in(
            TARGET_EXE_PATH,
            &[
                "-e",
                "[0-9]+",
                "--according-to",
                "numeric",
                "-t",
                "1",
                "--color",
                "always",
            ],
            env,
            in_w.as_bytes(),
        );
        assert_eq!(oup.stderr, "");
        assert_eq!(
            oup.stdout,
            concat!(
                "Cherry:<S>4<E>:4:good:Oct\n",
                "Apple:<S>33<E>:3.3:good:Mar\n",
                "Orange:<S>222<E>:1.1.2:good:Jan\n",
                "Kiwi:<S>1111<E>:1.1.11:good:Jun\n",
                "This is footer line. 1\n",
            )
        );
        assert_eq!(oup.status.success(), true);
    }
}

mod test_version {
    use crate::helper::exec_target_with_in;
    const TARGET_EXE_PATH: &'static str = super::TARGET_EXE_PATH;
    //
    #[test]
    fn test_t1() {
        let oup = exec_target_with_in(
            TARGET_EXE_PATH,
            &["--according-to", "version"],
            super::IN_DAT_FRUIT.as_bytes(),
        );
        assert_eq!(
            oup.stderr,
            concat!(
                program_name!(),
                ": (0,21):\'Apple:33:3.3:good:Mar\': lexer error: UnexpectedChar(\':\')\n",
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
            &["--according-to", "version", "-r"],
            super::IN_DAT_FRUIT.as_bytes(),
        );
        assert_eq!(
            oup.stderr,
            concat!(
                program_name!(),
                ": (0,21):\'Apple:33:3.3:good:Mar\': lexer error: UnexpectedChar(\':\')\n",
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
            &["-e", "[^:]+:[^:]+:([0-9.]+):", "--according-to", "version"],
            super::IN_DAT_FRUIT.as_bytes(),
        );
        assert_eq!(oup.stderr, "");
        assert_eq!(
            oup.stdout,
            concat!(
                "Orange:222:1.1.2:good:Jan\n",
                "Kiwi:1111:1.1.11:good:Jun\n",
                "Apple:33:3.3:good:Mar\n",
                "Cherry:4:4:good:Oct\n",
            )
        );
        assert_eq!(oup.status.success(), true);
    }
    //
    #[test]
    fn test_t4() {
        let oup = exec_target_with_in(
            TARGET_EXE_PATH,
            &[
                "-e",
                "[^:]+:[^:]+:([0-9.]+):",
                "--according-to",
                "version",
                "-r",
            ],
            super::IN_DAT_FRUIT.as_bytes(),
        );
        assert_eq!(oup.stderr, "");
        assert_eq!(
            oup.stdout,
            concat!(
                "Cherry:4:4:good:Oct\n",
                "Apple:33:3.3:good:Mar\n",
                "Kiwi:1111:1.1.11:good:Jun\n",
                "Orange:222:1.1.2:good:Jan\n",
            )
        );
        assert_eq!(oup.status.success(), true);
    }
    //
    #[test]
    fn test_t5() {
        let in_w = super::IN_DAT_FRUIT.to_string() + super::IN_DAT_FRUIT;
        let oup = exec_target_with_in(
            TARGET_EXE_PATH,
            &["-e", "[^:]+:[^:]+:([0-9.]+):", "--according-to", "version"],
            in_w.as_bytes(),
        );
        assert_eq!(oup.stderr, "");
        assert_eq!(
            oup.stdout,
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
        assert_eq!(oup.status.success(), true);
    }
    //
    #[test]
    fn test_t6_header() {
        let in_w = super::IN_DAT_FRUIT_HEADER.to_string() + super::IN_DAT_FRUIT;
        let oup = exec_target_with_in(
            TARGET_EXE_PATH,
            &[
                "-e",
                "[^:]+:[^:]+:([0-9.]+):",
                "--according-to",
                "version",
                "-h",
                "1",
            ],
            in_w.as_bytes(),
        );
        assert_eq!(oup.stderr, "");
        assert_eq!(
            oup.stdout,
            concat!(
                "name:number:version:nice:month\n",
                "Orange:222:1.1.2:good:Jan\n",
                "Kiwi:1111:1.1.11:good:Jun\n",
                "Apple:33:3.3:good:Mar\n",
                "Cherry:4:4:good:Oct\n",
            )
        );
        assert_eq!(oup.status.success(), true);
    }
    //
    #[test]
    fn test_t6_footer() {
        let in_w = super::IN_DAT_FRUIT.to_string() + super::IN_DAT_FRUIT_FOOTER;
        let oup = exec_target_with_in(
            TARGET_EXE_PATH,
            &[
                "-e",
                "[^:]+:[^:]+:([0-9.]+):",
                "--according-to",
                "version",
                "-t",
                "1",
            ],
            in_w.as_bytes(),
        );
        assert_eq!(oup.stderr, "");
        assert_eq!(
            oup.stdout,
            concat!(
                "Orange:222:1.1.2:good:Jan\n",
                "Kiwi:1111:1.1.11:good:Jun\n",
                "Apple:33:3.3:good:Mar\n",
                "Cherry:4:4:good:Oct\n",
                "This is footer line. 1\n",
            )
        );
        assert_eq!(oup.status.success(), true);
    }
}

mod test_version_color {
    use crate::helper::exec_target_with_env_in;
    const TARGET_EXE_PATH: &'static str = super::TARGET_EXE_PATH;
    //
    #[test]
    fn test_t1() {
        let env = env_1!();
        let oup = exec_target_with_env_in(
            TARGET_EXE_PATH,
            &["--according-to", "version", "--color", "always"],
            env,
            super::IN_DAT_FRUIT.as_bytes(),
        );
        assert_eq!(
            oup.stderr,
            concat!(
                program_name!(),
                ": (0,21):\'Apple:33:3.3:good:Mar\': lexer error: UnexpectedChar(\':\')\n",
            )
        );
        assert_eq!(oup.stdout, "");
        assert_eq!(oup.status.success(), false);
    }
    //
    #[test]
    fn test_t2() {
        let env = env_1!();
        let oup = exec_target_with_env_in(
            TARGET_EXE_PATH,
            &["--according-to", "version", "-r", "--color", "always"],
            env,
            super::IN_DAT_FRUIT.as_bytes(),
        );
        assert_eq!(
            oup.stderr,
            concat!(
                program_name!(),
                ": (0,21):\'Apple:33:3.3:good:Mar\': lexer error: UnexpectedChar(\':\')\n",
            )
        );
        assert_eq!(oup.stdout, "");
        assert_eq!(oup.status.success(), false);
    }
    //
    #[test]
    fn test_t3() {
        let env = env_1!();
        let oup = exec_target_with_env_in(
            TARGET_EXE_PATH,
            &[
                "-e",
                "[^:]+:[^:]+:([0-9.]+):",
                "--according-to",
                "version",
                "--color",
                "always",
            ],
            env,
            super::IN_DAT_FRUIT.as_bytes(),
        );
        assert_eq!(oup.stderr, "");
        assert_eq!(
            oup.stdout,
            concat!(
                "Orange:222:<S>1.1.2<E>:good:Jan\n",
                "Kiwi:1111:<S>1.1.11<E>:good:Jun\n",
                "Apple:33:<S>3.3<E>:good:Mar\n",
                "Cherry:4:<S>4<E>:good:Oct\n",
            )
        );
        assert_eq!(oup.status.success(), true);
    }
    //
    #[test]
    fn test_t4() {
        let env = env_1!();
        let oup = exec_target_with_env_in(
            TARGET_EXE_PATH,
            &[
                "-e",
                "[^:]+:[^:]+:([0-9.]+):",
                "--according-to",
                "version",
                "-r",
                "--color",
                "always",
            ],
            env,
            super::IN_DAT_FRUIT.as_bytes(),
        );
        assert_eq!(oup.stderr, "");
        assert_eq!(
            oup.stdout,
            concat!(
                "Cherry:4:<S>4<E>:good:Oct\n",
                "Apple:33:<S>3.3<E>:good:Mar\n",
                "Kiwi:1111:<S>1.1.11<E>:good:Jun\n",
                "Orange:222:<S>1.1.2<E>:good:Jan\n",
            )
        );
        assert_eq!(oup.status.success(), true);
    }
    //
    #[test]
    fn test_t5() {
        let env = env_1!();
        let in_w = super::IN_DAT_FRUIT.to_string() + super::IN_DAT_FRUIT;
        let oup = exec_target_with_env_in(
            TARGET_EXE_PATH,
            &[
                "-e",
                "[^:]+:[^:]+:([0-9.]+):",
                "--according-to",
                "version",
                "--color",
                "always",
            ],
            env,
            in_w.as_bytes(),
        );
        assert_eq!(oup.stderr, "");
        assert_eq!(
            oup.stdout,
            concat!(
                "Orange:222:<S>1.1.2<E>:good:Jan\n",
                "Orange:222:<S>1.1.2<E>:good:Jan\n",
                "Kiwi:1111:<S>1.1.11<E>:good:Jun\n",
                "Kiwi:1111:<S>1.1.11<E>:good:Jun\n",
                "Apple:33:<S>3.3<E>:good:Mar\n",
                "Apple:33:<S>3.3<E>:good:Mar\n",
                "Cherry:4:<S>4<E>:good:Oct\n",
                "Cherry:4:<S>4<E>:good:Oct\n",
            )
        );
        assert_eq!(oup.status.success(), true);
    }
    //
    #[test]
    fn test_t6_header() {
        let env = env_1!();
        let in_w = super::IN_DAT_FRUIT_HEADER.to_string() + super::IN_DAT_FRUIT;
        let oup = exec_target_with_env_in(
            TARGET_EXE_PATH,
            &[
                "-e",
                "[^:]+:[^:]+:([0-9.]+):",
                "--according-to",
                "version",
                "-h",
                "1",
                "--color",
                "always",
            ],
            env,
            in_w.as_bytes(),
        );
        assert_eq!(oup.stderr, "");
        assert_eq!(
            oup.stdout,
            concat!(
                "name:number:version:nice:month\n",
                "Orange:222:<S>1.1.2<E>:good:Jan\n",
                "Kiwi:1111:<S>1.1.11<E>:good:Jun\n",
                "Apple:33:<S>3.3<E>:good:Mar\n",
                "Cherry:4:<S>4<E>:good:Oct\n",
            )
        );
        assert_eq!(oup.status.success(), true);
    }
    //
    #[test]
    fn test_t6_footer() {
        let env = env_1!();
        let in_w = super::IN_DAT_FRUIT.to_string() + super::IN_DAT_FRUIT_FOOTER;
        let oup = exec_target_with_env_in(
            TARGET_EXE_PATH,
            &[
                "-e",
                "[^:]+:[^:]+:([0-9.]+):",
                "--according-to",
                "version",
                "-t",
                "1",
                "--color",
                "always",
            ],
            env,
            in_w.as_bytes(),
        );
        assert_eq!(oup.stderr, "");
        assert_eq!(
            oup.stdout,
            concat!(
                "Orange:222:<S>1.1.2<E>:good:Jan\n",
                "Kiwi:1111:<S>1.1.11<E>:good:Jun\n",
                "Apple:33:<S>3.3<E>:good:Mar\n",
                "Cherry:4:<S>4<E>:good:Oct\n",
                "This is footer line. 1\n",
            )
        );
        assert_eq!(oup.status.success(), true);
    }
}

mod test_month {
    use crate::helper::exec_target_with_in;
    const TARGET_EXE_PATH: &'static str = super::TARGET_EXE_PATH;
    //
    #[test]
    fn test_t1() {
        let oup = exec_target_with_in(
            TARGET_EXE_PATH,
            &["--according-to", "month"],
            super::IN_DAT_FRUIT.as_bytes(),
        );
        assert_eq!(
            oup.stderr,
            concat!(
                program_name!(),
                ": (0,21):\'Apple:33:3.3:good:Mar\': invalid month strings\n",
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
            &["--according-to", "month", "-r"],
            super::IN_DAT_FRUIT.as_bytes(),
        );
        assert_eq!(
            oup.stderr,
            concat!(
                program_name!(),
                ": (0,21):\'Apple:33:3.3:good:Mar\': invalid month strings\n",
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
            &["-e", ":([^:]+)$", "--according-to", "month"],
            super::IN_DAT_FRUIT.as_bytes(),
        );
        assert_eq!(oup.stderr, "");
        assert_eq!(
            oup.stdout,
            concat!(
                "Orange:222:1.1.2:good:Jan\n",
                "Apple:33:3.3:good:Mar\n",
                "Kiwi:1111:1.1.11:good:Jun\n",
                "Cherry:4:4:good:Oct\n",
            )
        );
        assert_eq!(oup.status.success(), true);
    }
    //
    #[test]
    fn test_t4() {
        let oup = exec_target_with_in(
            TARGET_EXE_PATH,
            &["-e", ":([^:]+)$", "--according-to", "month", "-r"],
            super::IN_DAT_FRUIT.as_bytes(),
        );
        assert_eq!(oup.stderr, "");
        assert_eq!(
            oup.stdout,
            concat!(
                "Cherry:4:4:good:Oct\n",
                "Kiwi:1111:1.1.11:good:Jun\n",
                "Apple:33:3.3:good:Mar\n",
                "Orange:222:1.1.2:good:Jan\n",
            )
        );
        assert_eq!(oup.status.success(), true);
    }
    //
    #[test]
    fn test_t5() {
        let in_w = super::IN_DAT_FRUIT.to_string() + super::IN_DAT_FRUIT;
        let oup = exec_target_with_in(
            TARGET_EXE_PATH,
            &["-e", ":([^:]+)$", "--according-to", "month"],
            in_w.as_bytes(),
        );
        assert_eq!(oup.stderr, "");
        assert_eq!(
            oup.stdout,
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
        assert_eq!(oup.status.success(), true);
    }
    //
    #[test]
    fn test_t6_header() {
        let in_w = super::IN_DAT_FRUIT_HEADER.to_string() + super::IN_DAT_FRUIT;
        let oup = exec_target_with_in(
            TARGET_EXE_PATH,
            &["-e", ":([^:]+)$", "--according-to", "month", "-h", "1"],
            in_w.as_bytes(),
        );
        assert_eq!(oup.stderr, "");
        assert_eq!(
            oup.stdout,
            concat!(
                "name:number:version:nice:month\n",
                "Orange:222:1.1.2:good:Jan\n",
                "Apple:33:3.3:good:Mar\n",
                "Kiwi:1111:1.1.11:good:Jun\n",
                "Cherry:4:4:good:Oct\n",
            )
        );
        assert_eq!(oup.status.success(), true);
    }
    //
    #[test]
    fn test_t6_footer() {
        let in_w = super::IN_DAT_FRUIT.to_string() + super::IN_DAT_FRUIT_FOOTER;
        let oup = exec_target_with_in(
            TARGET_EXE_PATH,
            &["-e", ":([^:]+)$", "--according-to", "month", "-t", "1"],
            in_w.as_bytes(),
        );
        assert_eq!(oup.stderr, "");
        assert_eq!(
            oup.stdout,
            concat!(
                "Orange:222:1.1.2:good:Jan\n",
                "Apple:33:3.3:good:Mar\n",
                "Kiwi:1111:1.1.11:good:Jun\n",
                "Cherry:4:4:good:Oct\n",
                "This is footer line. 1\n",
            )
        );
        assert_eq!(oup.status.success(), true);
    }
}

mod test_month_color {
    use crate::helper::exec_target_with_env_in;
    const TARGET_EXE_PATH: &'static str = super::TARGET_EXE_PATH;
    //
    #[test]
    fn test_t1() {
        let env = env_1!();
        let oup = exec_target_with_env_in(
            TARGET_EXE_PATH,
            &["--according-to", "month", "--color", "always"],
            env,
            super::IN_DAT_FRUIT.as_bytes(),
        );
        assert_eq!(
            oup.stderr,
            concat!(
                program_name!(),
                ": (0,21):\'Apple:33:3.3:good:Mar\': invalid month strings\n",
            )
        );
        assert_eq!(oup.stdout, "");
        assert_eq!(oup.status.success(), false);
    }
    //
    #[test]
    fn test_t2() {
        let env = env_1!();
        let oup = exec_target_with_env_in(
            TARGET_EXE_PATH,
            &["--according-to", "month", "-r", "--color", "always"],
            env,
            super::IN_DAT_FRUIT.as_bytes(),
        );
        assert_eq!(
            oup.stderr,
            concat!(
                program_name!(),
                ": (0,21):\'Apple:33:3.3:good:Mar\': invalid month strings\n",
            )
        );
        assert_eq!(oup.stdout, "");
        assert_eq!(oup.status.success(), false);
    }
    //
    #[test]
    fn test_t3() {
        let env = env_1!();
        let oup = exec_target_with_env_in(
            TARGET_EXE_PATH,
            &[
                "-e",
                ":([^:]+)$",
                "--according-to",
                "month",
                "--color",
                "always",
            ],
            env,
            super::IN_DAT_FRUIT.as_bytes(),
        );
        assert_eq!(oup.stderr, "");
        assert_eq!(
            oup.stdout,
            concat!(
                "Orange:222:1.1.2:good:<S>Jan<E>\n",
                "Apple:33:3.3:good:<S>Mar<E>\n",
                "Kiwi:1111:1.1.11:good:<S>Jun<E>\n",
                "Cherry:4:4:good:<S>Oct<E>\n",
            )
        );
        assert_eq!(oup.status.success(), true);
    }
    //
    #[test]
    fn test_t4() {
        let env = env_1!();
        let oup = exec_target_with_env_in(
            TARGET_EXE_PATH,
            &[
                "-e",
                ":([^:]+)$",
                "--according-to",
                "month",
                "-r",
                "--color",
                "always",
            ],
            env,
            super::IN_DAT_FRUIT.as_bytes(),
        );
        assert_eq!(oup.stderr, "");
        assert_eq!(
            oup.stdout,
            concat!(
                "Cherry:4:4:good:<S>Oct<E>\n",
                "Kiwi:1111:1.1.11:good:<S>Jun<E>\n",
                "Apple:33:3.3:good:<S>Mar<E>\n",
                "Orange:222:1.1.2:good:<S>Jan<E>\n",
            )
        );
        assert_eq!(oup.status.success(), true);
    }
    //
    #[test]
    fn test_t5() {
        let env = env_1!();
        let in_w = super::IN_DAT_FRUIT.to_string() + super::IN_DAT_FRUIT;
        let oup = exec_target_with_env_in(
            TARGET_EXE_PATH,
            &[
                "-e",
                ":([^:]+)$",
                "--according-to",
                "month",
                "--color",
                "always",
            ],
            env,
            in_w.as_bytes(),
        );
        assert_eq!(oup.stderr, "");
        assert_eq!(
            oup.stdout,
            concat!(
                "Orange:222:1.1.2:good:<S>Jan<E>\n",
                "Orange:222:1.1.2:good:<S>Jan<E>\n",
                "Apple:33:3.3:good:<S>Mar<E>\n",
                "Apple:33:3.3:good:<S>Mar<E>\n",
                "Kiwi:1111:1.1.11:good:<S>Jun<E>\n",
                "Kiwi:1111:1.1.11:good:<S>Jun<E>\n",
                "Cherry:4:4:good:<S>Oct<E>\n",
                "Cherry:4:4:good:<S>Oct<E>\n",
            )
        );
        assert_eq!(oup.status.success(), true);
    }
    //
    #[test]
    fn test_t6_header() {
        let env = env_1!();
        let in_w = super::IN_DAT_FRUIT_HEADER.to_string() + super::IN_DAT_FRUIT;
        let oup = exec_target_with_env_in(
            TARGET_EXE_PATH,
            &[
                "-e",
                ":([^:]+)$",
                "--according-to",
                "month",
                "-h",
                "1",
                "--color",
                "always",
            ],
            env,
            in_w.as_bytes(),
        );
        assert_eq!(oup.stderr, "");
        assert_eq!(
            oup.stdout,
            concat!(
                "name:number:version:nice:month\n",
                "Orange:222:1.1.2:good:<S>Jan<E>\n",
                "Apple:33:3.3:good:<S>Mar<E>\n",
                "Kiwi:1111:1.1.11:good:<S>Jun<E>\n",
                "Cherry:4:4:good:<S>Oct<E>\n",
            )
        );
        assert_eq!(oup.status.success(), true);
    }
    //
    #[test]
    fn test_t6_footer() {
        let env = env_1!();
        let in_w = super::IN_DAT_FRUIT.to_string() + super::IN_DAT_FRUIT_FOOTER;
        let oup = exec_target_with_env_in(
            TARGET_EXE_PATH,
            &[
                "-e",
                ":([^:]+)$",
                "--according-to",
                "month",
                "-t",
                "1",
                "--color",
                "always",
            ],
            env,
            in_w.as_bytes(),
        );
        assert_eq!(oup.stderr, "");
        assert_eq!(
            oup.stdout,
            concat!(
                "Orange:222:1.1.2:good:<S>Jan<E>\n",
                "Apple:33:3.3:good:<S>Mar<E>\n",
                "Kiwi:1111:1.1.11:good:<S>Jun<E>\n",
                "Cherry:4:4:good:<S>Oct<E>\n",
                "This is footer line. 1\n",
            )
        );
        assert_eq!(oup.status.success(), true);
    }
}

mod test_2 {
    use crate::helper::exec_target_with_env_in;
    use crate::helper::exec_target_with_in;
    const TARGET_EXE_PATH: &'static str = super::TARGET_EXE_PATH;
    //
    #[test]
    fn test_max_buffer() {
        let oup = exec_target_with_in(
            TARGET_EXE_PATH,
            &["--max-buffer", "20"],
            super::IN_DAT_FRUIT.as_bytes(),
        );
        assert_eq!(
            oup.stderr,
            concat!(program_name!(), ": over max buffer size: 20\n")
        );
        assert_eq!(oup.stdout, "");
        assert_eq!(oup.status.success(), false);
    }
    //
    #[test]
    fn test_uniq() {
        let in_w = super::IN_DAT_FRUIT.to_string() + super::IN_DAT_FRUIT;
        let oup = exec_target_with_in(TARGET_EXE_PATH, &["-u"], in_w.as_bytes());
        assert_eq!(oup.stderr, "");
        assert_eq!(
            oup.stdout,
            concat!(
                "Apple:33:3.3:good:Mar\n",
                "Cherry:4:4:good:Oct\n",
                "Kiwi:1111:1.1.11:good:Jun\n",
                "Orange:222:1.1.2:good:Jan\n",
            )
        );
        assert_eq!(oup.status.success(), true);
    }
    //
    #[test]
    fn test_uniq_color() {
        let env = env_1!();
        let in_w = super::IN_DAT_FRUIT.to_string() + super::IN_DAT_FRUIT;
        let oup = exec_target_with_env_in(
            TARGET_EXE_PATH,
            &["-u", "--color", "always"],
            env,
            in_w.as_bytes(),
        );
        assert_eq!(oup.stderr, "");
        assert_eq!(
            oup.stdout,
            concat!(
                "<S>Apple:33:3.3:good:Mar<E>\n",
                "<S>Cherry:4:4:good:Oct<E>\n",
                "<S>Kiwi:1111:1.1.11:good:Jun<E>\n",
                "<S>Orange:222:1.1.2:good:Jan<E>\n",
            )
        );
        assert_eq!(oup.status.success(), true);
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
