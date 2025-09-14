const TARGET_EXE_PATH: &str = env!(concat!("CARGO_BIN_EXE_", env!("CARGO_PKG_NAME")));

#[macro_use]
mod helper;

#[macro_use]
mod helper_e;

mod test_0_e {
    use exec_target::exec_target;
    const TARGET_EXE_PATH: &str = super::TARGET_EXE_PATH;
    //
    #[test]
    fn test_help() {
        let oup = exec_target(TARGET_EXE_PATH, ["-H"]);
        assert_eq!(oup.stderr, "");
        assert_eq!(oup.stdout, help_msg!());
        assert!(oup.status.success());
    }
    #[test]
    fn test_help_long() {
        let oup = exec_target(TARGET_EXE_PATH, ["--help"]);
        assert_eq!(oup.stderr, "");
        assert_eq!(oup.stdout, help_msg!());
        assert!(oup.status.success());
    }
    #[test]
    fn test_version() {
        let oup = exec_target(TARGET_EXE_PATH, ["-V"]);
        assert_eq!(oup.stderr, "");
        assert_eq!(oup.stdout, version_msg!());
        assert!(oup.status.success());
    }
    #[test]
    fn test_version_long() {
        let oup = exec_target(TARGET_EXE_PATH, ["--version"]);
        assert_eq!(oup.stderr, "");
        assert_eq!(oup.stdout, version_msg!());
        assert!(oup.status.success());
    }
    #[test]
    fn test_invalid_opt() {
        let oup = exec_target(TARGET_EXE_PATH, ["-z"]);
        assert_eq!(
            oup.stderr,
            concat!(
                program_name!(),
                ": ",
                "Invalid option: z\n",
                try_help_msg!()
            )
        );
        assert_eq!(oup.stdout, "");
        assert!(!oup.status.success());
    }
}

mod test_0_x_options_e {
    use exec_target::exec_target;
    const TARGET_EXE_PATH: &str = super::TARGET_EXE_PATH;
    //
    #[test]
    fn test_x_option_help() {
        let oup = exec_target(TARGET_EXE_PATH, ["-X", "help"]);
        assert_eq!(oup.stderr, "");
        assert_eq!(oup.stdout, x_help_msg!());
        assert!(oup.status.success());
    }
    //
    #[test]
    fn test_x_option_rust_version_info() {
        let oup = exec_target(TARGET_EXE_PATH, ["-X", "rust-version-info"]);
        assert_eq!(oup.stderr, "");
        assert!(oup.stdout.contains("rustc"));
        assert!(oup.status.success());
    }
    //
    #[test]
    fn test_multiple_x_options() {
        let oup = exec_target(TARGET_EXE_PATH, ["-X", "help", "-X", "rust-version-info"]);
        assert_eq!(oup.stderr, "");
        // The first one should be executed and the program should exit.
        assert!(oup.stdout.contains("Options:"));
        assert!(!oup.stdout.contains("rustc"));
        assert!(oup.status.success());
    }
}

mod test_1_e {
    use exec_target::exec_target_with_in;
    const TARGET_EXE_PATH: &str = super::TARGET_EXE_PATH;
    //
    #[test]
    fn test_invalid_utf8() {
        let v = std::fs::read(fixture_invalid_utf8!()).unwrap();
        let oup = exec_target_with_in(TARGET_EXE_PATH, &[] as &[&str], &v);
        assert_eq!(
            oup.stderr,
            concat!(program_name!(), ": stream did not contain valid UTF-8\n",)
        );
        assert_eq!(oup.stdout, "");
        assert!(!oup.status.success());
    }
    //
    #[test]
    fn test_empty_input() {
        let in_w = "";
        let oup = exec_target_with_in(TARGET_EXE_PATH, &[] as &[&str], in_w.as_bytes());
        assert_eq!(oup.stderr, "");
        assert_eq!(oup.stdout, "");
        assert!(oup.status.success());
    }
    //
    #[test]
    fn test_input_with_empty_lines() {
        let in_w = "b\n\na\n\nc\n";
        let oup = exec_target_with_in(TARGET_EXE_PATH, &[] as &[&str], in_w.as_bytes());
        assert_eq!(oup.stderr, "");
        assert_eq!(oup.stdout, "\n\na\nb\nc\n");
        assert!(oup.status.success());
    }
}

mod test_1_regex_options_e {
    use exec_target::exec_target_with_in;
    const TARGET_EXE_PATH: &str = super::TARGET_EXE_PATH;
    //
    #[test]
    fn test_regex_no_match() {
        let in_w = "b:1\na:2\nc:3\n";
        let oup = exec_target_with_in(TARGET_EXE_PATH, ["-e", "d:."], in_w.as_bytes());
        assert_eq!(oup.stderr, "");
        assert_eq!(oup.stdout, "b:1\na:2\nc:3\n");
        assert!(oup.status.success());
    }
    //
    #[test]
    fn test_regex_full_match() {
        let in_w = "b:1\na:2\nc:3\n";
        let oup = exec_target_with_in(TARGET_EXE_PATH, ["-e", ".*"], in_w.as_bytes());
        assert_eq!(oup.stderr, "");
        assert_eq!(oup.stdout, "a:2\nb:1\nc:3\n");
        assert!(oup.status.success());
    }
    //
    #[test]
    fn test_regex_capture_group() {
        let in_w = "b:1\na:2\nc:3\n";
        let oup = exec_target_with_in(TARGET_EXE_PATH, ["-e", ":(.)"], in_w.as_bytes());
        assert_eq!(oup.stderr, "");
        assert_eq!(oup.stdout, "b:1\na:2\nc:3\n");
        assert!(oup.status.success());
    }
    //
    #[test]
    fn test_regex_capture_group_no_match() {
        let in_w = "b:1\na:2\nc:3\n";
        let oup = exec_target_with_in(TARGET_EXE_PATH, ["-e", "d(.)"], in_w.as_bytes());
        assert_eq!(oup.stderr, "");
        assert_eq!(oup.stdout, "b:1\na:2\nc:3\n");
        assert!(oup.status.success());
    }
}

mod test_2_string_e {
    use exec_target::exec_target_with_in;
    const TARGET_EXE_PATH: &str = super::TARGET_EXE_PATH;
    //
    #[test]
    fn test_t1() {
        let in_w = std::fs::read_to_string(fixture_fruit!()).unwrap();
        let oup = exec_target_with_in(TARGET_EXE_PATH, &[] as &[&str], in_w.as_bytes());
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
        assert!(oup.status.success());
    }
    //
    #[test]
    fn test_t2() {
        let in_w = std::fs::read_to_string(fixture_fruit!()).unwrap();
        let oup = exec_target_with_in(TARGET_EXE_PATH, ["-r"], in_w.as_bytes());
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
        assert!(oup.status.success());
    }
    //
    #[test]
    fn test_t3() {
        let in_w = std::fs::read_to_string(fixture_fruit!()).unwrap();
        let oup = exec_target_with_in(TARGET_EXE_PATH, ["-e", "[0-9]+"], in_w.as_bytes());
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
        assert!(oup.status.success());
    }
    //
    #[test]
    fn test_t4() {
        let in_w = std::fs::read_to_string(fixture_fruit!()).unwrap();
        let oup = exec_target_with_in(TARGET_EXE_PATH, ["-e", "[0-9]+", "-r"], in_w.as_bytes());
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
        assert!(oup.status.success());
    }
    //
    #[test]
    fn test_t5() {
        let in_w = std::fs::read_to_string(fixture_fruit!()).unwrap();
        let in_w = in_w.clone() + &in_w;
        let oup = exec_target_with_in(TARGET_EXE_PATH, ["-e", "[0-9]+"], in_w.as_bytes());
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
        assert!(oup.status.success());
    }
    //
    #[test]
    fn test_t6_header() {
        let in_w = std::fs::read_to_string(fixture_fruit_header!()).unwrap();
        let oup = exec_target_with_in(
            TARGET_EXE_PATH,
            ["-e", "[0-9]+", "-h", "1"],
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
        assert!(oup.status.success());
    }
    //
    #[test]
    fn test_t6_footer() {
        let in_w = std::fs::read_to_string(fixture_fruit_footer!()).unwrap();
        let oup = exec_target_with_in(
            TARGET_EXE_PATH,
            ["-e", "[0-9]+", "-t", "1"],
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
        assert!(oup.status.success());
    }
}

mod test_2_string_color_e {
    use exec_target::exec_target_with_env_in;
    const TARGET_EXE_PATH: &str = super::TARGET_EXE_PATH;
    //
    #[test]
    fn test_t1() {
        let in_w = std::fs::read_to_string(fixture_fruit!()).unwrap();
        let oup = exec_target_with_env_in(
            TARGET_EXE_PATH,
            ["--color", "always"],
            env_1!(),
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
        assert!(oup.status.success());
    }
    //
    #[test]
    fn test_t2() {
        let in_w = std::fs::read_to_string(fixture_fruit!()).unwrap();
        let oup = exec_target_with_env_in(
            TARGET_EXE_PATH,
            ["-r", "--color", "always"],
            env_1!(),
            in_w.as_bytes(),
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
        assert!(oup.status.success());
    }
    //
    #[test]
    fn test_t3() {
        let in_w = std::fs::read_to_string(fixture_fruit!()).unwrap();
        let oup = exec_target_with_env_in(
            TARGET_EXE_PATH,
            ["-e", "[0-9]+", "--color", "always"],
            env_1!(),
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
            )
        );
        assert!(oup.status.success());
    }
    //
    #[test]
    fn test_t4() {
        let in_w = std::fs::read_to_string(fixture_fruit!()).unwrap();
        let oup = exec_target_with_env_in(
            TARGET_EXE_PATH,
            ["-e", "[0-9]+", "-r", "--color", "always"],
            env_1!(),
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
            )
        );
        assert!(oup.status.success());
    }
    //
    #[test]
    fn test_t5() {
        let in_w = std::fs::read_to_string(fixture_fruit!()).unwrap();
        let in_w = in_w.clone() + &in_w;
        let oup = exec_target_with_env_in(
            TARGET_EXE_PATH,
            ["-e", "[0-9]+", "--color", "always"],
            env_1!(),
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
        assert!(oup.status.success());
    }
    //
    #[test]
    fn test_t6_header() {
        let in_w = std::fs::read_to_string(fixture_fruit_header!()).unwrap();
        let oup = exec_target_with_env_in(
            TARGET_EXE_PATH,
            ["-e", "[0-9]+", "-h", "1", "--color", "always"],
            env_1!(),
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
        assert!(oup.status.success());
    }
    //
    #[test]
    fn test_t6_footer() {
        let in_w = std::fs::read_to_string(fixture_fruit_footer!()).unwrap();
        let oup = exec_target_with_env_in(
            TARGET_EXE_PATH,
            ["-e", "[0-9]+", "-t", "1", "--color", "always"],
            env_1!(),
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
        assert!(oup.status.success());
    }
}

mod test_2_numeric_e {
    use exec_target::exec_target_with_in;
    const TARGET_EXE_PATH: &str = super::TARGET_EXE_PATH;
    //
    #[test]
    fn test_t1() {
        let in_w = std::fs::read_to_string(fixture_fruit!()).unwrap();
        let oup = exec_target_with_in(
            TARGET_EXE_PATH,
            ["--according-to", "numeric"],
            in_w.as_bytes(),
        );
        assert_eq!(
            oup.stderr,
            concat!(
                program_name!(),
                ": (0,21):\'Apple:33:3.3:good:Mar\': invalid digit found in string\n"
            )
        );
        assert_eq!(oup.stdout, "");
        assert!(!oup.status.success());
    }
    //
    #[test]
    fn test_t2() {
        let in_w = std::fs::read_to_string(fixture_fruit!()).unwrap();
        let oup = exec_target_with_in(
            TARGET_EXE_PATH,
            ["--according-to", "numeric", "-r"],
            in_w.as_bytes(),
        );
        assert_eq!(
            oup.stderr,
            concat!(
                program_name!(),
                ": (0,21):\'Apple:33:3.3:good:Mar\': invalid digit found in string\n"
            )
        );
        assert_eq!(oup.stdout, "");
        assert!(!oup.status.success());
    }
    //
    #[test]
    fn test_t3() {
        let in_w = std::fs::read_to_string(fixture_fruit!()).unwrap();
        let oup = exec_target_with_in(
            TARGET_EXE_PATH,
            ["-e", "[0-9]+", "--according-to", "numeric"],
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
            )
        );
        assert!(oup.status.success());
    }
    //
    #[test]
    fn test_t4() {
        let in_w = std::fs::read_to_string(fixture_fruit!()).unwrap();
        let oup = exec_target_with_in(
            TARGET_EXE_PATH,
            ["-e", "[0-9]+", "--according-to", "numeric", "-r"],
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
            )
        );
        assert!(oup.status.success());
    }
    //
    #[test]
    fn test_t5() {
        let in_w = std::fs::read_to_string(fixture_fruit!()).unwrap();
        let in_w = in_w.clone() + &in_w;
        let oup = exec_target_with_in(
            TARGET_EXE_PATH,
            ["-e", "[0-9]+", "--according-to", "numeric"],
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
        assert!(oup.status.success());
    }
    //
    #[test]
    fn test_t6_header() {
        let in_w = std::fs::read_to_string(fixture_fruit_header!()).unwrap();
        let oup = exec_target_with_in(
            TARGET_EXE_PATH,
            ["-e", "[0-9]+", "--according-to", "numeric", "-h", "1"],
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
        assert!(oup.status.success());
    }
    //
    #[test]
    fn test_t6_footer() {
        let in_w = std::fs::read_to_string(fixture_fruit_footer!()).unwrap();
        let oup = exec_target_with_in(
            TARGET_EXE_PATH,
            ["-e", "[0-9]+", "--according-to", "numeric", "-t", "1"],
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
        assert!(oup.status.success());
    }
}

mod test_2_numeric_color_e {
    use exec_target::exec_target_with_env_in;
    const TARGET_EXE_PATH: &str = super::TARGET_EXE_PATH;
    //
    #[test]
    fn test_t1() {
        let in_w = std::fs::read_to_string(fixture_fruit!()).unwrap();
        let oup = exec_target_with_env_in(
            TARGET_EXE_PATH,
            ["--according-to", "numeric", "--color", "always"],
            env_1!(),
            in_w.as_bytes(),
        );
        assert_eq!(
            oup.stderr,
            concat!(
                program_name!(),
                ": (0,21):\'Apple:33:3.3:good:Mar\': invalid digit found in string\n"
            )
        );
        assert_eq!(oup.stdout, "");
        assert!(!oup.status.success());
    }
    //
    #[test]
    fn test_t2() {
        let in_w = std::fs::read_to_string(fixture_fruit!()).unwrap();
        let oup = exec_target_with_env_in(
            TARGET_EXE_PATH,
            ["--according-to", "numeric", "-r", "--color", "always"],
            env_1!(),
            in_w.as_bytes(),
        );
        assert_eq!(
            oup.stderr,
            concat!(
                program_name!(),
                ": (0,21):\'Apple:33:3.3:good:Mar\': invalid digit found in string\n"
            )
        );
        assert_eq!(oup.stdout, "");
        assert!(!oup.status.success());
    }
    //
    #[test]
    fn test_t3() {
        let in_w = std::fs::read_to_string(fixture_fruit!()).unwrap();
        let oup = exec_target_with_env_in(
            TARGET_EXE_PATH,
            [
                "-e",
                "[0-9]+",
                "--according-to",
                "numeric",
                "--color",
                "always",
            ],
            env_1!(),
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
            )
        );
        assert!(oup.status.success());
    }
    //
    #[test]
    fn test_t4() {
        let in_w = std::fs::read_to_string(fixture_fruit!()).unwrap();
        let oup = exec_target_with_env_in(
            TARGET_EXE_PATH,
            [
                "-e",
                "[0-9]+",
                "--according-to",
                "numeric",
                "-r",
                "--color",
                "always",
            ],
            env_1!(),
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
            )
        );
        assert!(oup.status.success());
    }
    //
    #[test]
    fn test_t5() {
        let in_w = std::fs::read_to_string(fixture_fruit!()).unwrap();
        let in_w = in_w.clone() + &in_w;
        let oup = exec_target_with_env_in(
            TARGET_EXE_PATH,
            [
                "-e",
                "[0-9]+",
                "--according-to",
                "numeric",
                "--color",
                "always",
            ],
            env_1!(),
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
        assert!(oup.status.success());
    }
    //
    #[test]
    fn test_t6_header() {
        let in_w = std::fs::read_to_string(fixture_fruit_header!()).unwrap();
        let oup = exec_target_with_env_in(
            TARGET_EXE_PATH,
            [
                "-e",
                "[0-9]+",
                "--according-to",
                "numeric",
                "-h",
                "1",
                "--color",
                "always",
            ],
            env_1!(),
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
        assert!(oup.status.success());
    }
    //
    #[test]
    fn test_t6_footer() {
        let in_w = std::fs::read_to_string(fixture_fruit_footer!()).unwrap();
        let oup = exec_target_with_env_in(
            TARGET_EXE_PATH,
            [
                "-e",
                "[0-9]+",
                "--according-to",
                "numeric",
                "-t",
                "1",
                "--color",
                "always",
            ],
            env_1!(),
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
        assert!(oup.status.success());
    }
}

mod test_2_version_e {
    use exec_target::exec_target_with_in;
    const TARGET_EXE_PATH: &str = super::TARGET_EXE_PATH;
    //
    #[test]
    fn test_t1() {
        let in_w = std::fs::read_to_string(fixture_fruit!()).unwrap();
        let oup = exec_target_with_in(
            TARGET_EXE_PATH,
            ["--according-to", "version"],
            in_w.as_bytes(),
        );
        assert_eq!(
            oup.stderr,
            concat!(
                program_name!(),
                ": (0,21):\'Apple:33:3.3:good:Mar\': unexpected character \'A\' while parsing major version number\n",
            )
        );
        assert_eq!(oup.stdout, "");
        assert!(!oup.status.success());
    }
    //
    #[test]
    fn test_t2() {
        let in_w = std::fs::read_to_string(fixture_fruit!()).unwrap();
        let oup = exec_target_with_in(
            TARGET_EXE_PATH,
            ["--according-to", "version", "-r"],
            in_w.as_bytes(),
        );
        assert_eq!(
            oup.stderr,
            concat!(
                program_name!(),
                ": (0,21):\'Apple:33:3.3:good:Mar\': unexpected character \'A\' while parsing major version number\n",
            )
        );
        assert_eq!(oup.stdout, "");
        assert!(!oup.status.success());
    }
    //
    #[test]
    fn test_t3() {
        let in_w = std::fs::read_to_string(fixture_fruit!()).unwrap();
        let oup = exec_target_with_in(
            TARGET_EXE_PATH,
            ["-e", "[^:]+:[^:]+:([0-9.]+):", "--according-to", "version"],
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
            )
        );
        assert!(oup.status.success());
    }
    //
    #[test]
    fn test_t4() {
        let in_w = std::fs::read_to_string(fixture_fruit!()).unwrap();
        let oup = exec_target_with_in(
            TARGET_EXE_PATH,
            [
                "-e",
                "[^:]+:[^:]+:([0-9.]+):",
                "--according-to",
                "version",
                "-r",
            ],
            in_w.as_bytes(),
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
        assert!(oup.status.success());
    }
    //
    #[test]
    fn test_t5() {
        let in_w = std::fs::read_to_string(fixture_fruit!()).unwrap();
        let in_w = in_w.clone() + &in_w;
        let oup = exec_target_with_in(
            TARGET_EXE_PATH,
            ["-e", "[^:]+:[^:]+:([0-9.]+):", "--according-to", "version"],
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
        assert!(oup.status.success());
    }
    //
    #[test]
    fn test_t6_header() {
        let in_w = std::fs::read_to_string(fixture_fruit_header!()).unwrap();
        let oup = exec_target_with_in(
            TARGET_EXE_PATH,
            [
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
        assert!(oup.status.success());
    }
    //
    #[test]
    fn test_t6_footer() {
        let in_w = std::fs::read_to_string(fixture_fruit_footer!()).unwrap();
        let oup = exec_target_with_in(
            TARGET_EXE_PATH,
            [
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
        assert!(oup.status.success());
    }
}

mod test_2_version_color_e {
    use exec_target::exec_target_with_env_in;
    const TARGET_EXE_PATH: &str = super::TARGET_EXE_PATH;
    //
    #[test]
    fn test_t1() {
        let in_w = std::fs::read_to_string(fixture_fruit!()).unwrap();
        let oup = exec_target_with_env_in(
            TARGET_EXE_PATH,
            ["--according-to", "version", "--color", "always"],
            env_1!(),
            in_w.as_bytes(),
        );
        assert_eq!(
            oup.stderr,
            concat!(
                program_name!(),
                ": (0,21):\'Apple:33:3.3:good:Mar\': unexpected character \'A\' while parsing major version number\n",
            )
        );
        assert_eq!(oup.stdout, "");
        assert!(!oup.status.success());
    }
    //
    #[test]
    fn test_t2() {
        let in_w = std::fs::read_to_string(fixture_fruit!()).unwrap();
        let oup = exec_target_with_env_in(
            TARGET_EXE_PATH,
            ["--according-to", "version", "-r", "--color", "always"],
            env_1!(),
            in_w.as_bytes(),
        );
        assert_eq!(
            oup.stderr,
            concat!(
                program_name!(),
                ": (0,21):\'Apple:33:3.3:good:Mar\': unexpected character \'A\' while parsing major version number\n",
            )
        );
        assert_eq!(oup.stdout, "");
        assert!(!oup.status.success());
    }
    //
    #[test]
    fn test_t3() {
        let in_w = std::fs::read_to_string(fixture_fruit!()).unwrap();
        let oup = exec_target_with_env_in(
            TARGET_EXE_PATH,
            [
                "-e",
                "[^:]+:[^:]+:([0-9.]+):",
                "--according-to",
                "version",
                "--color",
                "always",
            ],
            env_1!(),
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
            )
        );
        assert!(oup.status.success());
    }
    //
    #[test]
    fn test_t4() {
        let in_w = std::fs::read_to_string(fixture_fruit!()).unwrap();
        let oup = exec_target_with_env_in(
            TARGET_EXE_PATH,
            [
                "-e",
                "[^:]+:[^:]+:([0-9.]+):",
                "--according-to",
                "version",
                "-r",
                "--color",
                "always",
            ],
            env_1!(),
            in_w.as_bytes(),
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
        assert!(oup.status.success());
    }
    //
    #[test]
    fn test_t5() {
        let in_w = std::fs::read_to_string(fixture_fruit!()).unwrap();
        let in_w = in_w.clone() + &in_w;
        let oup = exec_target_with_env_in(
            TARGET_EXE_PATH,
            [
                "-e",
                "[^:]+:[^:]+:([0-9.]+):",
                "--according-to",
                "version",
                "--color",
                "always",
            ],
            env_1!(),
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
        assert!(oup.status.success());
    }
    //
    #[test]
    fn test_t6_header() {
        let in_w = std::fs::read_to_string(fixture_fruit_header!()).unwrap();
        let oup = exec_target_with_env_in(
            TARGET_EXE_PATH,
            [
                "-e",
                "[^:]+:[^:]+:([0-9.]+):",
                "--according-to",
                "version",
                "-h",
                "1",
                "--color",
                "always",
            ],
            env_1!(),
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
        assert!(oup.status.success());
    }
    //
    #[test]
    fn test_t6_footer() {
        let in_w = std::fs::read_to_string(fixture_fruit_footer!()).unwrap();
        let oup = exec_target_with_env_in(
            TARGET_EXE_PATH,
            [
                "-e",
                "[^:]+:[^:]+:([0-9.]+):",
                "--according-to",
                "version",
                "-t",
                "1",
                "--color",
                "always",
            ],
            env_1!(),
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
        assert!(oup.status.success());
    }
}

mod test_2_month_e {
    use exec_target::exec_target_with_in;
    const TARGET_EXE_PATH: &str = super::TARGET_EXE_PATH;
    //
    #[test]
    fn test_t1() {
        let in_w = std::fs::read_to_string(fixture_fruit!()).unwrap();
        let oup = exec_target_with_in(
            TARGET_EXE_PATH,
            ["--according-to", "month"],
            in_w.as_bytes(),
        );
        assert_eq!(
            oup.stderr,
            concat!(
                program_name!(),
                ": (0,21):\'Apple:33:3.3:good:Mar\': invalid month strings\n",
            )
        );
        assert_eq!(oup.stdout, "");
        assert!(!oup.status.success());
    }
    //
    #[test]
    fn test_t2() {
        let in_w = std::fs::read_to_string(fixture_fruit!()).unwrap();
        let oup = exec_target_with_in(
            TARGET_EXE_PATH,
            ["--according-to", "month", "-r"],
            in_w.as_bytes(),
        );
        assert_eq!(
            oup.stderr,
            concat!(
                program_name!(),
                ": (0,21):\'Apple:33:3.3:good:Mar\': invalid month strings\n",
            )
        );
        assert_eq!(oup.stdout, "");
        assert!(!oup.status.success());
    }
    //
    #[test]
    fn test_t3() {
        let in_w = std::fs::read_to_string(fixture_fruit!()).unwrap();
        let oup = exec_target_with_in(
            TARGET_EXE_PATH,
            ["-e", ":([^:]+)$", "--according-to", "month"],
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
            )
        );
        assert!(oup.status.success());
    }
    //
    #[test]
    fn test_t4() {
        let in_w = std::fs::read_to_string(fixture_fruit!()).unwrap();
        let oup = exec_target_with_in(
            TARGET_EXE_PATH,
            ["-e", ":([^:]+)$", "--according-to", "month", "-r"],
            in_w.as_bytes(),
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
        assert!(oup.status.success());
    }
    //
    #[test]
    fn test_t5() {
        let in_w = std::fs::read_to_string(fixture_fruit!()).unwrap();
        let in_w = in_w.clone() + &in_w;
        let oup = exec_target_with_in(
            TARGET_EXE_PATH,
            ["-e", ":([^:]+)$", "--according-to", "month"],
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
        assert!(oup.status.success());
    }
    //
    #[test]
    fn test_t6_header() {
        let in_w = std::fs::read_to_string(fixture_fruit_header!()).unwrap();
        let oup = exec_target_with_in(
            TARGET_EXE_PATH,
            ["-e", ":([^:]+)$", "--according-to", "month", "-h", "1"],
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
        assert!(oup.status.success());
    }
    //
    #[test]
    fn test_t6_footer() {
        let in_w = std::fs::read_to_string(fixture_fruit_footer!()).unwrap();
        let oup = exec_target_with_in(
            TARGET_EXE_PATH,
            ["-e", ":([^:]+)$", "--according-to", "month", "-t", "1"],
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
        assert!(oup.status.success());
    }
}

mod test_2_month_color_e {
    use exec_target::exec_target_with_env_in;
    const TARGET_EXE_PATH: &str = super::TARGET_EXE_PATH;
    //
    #[test]
    fn test_t1() {
        let in_w = std::fs::read_to_string(fixture_fruit!()).unwrap();
        let oup = exec_target_with_env_in(
            TARGET_EXE_PATH,
            ["--according-to", "month", "--color", "always"],
            env_1!(),
            in_w.as_bytes(),
        );
        assert_eq!(
            oup.stderr,
            concat!(
                program_name!(),
                ": (0,21):\'Apple:33:3.3:good:Mar\': invalid month strings\n",
            )
        );
        assert_eq!(oup.stdout, "");
        assert!(!oup.status.success());
    }
    //
    #[test]
    fn test_t2() {
        let in_w = std::fs::read_to_string(fixture_fruit!()).unwrap();
        let oup = exec_target_with_env_in(
            TARGET_EXE_PATH,
            ["--according-to", "month", "-r", "--color", "always"],
            env_1!(),
            in_w.as_bytes(),
        );
        assert_eq!(
            oup.stderr,
            concat!(
                program_name!(),
                ": (0,21):\'Apple:33:3.3:good:Mar\': invalid month strings\n",
            )
        );
        assert_eq!(oup.stdout, "");
        assert!(!oup.status.success());
    }
    //
    #[test]
    fn test_t3() {
        let in_w = std::fs::read_to_string(fixture_fruit!()).unwrap();
        let oup = exec_target_with_env_in(
            TARGET_EXE_PATH,
            [
                "-e",
                ":([^:]+)$",
                "--according-to",
                "month",
                "--color",
                "always",
            ],
            env_1!(),
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
            )
        );
        assert!(oup.status.success());
    }
    //
    #[test]
    fn test_t4() {
        let in_w = std::fs::read_to_string(fixture_fruit!()).unwrap();
        let oup = exec_target_with_env_in(
            TARGET_EXE_PATH,
            [
                "-e",
                ":([^:]+)$",
                "--according-to",
                "month",
                "-r",
                "--color",
                "always",
            ],
            env_1!(),
            in_w.as_bytes(),
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
        assert!(oup.status.success());
    }
    //
    #[test]
    fn test_t5() {
        let in_w = std::fs::read_to_string(fixture_fruit!()).unwrap();
        let in_w = in_w.clone() + &in_w;
        let oup = exec_target_with_env_in(
            TARGET_EXE_PATH,
            [
                "-e",
                ":([^:]+)$",
                "--according-to",
                "month",
                "--color",
                "always",
            ],
            env_1!(),
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
        assert!(oup.status.success());
    }
    //
    #[test]
    fn test_t6_header() {
        let in_w = std::fs::read_to_string(fixture_fruit_header!()).unwrap();
        let oup = exec_target_with_env_in(
            TARGET_EXE_PATH,
            [
                "-e",
                ":([^:]+)$",
                "--according-to",
                "month",
                "-h",
                "1",
                "--color",
                "always",
            ],
            env_1!(),
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
        assert!(oup.status.success());
    }
    //
    #[test]
    fn test_t6_footer() {
        let in_w = std::fs::read_to_string(fixture_fruit_footer!()).unwrap();
        let oup = exec_target_with_env_in(
            TARGET_EXE_PATH,
            [
                "-e",
                ":([^:]+)$",
                "--according-to",
                "month",
                "-t",
                "1",
                "--color",
                "always",
            ],
            env_1!(),
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
        assert!(oup.status.success());
    }
}

mod test_2_time_e {
    use exec_target::exec_target_with_in;
    const TARGET_EXE_PATH: &str = super::TARGET_EXE_PATH;
    //
    #[test]
    fn test_t1() {
        let in_w = std::fs::read_to_string(fixture_fruit!()).unwrap();
        let oup = exec_target_with_in(TARGET_EXE_PATH, ["--according-to", "time"], in_w.as_bytes());
        assert_eq!(
            oup.stderr,
            concat!(
                program_name!(),
                ": (0,21):\'Apple:33:3.3:good:Mar\': unexpected character \'A\' while parsing time\n",
            )
        );
        assert_eq!(oup.stdout, "");
        assert!(!oup.status.success());
    }
    //
    #[test]
    fn test_t2() {
        let in_w = std::fs::read_to_string(fixture_fruit!()).unwrap();
        let oup = exec_target_with_in(
            TARGET_EXE_PATH,
            ["--according-to", "time", "-r"],
            in_w.as_bytes(),
        );
        assert_eq!(
            oup.stderr,
            concat!(
                program_name!(),
                ": (0,21):\'Apple:33:3.3:good:Mar\': unexpected character \'A\' while parsing time\n",
            )
        );
        assert_eq!(oup.stdout, "");
        assert!(!oup.status.success());
    }
    //
    #[test]
    fn test_t3() {
        let in_w = std::fs::read_to_string(fixture_fruit!()).unwrap();
        let oup = exec_target_with_in(
            TARGET_EXE_PATH,
            [
                "-e",
                "([0-9]+:([0-9]+:)?[0-9]+(.[0-9]+)?)",
                "--according-to",
                "time",
            ],
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
            )
        );
        assert!(oup.status.success());
    }
    //
    #[test]
    fn test_t4() {
        let in_w = std::fs::read_to_string(fixture_fruit!()).unwrap();
        let oup = exec_target_with_in(
            TARGET_EXE_PATH,
            [
                "-e",
                "([0-9]+:([0-9]+:)?[0-9]+(.[0-9]+)?)",
                "--according-to",
                "time",
                "-r",
            ],
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
            )
        );
        assert!(oup.status.success());
    }
    //
    #[test]
    fn test_t5() {
        let in_w = std::fs::read_to_string(fixture_fruit!()).unwrap();
        let in_w = in_w.clone() + &in_w;
        let oup = exec_target_with_in(
            TARGET_EXE_PATH,
            [
                "-e",
                "([0-9]+:([0-9]+:)?[0-9]+(.[0-9]+)?)",
                "--according-to",
                "time",
            ],
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
        assert!(oup.status.success());
    }
    //
    #[test]
    fn test_t6_header() {
        let in_w = std::fs::read_to_string(fixture_fruit_header!()).unwrap();
        let oup = exec_target_with_in(
            TARGET_EXE_PATH,
            [
                "-e",
                "([0-9]+:([0-9]+:)?[0-9]+(.[0-9]+)?)",
                "--according-to",
                "time",
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
                "Cherry:4:4:good:Oct\n",
                "Apple:33:3.3:good:Mar\n",
                "Orange:222:1.1.2:good:Jan\n",
                "Kiwi:1111:1.1.11:good:Jun\n",
            )
        );
        assert!(oup.status.success());
    }
    //
    #[test]
    fn test_t6_footer() {
        let in_w = std::fs::read_to_string(fixture_fruit_footer!()).unwrap();
        let oup = exec_target_with_in(
            TARGET_EXE_PATH,
            [
                "-e",
                "([0-9]+:([0-9]+:)?[0-9]+(.[0-9]+)?)",
                "--according-to",
                "time",
                "-t",
                "1",
            ],
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
        assert!(oup.status.success());
    }
    //
    #[test]
    fn test_t7() {
        let in_w = std::fs::read_to_string(fixture_time!()).unwrap();
        let oup = exec_target_with_in(
            TARGET_EXE_PATH,
            [
                "-e",
                "([0-9]+:([0-9]+:)?[0-9]+(.[0-9]+)?)",
                "--according-to",
                "time",
            ],
            in_w.as_bytes(),
        );
        assert_eq!(oup.stderr, "");
        assert_eq!(
            oup.stdout,
            concat!(
                "bench-c abyssiniandb \t67.91user 4.58system 1:21.45elapsed 89%CPU (655864maxresident)k\n",
                "bench-c berkeleydb_hs \t86.01user 6.75system 2:08.07elapsed 72%CPU (2219324maxresident)k\n",
                "bench-c tokyocabinet_b \t79.77user 7.38system 2:24.25elapsed 60%CPU (2493500maxresident)k\n",
                "bench-c berkeleydb_bt \t84.08user 7.09system 2:24.57elapsed 63%CPU (3061200maxresident)k\n",
                "bench-c siamesedb \t153.58user 5.23system 2:46.64elapsed 95%CPU (1003164maxresident)k\n",
                "bench-c tokyocabinet_h \t82.63user 9.64system 2:47.39elapsed 55%CPU (2493452maxresident)k\n",
                "bench-c pickledb \t57.26user 15.36system 2:53.24elapsed 41%CPU (5610668maxresident)k\n",
                "bench-c leveldb \t183.32user 132.09system 11:51.47elapsed 44%CPU (86456maxresident)k\n",
                "bench-c qdbm \t551.57user 145.02system 12:09.98elapsed 95%CPU (30696maxresident)k\n",
                "bench-c sled \t762.92user 51.03system 12:48.55elapsed 105%CPU (3209012maxresident)k\n",
                "bench-c gdbm \t166.57user 276.04system 19:32.18elapsed 37%CPU (680304maxresident)k\n",
                "bench-c kyotocabinet \t743.80user 290.03system 25:45.48elapsed 66%CPU (762084maxresident)k\n",
                "bench-c sqlite \t191.97user 745.43system 2:01:53elapsed 12%CPU (8788maxresident)k\n"
            )
        );
        assert!(oup.status.success());
    }
}

mod test_2_time_color_e {
    use exec_target::exec_target_with_env_in;
    const TARGET_EXE_PATH: &str = super::TARGET_EXE_PATH;
    //
    #[test]
    fn test_t1() {
        let in_w = std::fs::read_to_string(fixture_fruit!()).unwrap();
        let oup = exec_target_with_env_in(
            TARGET_EXE_PATH,
            ["--according-to", "time", "--color", "always"],
            env_1!(),
            in_w.as_bytes(),
        );
        assert_eq!(
            oup.stderr,
            concat!(
                program_name!(),
                ": (0,21):\'Apple:33:3.3:good:Mar\': unexpected character \'A\' while parsing time\n",
            )
        );
        assert_eq!(oup.stdout, "");
        assert!(!oup.status.success());
    }
    //
    #[test]
    fn test_t2() {
        let in_w = std::fs::read_to_string(fixture_fruit!()).unwrap();
        let oup = exec_target_with_env_in(
            TARGET_EXE_PATH,
            ["--according-to", "time", "-r", "--color", "always"],
            env_1!(),
            in_w.as_bytes(),
        );
        assert_eq!(
            oup.stderr,
            concat!(
                program_name!(),
                ": (0,21):\'Apple:33:3.3:good:Mar\': unexpected character \'A\' while parsing time\n",
            )
        );
        assert_eq!(oup.stdout, "");
        assert!(!oup.status.success());
    }
    //
    #[test]
    fn test_t3() {
        let in_w = std::fs::read_to_string(fixture_fruit!()).unwrap();
        let oup = exec_target_with_env_in(
            TARGET_EXE_PATH,
            [
                "-e",
                "([0-9]+:([0-9]+:)?[0-9]+(.[0-9]+)?)",
                "--according-to",
                "time",
                "--color",
                "always",
            ],
            env_1!(),
            in_w.as_bytes(),
        );
        assert_eq!(oup.stderr, "");
        assert_eq!(
            oup.stdout,
            concat!(
                "Cherry:<S>4:4<E>:good:Oct\n",
                "Apple:<S>33:3.3<E>:good:Mar\n",
                "Orange:<S>222:1.1<E>.2:good:Jan\n",
                "Kiwi:<S>1111:1.1<E>.11:good:Jun\n",
            )
        );
        assert!(oup.status.success());
    }
    //
    #[test]
    fn test_t4() {
        let in_w = std::fs::read_to_string(fixture_fruit!()).unwrap();
        let oup = exec_target_with_env_in(
            TARGET_EXE_PATH,
            [
                "-e",
                "([0-9]+:([0-9]+:)?[0-9]+(.[0-9]+)?)",
                "--according-to",
                "time",
                "-r",
                "--color",
                "always",
            ],
            env_1!(),
            in_w.as_bytes(),
        );
        assert_eq!(oup.stderr, "");
        assert_eq!(
            oup.stdout,
            concat!(
                "Kiwi:<S>1111:1.1<E>.11:good:Jun\n",
                "Orange:<S>222:1.1<E>.2:good:Jan\n",
                "Apple:<S>33:3.3<E>:good:Mar\n",
                "Cherry:<S>4:4<E>:good:Oct\n",
            )
        );
        assert!(oup.status.success());
    }
    //
    #[test]
    fn test_t5() {
        let in_w = std::fs::read_to_string(fixture_fruit!()).unwrap();
        let in_w = in_w.clone() + &in_w;
        let oup = exec_target_with_env_in(
            TARGET_EXE_PATH,
            [
                "-e",
                "([0-9]+:([0-9]+:)?[0-9]+(.[0-9]+)?)",
                "--according-to",
                "time",
                "--color",
                "always",
            ],
            env_1!(),
            in_w.as_bytes(),
        );
        assert_eq!(oup.stderr, "");
        assert_eq!(
            oup.stdout,
            concat!(
                "Cherry:<S>4:4<E>:good:Oct\n",
                "Cherry:<S>4:4<E>:good:Oct\n",
                "Apple:<S>33:3.3<E>:good:Mar\n",
                "Apple:<S>33:3.3<E>:good:Mar\n",
                "Orange:<S>222:1.1<E>.2:good:Jan\n",
                "Orange:<S>222:1.1<E>.2:good:Jan\n",
                "Kiwi:<S>1111:1.1<E>.11:good:Jun\n",
                "Kiwi:<S>1111:1.1<E>.11:good:Jun\n",
            )
        );
        assert!(oup.status.success());
    }
    //
    #[test]
    fn test_t6_header() {
        let in_w = std::fs::read_to_string(fixture_fruit_header!()).unwrap();
        let oup = exec_target_with_env_in(
            TARGET_EXE_PATH,
            [
                "-e",
                "([0-9]+:([0-9]+:)?[0-9]+(.[0-9]+)?)",
                "--according-to",
                "time",
                "-h",
                "1",
                "--color",
                "always",
            ],
            env_1!(),
            in_w.as_bytes(),
        );
        assert_eq!(oup.stderr, "");
        assert_eq!(
            oup.stdout,
            concat!(
                "name:number:version:nice:month\n",
                "Cherry:<S>4:4<E>:good:Oct\n",
                "Apple:<S>33:3.3<E>:good:Mar\n",
                "Orange:<S>222:1.1<E>.2:good:Jan\n",
                "Kiwi:<S>1111:1.1<E>.11:good:Jun\n",
            )
        );
        assert!(oup.status.success());
    }
    //
    #[test]
    fn test_t6_footer() {
        let in_w = std::fs::read_to_string(fixture_fruit_footer!()).unwrap();
        let oup = exec_target_with_env_in(
            TARGET_EXE_PATH,
            [
                "-e",
                "([0-9]+:([0-9]+:)?[0-9]+(.[0-9]+)?)",
                "--according-to",
                "time",
                "-t",
                "1",
                "--color",
                "always",
            ],
            env_1!(),
            in_w.as_bytes(),
        );
        assert_eq!(oup.stderr, "");
        assert_eq!(
            oup.stdout,
            concat!(
                "Cherry:<S>4:4<E>:good:Oct\n",
                "Apple:<S>33:3.3<E>:good:Mar\n",
                "Orange:<S>222:1.1<E>.2:good:Jan\n",
                "Kiwi:<S>1111:1.1<E>.11:good:Jun\n",
                "This is footer line. 1\n",
            )
        );
        assert!(oup.status.success());
    }
}

mod test_3_e {
    use exec_target::exec_target_with_env_in;
    use exec_target::exec_target_with_in;
    const TARGET_EXE_PATH: &str = super::TARGET_EXE_PATH;
    //
    #[test]
    fn test_max_buffer() {
        let in_w = std::fs::read_to_string(fixture_fruit!()).unwrap();
        let oup = exec_target_with_in(TARGET_EXE_PATH, ["--max-buffer", "20"], in_w.as_bytes());
        assert_eq!(
            oup.stderr,
            concat!(program_name!(), ": over max buffer size: 20\n")
        );
        assert_eq!(oup.stdout, "");
        assert!(!oup.status.success());
    }
    //
    #[test]
    fn test_uniq() {
        let in_w = std::fs::read_to_string(fixture_fruit!()).unwrap();
        let in_w = in_w.clone() + &in_w;
        let oup = exec_target_with_in(TARGET_EXE_PATH, ["-u"], in_w.as_bytes());
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
        assert!(oup.status.success());
    }
    //
    #[test]
    fn test_uniq_color() {
        let in_w = std::fs::read_to_string(fixture_fruit!()).unwrap();
        let in_w = in_w.clone() + &in_w;
        let oup = exec_target_with_env_in(
            TARGET_EXE_PATH,
            ["-u", "--color", "always"],
            env_1!(),
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
        assert!(oup.status.success());
    }
    //
    #[test]
    fn test_unique_with_reverse() {
        let in_w = "b\na\nc\nb\na\n";
        let oup = exec_target_with_in(TARGET_EXE_PATH, ["-u", "-r"], in_w.as_bytes());
        assert_eq!(oup.stderr, "");
        assert_eq!(oup.stdout, "c\nb\na\n");
        assert!(oup.status.success());
    }
    //
    #[test]
    fn test_unique_with_header() {
        let in_w = "header\nb\na\nc\nb\na\n";
        let oup = exec_target_with_in(TARGET_EXE_PATH, ["-u", "-h", "1"], in_w.as_bytes());
        assert_eq!(oup.stderr, "");
        assert_eq!(oup.stdout, "header\na\nb\nc\n");
        assert!(oup.status.success());
    }
    //
    #[test]
    fn test_unique_with_header_and_reverse() {
        let in_w = "header\nb\na\nc\nb\na\n";
        let oup = exec_target_with_in(TARGET_EXE_PATH, ["-u", "-h", "1", "-r"], in_w.as_bytes());
        assert_eq!(oup.stderr, "");
        assert_eq!(oup.stdout, "header\nc\nb\na\n");
        assert!(oup.status.success());
    }
}

mod test_4_combination_options_e {
    use exec_target::exec_target_with_in;
    const TARGET_EXE_PATH: &str = super::TARGET_EXE_PATH;
    //
    #[test]
    fn test_reverse_unique() {
        let in_w = "b\na\nc\nb\na\n";
        let oup = exec_target_with_in(TARGET_EXE_PATH, ["-r", "-u"], in_w.as_bytes());
        assert_eq!(oup.stderr, "");
        assert_eq!(oup.stdout, "c\nb\na\n");
        assert!(oup.status.success());
    }
    //
    #[test]
    fn test_reverse_head() {
        let in_w = "b\na\nc\n";
        let oup = exec_target_with_in(TARGET_EXE_PATH, ["-r", "-h", "1"], in_w.as_bytes());
        assert_eq!(oup.stderr, "");
        assert_eq!(oup.stdout, "b\nc\na\n");
        assert!(oup.status.success());
    }
    //
    #[test]
    fn test_reverse_tail() {
        let in_w = "b\na\nc\n";
        let oup = exec_target_with_in(TARGET_EXE_PATH, ["-r", "-t", "1"], in_w.as_bytes());
        assert_eq!(oup.stderr, "");
        assert_eq!(oup.stdout, "b\na\nc\n");
        assert!(oup.status.success());
    }
    //
    #[test]
    fn test_unique_head() {
        let in_w = "b\na\nc\nb\na\n";
        let oup = exec_target_with_in(TARGET_EXE_PATH, ["-u", "-h", "1"], in_w.as_bytes());
        assert_eq!(oup.stderr, "");
        assert_eq!(oup.stdout, "b\na\nb\nc\n");
        assert!(oup.status.success());
    }
}

mod test_4_invalid_option_arguments_e {
    use exec_target::exec_target_with_in;
    const TARGET_EXE_PATH: &str = super::TARGET_EXE_PATH;
    //
    #[test]
    fn test_head_invalid() {
        let in_w = "b\na\nc\n";
        let oup = exec_target_with_in(TARGET_EXE_PATH, ["-h", "a"], in_w.as_bytes());
        assert!(oup.stderr.contains("invalid digit"));
        assert_eq!(oup.stdout, "");
        assert!(!oup.status.success());
    }
    //
    #[test]
    fn test_tail_invalid() {
        let in_w = "b\na\nc\n";
        let oup = exec_target_with_in(TARGET_EXE_PATH, ["-t", "a"], in_w.as_bytes());
        assert!(oup.stderr.contains("invalid digit"));
        assert_eq!(oup.stdout, "");
        assert!(!oup.status.success());
    }
    //
    #[test]
    fn test_max_buffer_invalid() {
        let in_w = "b\na\nc\n";
        let oup = exec_target_with_in(TARGET_EXE_PATH, ["--max-buffer", "a"], in_w.as_bytes());
        assert!(oup.stderr.contains("max-buffer: can not parse"));
        assert_eq!(oup.stdout, "");
        assert!(!oup.status.success());
    }
    //
    #[test]
    fn test_according_to_invalid() {
        let in_w = "b\na\nc\n";
        let oup = exec_target_with_in(
            TARGET_EXE_PATH,
            ["--according-to", "invalid"],
            in_w.as_bytes(),
        );
        assert!(oup.stderr.contains("according-to: can not parse"));
        assert_eq!(oup.stdout, "");
        assert!(!oup.status.success());
    }
    //
    #[test]
    fn test_color_invalid() {
        let in_w = "b\na\nc\n";
        let oup = exec_target_with_in(TARGET_EXE_PATH, ["--color", "invalid"], in_w.as_bytes());
        assert!(oup.stderr.contains("color: can not parse"));
        assert_eq!(oup.stdout, "");
        assert!(!oup.status.success());
    }
    //
    #[test]
    fn test_exp_invalid() {
        let in_w = "b\na\nc\n";
        let oup = exec_target_with_in(TARGET_EXE_PATH, ["-e", "["], in_w.as_bytes());
        assert!(oup.stderr.contains("regex parse error"));
        assert_eq!(oup.stdout, "");
        assert!(!oup.status.success());
    }
}

mod test_4_according_to_option_e {
    use exec_target::exec_target_with_in;
    const TARGET_EXE_PATH: &str = super::TARGET_EXE_PATH;
    //
    #[test]
    fn test_numeric_invalid() {
        let in_w = "b\na\nc\n";
        let oup = exec_target_with_in(
            TARGET_EXE_PATH,
            ["--according-to", "numeric"],
            in_w.as_bytes(),
        );
        assert!(oup.stderr.contains("invalid digit"));
        assert_eq!(oup.stdout, "");
        assert!(!oup.status.success());
    }
    //
    #[test]
    fn test_version_invalid() {
        let in_w = "b\na\nc\n";
        let oup = exec_target_with_in(
            TARGET_EXE_PATH,
            ["--according-to", "version"],
            in_w.as_bytes(),
        );
        assert!(oup.stderr.contains("unexpected character"));
        assert_eq!(oup.stdout, "");
        assert!(!oup.status.success());
    }
    //
    #[test]
    fn test_month_invalid() {
        let in_w = "b\na\nc\n";
        let oup = exec_target_with_in(
            TARGET_EXE_PATH,
            ["--according-to", "month"],
            in_w.as_bytes(),
        );
        assert!(oup.stderr.contains("invalid month"));
        assert_eq!(oup.stdout, "");
        assert!(!oup.status.success());
    }
    //
    #[test]
    fn test_time_invalid() {
        let in_w = "b\na\nc\n";
        let oup = exec_target_with_in(TARGET_EXE_PATH, ["--according-to", "time"], in_w.as_bytes());
        assert!(oup.stderr.contains("unexpected character"));
        assert_eq!(oup.stdout, "");
        assert!(!oup.status.success());
    }
    //
    #[test]
    fn test_numeric_negative() {
        let in_w = "-1\n-3\n-2\n";
        let oup = exec_target_with_in(
            TARGET_EXE_PATH,
            ["--according-to", "numeric"],
            in_w.as_bytes(),
        );
        assert_eq!(oup.stderr, "");
        assert_eq!(oup.stdout, "-3\n-2\n-1\n");
        assert!(oup.status.success());
    }
    //
    /*
    #[test]
    fn test_numeric_float() {
        let in_w = "1.1\n1.0\n1.2\n";
        let (r, sioe) = do_execute!(&["--according-to", "numeric"], in_w);
        assert_eq!(buff!(sioe, serr), "");
        assert_eq!(buff!(sioe, sout), "1.0\n1.1\n1.2\n");
        assert!(r.is_ok());
    }
    */
}

/*
mod test_3_e {
    use exec_target::exec_target;
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
mod test_4_e {
    use exec_target::exec_target_with_in;
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
