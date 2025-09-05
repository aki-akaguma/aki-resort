#[macro_use]
mod helper;

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
            #[allow(clippy::box_default)]
            Box::new(StringOut::default()),
            #[allow(clippy::box_default)]
            Box::new(StringErr::default()),
        );
        let program = env!("CARGO_PKG_NAME");
        let r = execute(&sioe, &program, $args);
        match r {
            Ok(_) => {}
            Err(ref err) => {
                let _ = sioe
                    .pg_err()
                    .lock()
                    .write_fmt(format_args!("{}: {:#}\n", program, err));
            }
        };
        (r, sioe)
    }};
    ($env:expr, $args:expr, $sin:expr,) => {{
        do_execute!($env, $args, $sin)
    }};
    ($env:expr, $args:expr, $sin:expr) => {{
        let sioe = RunnelIoe::new(
            Box::new(StringIn::with_str($sin)),
            #[allow(clippy::box_default)]
            Box::new(StringOut::default()),
            #[allow(clippy::box_default)]
            Box::new(StringErr::default()),
        );
        let program = env!("CARGO_PKG_NAME");
        let r = execute_env(&sioe, &program, $args, $env);
        match r {
            Ok(_) => {}
            Err(ref err) => {
                let _ = sioe
                    .pg_err()
                    .lock()
                    .write_fmt(format_args!("{}: {:#}\n", program, err));
            }
        };
        (r, sioe)
    }};
}

macro_rules! buff {
    ($sioe:expr, serr) => {
        $sioe.pg_err().lock().buffer_to_string()
    };
    ($sioe:expr, sout) => {
        $sioe.pg_out().lock().buffer_to_string()
    };
}

macro_rules! env_1 {
    () => {{
        let mut env = conf::EnvConf::new();
        env.color_seq_start = color_start!().to_string();
        env.color_seq_end = color_end!().to_string();
        env
    }};
}

mod test_0_s {
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
        assert!(r.is_ok());
    }
    #[test]
    fn test_help_long() {
        let (r, sioe) = do_execute!(&["--help"]);
        assert_eq!(buff!(sioe, serr), "");
        assert_eq!(buff!(sioe, sout), help_msg!());
        assert!(r.is_ok());
    }
    #[test]
    fn test_version() {
        let (r, sioe) = do_execute!(&["-V"]);
        assert_eq!(buff!(sioe, serr), "");
        assert_eq!(buff!(sioe, sout), version_msg!());
        assert!(r.is_ok());
    }
    #[test]
    fn test_version_long() {
        let (r, sioe) = do_execute!(&["--version"]);
        assert_eq!(buff!(sioe, serr), "");
        assert_eq!(buff!(sioe, sout), version_msg!());
        assert!(r.is_ok());
    }
    #[test]
    fn test_invalid_opt() {
        let (r, sioe) = do_execute!(["-z"]);
        assert_eq!(
            buff!(sioe, serr),
            concat!(
                program_name!(),
                ": ",
                "Invalid option: z\n",
                try_help_msg!()
            )
        );
        assert_eq!(buff!(sioe, sout), "");
        assert!(r.is_err());
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

mod test_0_x_options_s {
    use libaki_resort::*;
    use runnel::medium::stringio::*;
    use runnel::*;
    //
    #[test]
    fn test_x_rust_version_info() {
        let (r, sioe) = do_execute!(["-X", "rust-version-info"]);
        assert_eq!(buff!(sioe, serr), "");
        assert!(!buff!(sioe, sout).is_empty());
        assert!(r.is_ok());
    }
    //
    #[test]
    fn test_x_option_help() {
        let (r, sioe) = do_execute!(["-X", "help"]);
        assert_eq!(buff!(sioe, serr), "");
        assert!(buff!(sioe, sout).contains("Options:"));
        assert!(buff!(sioe, sout).contains("-X rust-version-info"));
        assert!(r.is_ok());
    }
    //
    #[test]
    fn test_x_option_rust_version_info() {
        let (r, sioe) = do_execute!(["-X", "rust-version-info"]);
        assert_eq!(buff!(sioe, serr), "");
        assert!(buff!(sioe, sout).contains("rustc"));
        assert!(r.is_ok());
    }
    //
    #[test]
    fn test_multiple_x_options() {
        let (r, sioe) = do_execute!(["-X", "help", "-X", "rust-version-info"]);
        assert_eq!(buff!(sioe, serr), "");
        // The first one should be executed and the program should exit.
        assert!(buff!(sioe, sout).contains("Options:"));
        assert!(!buff!(sioe, sout).contains("rustc"));
        assert!(r.is_ok());
    }
}

mod test_1_string_s {
    use libaki_resort::*;
    use runnel::medium::stringio::{StringErr, StringIn, StringOut};
    use runnel::RunnelIoe;
    use std::io::Write;
    //
    #[test]
    fn test_t1() {
        let in_w = std::fs::read_to_string(fixture_fruit!()).unwrap();
        let (r, sioe) = do_execute!(&[] as &[&str], &in_w);
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
        assert!(r.is_ok());
    }
    //
    #[test]
    fn test_t2() {
        let in_w = std::fs::read_to_string(fixture_fruit!()).unwrap();
        let (r, sioe) = do_execute!(&["-r"], &in_w);
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
        assert!(r.is_ok());
    }
    //
    #[test]
    fn test_t3() {
        let in_w = std::fs::read_to_string(fixture_fruit!()).unwrap();
        let (r, sioe) = do_execute!(&["-e", "[0-9]+"], &in_w);
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
        assert!(r.is_ok());
    }
    //
    #[test]
    fn test_t4() {
        let in_w = std::fs::read_to_string(fixture_fruit!()).unwrap();
        let (r, sioe) = do_execute!(&["-e", "[0-9]+", "-r"], &in_w);
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
        assert!(r.is_ok());
    }
    //
    #[test]
    fn test_t5() {
        let in_w = std::fs::read_to_string(fixture_fruit!()).unwrap();
        let in_w = in_w.to_string() + &in_w;
        let (r, sioe) = do_execute!(&["-e", "[0-9]+"], &in_w);
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
        assert!(r.is_ok());
    }
    //
    #[test]
    fn test_t6_header() {
        let in_w = std::fs::read_to_string(fixture_fruit_header!()).unwrap();
        let (r, sioe) = do_execute!(&["-e", "[0-9]+", "-h", "1"], &in_w);
        assert_eq!(buff!(sioe, serr), "");
        assert_eq!(
            buff!(sioe, sout),
            concat!(
                "name:number:version:nice:month\n",
                "Kiwi:1111:1.1.11:good:Jun\n",
                "Orange:222:1.1.2:good:Jan\n",
                "Apple:33:3.3:good:Mar\n",
                "Cherry:4:4:good:Oct\n",
            )
        );
        assert!(r.is_ok());
    }
    //
    #[test]
    fn test_t6_footer() {
        let in_w = std::fs::read_to_string(fixture_fruit_footer!()).unwrap();
        let (r, sioe) = do_execute!(&["-e", "[0-9]+", "-t", "1"], in_w.as_str(),);
        assert_eq!(buff!(sioe, serr), "");
        assert_eq!(
            buff!(sioe, sout),
            concat!(
                "Kiwi:1111:1.1.11:good:Jun\n",
                "Orange:222:1.1.2:good:Jan\n",
                "Apple:33:3.3:good:Mar\n",
                "Cherry:4:4:good:Oct\n",
                "This is footer line. 1\n",
            )
        );
        assert!(r.is_ok());
    }
}

mod test_1_string_color_s {
    use libaki_resort::*;
    use runnel::medium::stringio::{StringErr, StringIn, StringOut};
    use runnel::RunnelIoe;
    use std::io::Write;
    //
    #[test]
    fn test_t1() {
        let env = env_1!();
        let in_w = std::fs::read_to_string(fixture_fruit!()).unwrap();
        let (r, sioe) = do_execute!(&env, &["--color", "always"], &in_w);
        assert_eq!(buff!(sioe, serr), "");
        assert_eq!(
            buff!(sioe, sout),
            concat!(
                "<S>Apple:33:3.3:good:Mar<E>\n",
                "<S>Cherry:4:4:good:Oct<E>\n",
                "<S>Kiwi:1111:1.1.11:good:Jun<E>\n",
                "<S>Orange:222:1.1.2:good:Jan<E>\n",
            )
        );
        assert!(r.is_ok());
    }
    //
    #[test]
    fn test_t2() {
        let env = env_1!();
        let in_w = std::fs::read_to_string(fixture_fruit!()).unwrap();
        let (r, sioe) = do_execute!(&env, &["-r", "--color", "always"], &in_w);
        assert_eq!(buff!(sioe, serr), "");
        assert_eq!(
            buff!(sioe, sout),
            concat!(
                "<S>Orange:222:1.1.2:good:Jan<E>\n",
                "<S>Kiwi:1111:1.1.11:good:Jun<E>\n",
                "<S>Cherry:4:4:good:Oct<E>\n",
                "<S>Apple:33:3.3:good:Mar<E>\n",
            )
        );
        assert!(r.is_ok());
    }
    //
    #[test]
    fn test_t3() {
        let env = env_1!();
        let in_w = std::fs::read_to_string(fixture_fruit!()).unwrap();
        let (r, sioe) = do_execute!(&env, &["-e", "[0-9]+", "--color", "always"], &in_w);
        assert_eq!(buff!(sioe, serr), "");
        assert_eq!(
            buff!(sioe, sout),
            concat!(
                "Kiwi:<S>1111<E>:1.1.11:good:Jun\n",
                "Orange:<S>222<E>:1.1.2:good:Jan\n",
                "Apple:<S>33<E>:3.3:good:Mar\n",
                "Cherry:<S>4<E>:4:good:Oct\n",
            )
        );
        assert!(r.is_ok());
    }
    //
    #[test]
    fn test_t4() {
        let env = env_1!();
        let in_w = std::fs::read_to_string(fixture_fruit!()).unwrap();
        let (r, sioe) = do_execute!(&env, &["-e", "[0-9]+", "-r", "--color", "always"], &in_w);
        assert_eq!(buff!(sioe, serr), "");
        assert_eq!(
            buff!(sioe, sout),
            concat!(
                "Cherry:<S>4<E>:4:good:Oct\n",
                "Apple:<S>33<E>:3.3:good:Mar\n",
                "Orange:<S>222<E>:1.1.2:good:Jan\n",
                "Kiwi:<S>1111<E>:1.1.11:good:Jun\n",
            )
        );
        assert!(r.is_ok());
    }
    //
    #[test]
    fn test_t5() {
        let env = env_1!();
        let in_w = std::fs::read_to_string(fixture_fruit!()).unwrap();
        let in_w = in_w.to_string() + &in_w;
        let (r, sioe) = do_execute!(&env, &["-e", "[0-9]+", "--color", "always"], in_w.as_str());
        assert_eq!(buff!(sioe, serr), "");
        assert_eq!(
            buff!(sioe, sout),
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
        assert!(r.is_ok());
    }
    //
    #[test]
    fn test_t6_header() {
        let env = env_1!();
        let in_w = std::fs::read_to_string(fixture_fruit_header!()).unwrap();
        let (r, sioe) = do_execute!(
            &env,
            &["-e", "[0-9]+", "-h", "1", "--color", "always"],
            &in_w,
        );
        assert_eq!(buff!(sioe, serr), "");
        assert_eq!(
            buff!(sioe, sout),
            concat!(
                "name:number:version:nice:month\n",
                "Kiwi:<S>1111<E>:1.1.11:good:Jun\n",
                "Orange:<S>222<E>:1.1.2:good:Jan\n",
                "Apple:<S>33<E>:3.3:good:Mar\n",
                "Cherry:<S>4<E>:4:good:Oct\n",
            )
        );
        assert!(r.is_ok());
    }
    //
    #[test]
    fn test_t6_footer() {
        let env = env_1!();
        let in_w = std::fs::read_to_string(fixture_fruit_footer!()).unwrap();
        let (r, sioe) = do_execute!(
            &env,
            &["-e", "[0-9]+", "-t", "1", "--color", "always"],
            &in_w,
        );
        assert_eq!(buff!(sioe, serr), "");
        assert_eq!(
            buff!(sioe, sout),
            concat!(
                "Kiwi:<S>1111<E>:1.1.11:good:Jun\n",
                "Orange:<S>222<E>:1.1.2:good:Jan\n",
                "Apple:<S>33<E>:3.3:good:Mar\n",
                "Cherry:<S>4<E>:4:good:Oct\n",
                "This is footer line. 1\n",
            )
        );
        assert!(r.is_ok());
    }
}

mod test_1_numeric_2 {
    use libaki_resort::*;
    use runnel::medium::stringio::{StringErr, StringIn, StringOut};
    use runnel::RunnelIoe;
    use std::io::Write;
    //
    #[test]
    fn test_t1() {
        let in_w = std::fs::read_to_string(fixture_fruit!()).unwrap();
        let (r, sioe) = do_execute!(&["--according-to", "numeric"], &in_w);
        assert_eq!(
            buff!(sioe, serr),
            concat!(
                program_name!(),
                ": (0,21):\'Apple:33:3.3:good:Mar\': invalid digit found in string\n"
            )
        );
        assert_eq!(buff!(sioe, sout), "");
        assert!(r.is_err());
    }
    //
    #[test]
    fn test_t2() {
        let in_w = std::fs::read_to_string(fixture_fruit!()).unwrap();
        let (r, sioe) = do_execute!(&["--according-to", "numeric", "-r"], &in_w);
        assert_eq!(
            buff!(sioe, serr),
            concat!(
                program_name!(),
                ": (0,21):\'Apple:33:3.3:good:Mar\': invalid digit found in string\n"
            )
        );
        assert_eq!(buff!(sioe, sout), "");
        assert!(r.is_err());
    }
    //
    #[test]
    fn test_t3() {
        let in_w = std::fs::read_to_string(fixture_fruit!()).unwrap();
        let (r, sioe) = do_execute!(&["-e", "[0-9]+", "--according-to", "numeric"], &in_w);
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
        assert!(r.is_ok());
    }
    //
    #[test]
    fn test_t4() {
        let in_w = std::fs::read_to_string(fixture_fruit!()).unwrap();
        let (r, sioe) = do_execute!(&["-e", "[0-9]+", "--according-to", "numeric", "-r"], &in_w);
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
        assert!(r.is_ok());
    }
    //
    #[test]
    fn test_t5() {
        let in_w = std::fs::read_to_string(fixture_fruit!()).unwrap();
        let in_w = in_w.to_string() + &in_w;
        let (r, sioe) = do_execute!(&["-e", "[0-9]+", "--according-to", "numeric"], &in_w,);
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
        assert!(r.is_ok());
    }
    //
    #[test]
    fn test_t6_header() {
        let in_w = std::fs::read_to_string(fixture_fruit_header!()).unwrap();
        let (r, sioe) = do_execute!(
            &["-e", "[0-9]+", "--according-to", "numeric", "-h", "1"],
            &in_w,
        );
        assert_eq!(buff!(sioe, serr), "");
        assert_eq!(
            buff!(sioe, sout),
            concat!(
                "name:number:version:nice:month\n",
                "Cherry:4:4:good:Oct\n",
                "Apple:33:3.3:good:Mar\n",
                "Orange:222:1.1.2:good:Jan\n",
                "Kiwi:1111:1.1.11:good:Jun\n",
            )
        );
        assert!(r.is_ok());
    }
    //
    #[test]
    fn test_t6_footer() {
        let in_w = std::fs::read_to_string(fixture_fruit_footer!()).unwrap();
        let (r, sioe) = do_execute!(
            &["-e", "[0-9]+", "--according-to", "numeric", "-t", "1"],
            &in_w,
        );
        assert_eq!(buff!(sioe, serr), "");
        assert_eq!(
            buff!(sioe, sout),
            concat!(
                "Cherry:4:4:good:Oct\n",
                "Apple:33:3.3:good:Mar\n",
                "Orange:222:1.1.2:good:Jan\n",
                "Kiwi:1111:1.1.11:good:Jun\n",
                "This is footer line. 1\n",
            )
        );
        assert!(r.is_ok());
    }
}

mod test_1_numeric_color_s {
    use libaki_resort::*;
    use runnel::medium::stringio::{StringErr, StringIn, StringOut};
    use runnel::RunnelIoe;
    use std::io::Write;
    //
    #[test]
    fn test_t1() {
        let env = env_1!();
        let in_w = std::fs::read_to_string(fixture_fruit!()).unwrap();
        let (r, sioe) = do_execute!(
            &env,
            &["--according-to", "numeric", "--color", "always"],
            &in_w,
        );
        assert_eq!(
            buff!(sioe, serr),
            concat!(
                program_name!(),
                ": (0,21):\'Apple:33:3.3:good:Mar\': invalid digit found in string\n"
            )
        );
        assert_eq!(buff!(sioe, sout), "");
        assert!(r.is_err());
    }
    //
    #[test]
    fn test_t2() {
        let env = env_1!();
        let in_w = std::fs::read_to_string(fixture_fruit!()).unwrap();
        let (r, sioe) = do_execute!(
            &env,
            &["--according-to", "numeric", "-r", "--color", "always"],
            &in_w,
        );
        assert_eq!(
            buff!(sioe, serr),
            concat!(
                program_name!(),
                ": (0,21):\'Apple:33:3.3:good:Mar\': invalid digit found in string\n"
            )
        );
        assert_eq!(buff!(sioe, sout), "");
        assert!(r.is_err());
    }
    //
    #[test]
    fn test_t3() {
        let env = env_1!();
        let in_w = std::fs::read_to_string(fixture_fruit!()).unwrap();
        let (r, sioe) = do_execute!(
            &env,
            &[
                "-e",
                "[0-9]+",
                "--according-to",
                "numeric",
                "--color",
                "always"
            ],
            &in_w,
        );
        assert_eq!(buff!(sioe, serr), "");
        assert_eq!(
            buff!(sioe, sout),
            concat!(
                "Cherry:<S>4<E>:4:good:Oct\n",
                "Apple:<S>33<E>:3.3:good:Mar\n",
                "Orange:<S>222<E>:1.1.2:good:Jan\n",
                "Kiwi:<S>1111<E>:1.1.11:good:Jun\n",
            )
        );
        assert!(r.is_ok());
    }
    //
    #[test]
    fn test_t4() {
        let env = env_1!();
        let in_w = std::fs::read_to_string(fixture_fruit!()).unwrap();
        let (r, sioe) = do_execute!(
            &env,
            &[
                "-e",
                "[0-9]+",
                "--according-to",
                "numeric",
                "-r",
                "--color",
                "always"
            ],
            &in_w,
        );
        assert_eq!(buff!(sioe, serr), "");
        assert_eq!(
            buff!(sioe, sout),
            concat!(
                "Kiwi:<S>1111<E>:1.1.11:good:Jun\n",
                "Orange:<S>222<E>:1.1.2:good:Jan\n",
                "Apple:<S>33<E>:3.3:good:Mar\n",
                "Cherry:<S>4<E>:4:good:Oct\n",
            )
        );
        assert!(r.is_ok());
    }
    //
    #[test]
    fn test_t5() {
        let env = env_1!();
        let in_w = std::fs::read_to_string(fixture_fruit!()).unwrap();
        let in_w = in_w.to_string() + &in_w;
        let (r, sioe) = do_execute!(
            &env,
            &[
                "-e",
                "[0-9]+",
                "--according-to",
                "numeric",
                "--color",
                "always"
            ],
            &in_w,
        );
        assert_eq!(buff!(sioe, serr), "");
        assert_eq!(
            buff!(sioe, sout),
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
        assert!(r.is_ok());
    }
    //
    #[test]
    fn test_t6_header() {
        let env = env_1!();
        let in_w = std::fs::read_to_string(fixture_fruit_header!()).unwrap();
        let (r, sioe) = do_execute!(
            &env,
            &[
                "-e",
                "[0-9]+",
                "--according-to",
                "numeric",
                "-h",
                "1",
                "--color",
                "always"
            ],
            &in_w,
        );
        assert_eq!(buff!(sioe, serr), "");
        assert_eq!(
            buff!(sioe, sout),
            concat!(
                "name:number:version:nice:month\n",
                "Cherry:<S>4<E>:4:good:Oct\n",
                "Apple:<S>33<E>:3.3:good:Mar\n",
                "Orange:<S>222<E>:1.1.2:good:Jan\n",
                "Kiwi:<S>1111<E>:1.1.11:good:Jun\n",
            )
        );
        assert!(r.is_ok());
    }
    //
    #[test]
    fn test_t6_footer() {
        let env = env_1!();
        let in_w = std::fs::read_to_string(fixture_fruit_footer!()).unwrap();
        let (r, sioe) = do_execute!(
            &env,
            &[
                "-e",
                "[0-9]+",
                "--according-to",
                "numeric",
                "-t",
                "1",
                "--color",
                "always"
            ],
            &in_w,
        );
        assert_eq!(buff!(sioe, serr), "");
        assert_eq!(
            buff!(sioe, sout),
            concat!(
                "Cherry:<S>4<E>:4:good:Oct\n",
                "Apple:<S>33<E>:3.3:good:Mar\n",
                "Orange:<S>222<E>:1.1.2:good:Jan\n",
                "Kiwi:<S>1111<E>:1.1.11:good:Jun\n",
                "This is footer line. 1\n",
            )
        );
        assert!(r.is_ok());
    }
}

mod test_1_version_s {
    use libaki_resort::*;
    use runnel::medium::stringio::{StringErr, StringIn, StringOut};
    use runnel::RunnelIoe;
    use std::io::Write;
    //
    #[test]
    fn test_t1() {
        let in_w = std::fs::read_to_string(fixture_fruit!()).unwrap();
        let (r, sioe) = do_execute!(&["--according-to", "version"], &in_w);
        assert_eq!(
            buff!(sioe, serr),
            concat!(
                program_name!(),
                ": (0,21):\'Apple:33:3.3:good:Mar\': unexpected character \'A\' while parsing major version number\n",
            )
        );
        assert_eq!(buff!(sioe, sout), "");
        assert!(r.is_err());
    }
    //
    #[test]
    fn test_t2() {
        let in_w = std::fs::read_to_string(fixture_fruit!()).unwrap();
        let (r, sioe) = do_execute!(&["--according-to", "version", "-r"], &in_w);
        assert_eq!(
            buff!(sioe, serr),
            concat!(
                program_name!(),
                ": (0,21):\'Apple:33:3.3:good:Mar\': unexpected character \'A\' while parsing major version number\n",
            )
        );
        assert_eq!(buff!(sioe, sout), "");
        assert!(r.is_err());
    }
    //
    #[test]
    fn test_t3() {
        let in_w = std::fs::read_to_string(fixture_fruit!()).unwrap();
        let (r, sioe) = do_execute!(
            &["-e", "[^:]+:[^:]+:([0-9.]+):", "--according-to", "version"],
            &in_w,
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
        assert!(r.is_ok());
    }
    //
    #[test]
    fn test_t4() {
        let in_w = std::fs::read_to_string(fixture_fruit!()).unwrap();
        let (r, sioe) = do_execute!(
            &[
                "-e",
                "[^:]+:[^:]+:([0-9.]+):",
                "--according-to",
                "version",
                "-r",
            ],
            &in_w,
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
        assert!(r.is_ok());
    }
    //
    #[test]
    fn test_t5() {
        let in_w = std::fs::read_to_string(fixture_fruit!()).unwrap();
        let in_w = in_w.to_string() + &in_w;
        let (r, sioe) = do_execute!(
            &["-e", "[^:]+:[^:]+:([0-9.]+):", "--according-to", "version"],
            &in_w,
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
        assert!(r.is_ok());
    }
    //
    #[test]
    fn test_t6_header() {
        let in_w = std::fs::read_to_string(fixture_fruit_header!()).unwrap();
        let (r, sioe) = do_execute!(
            &[
                "-e",
                "[^:]+:[^:]+:([0-9.]+):",
                "--according-to",
                "version",
                "-h",
                "1"
            ],
            &in_w,
        );
        assert_eq!(buff!(sioe, serr), "");
        assert_eq!(
            buff!(sioe, sout),
            concat!(
                "name:number:version:nice:month\n",
                "Orange:222:1.1.2:good:Jan\n",
                "Kiwi:1111:1.1.11:good:Jun\n",
                "Apple:33:3.3:good:Mar\n",
                "Cherry:4:4:good:Oct\n",
            )
        );
        assert!(r.is_ok());
    }
    //
    #[test]
    fn test_t6_footer() {
        let in_w = std::fs::read_to_string(fixture_fruit_footer!()).unwrap();
        let (r, sioe) = do_execute!(
            &[
                "-e",
                "[^:]+:[^:]+:([0-9.]+):",
                "--according-to",
                "version",
                "-t",
                "1"
            ],
            &in_w,
        );
        assert_eq!(buff!(sioe, serr), "");
        assert_eq!(
            buff!(sioe, sout),
            concat!(
                "Orange:222:1.1.2:good:Jan\n",
                "Kiwi:1111:1.1.11:good:Jun\n",
                "Apple:33:3.3:good:Mar\n",
                "Cherry:4:4:good:Oct\n",
                "This is footer line. 1\n",
            )
        );
        assert!(r.is_ok());
    }
}

mod test_1_version_color_s {
    use libaki_resort::*;
    use runnel::medium::stringio::{StringErr, StringIn, StringOut};
    use runnel::RunnelIoe;
    use std::io::Write;
    //
    #[test]
    fn test_t1() {
        let env = env_1!();
        let in_w = std::fs::read_to_string(fixture_fruit!()).unwrap();
        let (r, sioe) = do_execute!(
            &env,
            &["--according-to", "version", "--color", "always"],
            &in_w,
        );
        assert_eq!(
            buff!(sioe, serr),
            concat!(
                program_name!(),
                ": (0,21):\'Apple:33:3.3:good:Mar\': unexpected character \'A\' while parsing major version number\n",
            )
        );
        assert_eq!(buff!(sioe, sout), "");
        assert!(r.is_err());
    }
    //
    #[test]
    fn test_t2() {
        let env = env_1!();
        let in_w = std::fs::read_to_string(fixture_fruit!()).unwrap();
        let (r, sioe) = do_execute!(
            &env,
            &["--according-to", "version", "-r", "--color", "always"],
            &in_w,
        );
        assert_eq!(
            buff!(sioe, serr),
            concat!(
                program_name!(),
                ": (0,21):\'Apple:33:3.3:good:Mar\': unexpected character \'A\' while parsing major version number\n",
            )
        );
        assert_eq!(buff!(sioe, sout), "");
        assert!(r.is_err());
    }
    //
    #[test]
    fn test_t3() {
        let env = env_1!();
        let in_w = std::fs::read_to_string(fixture_fruit!()).unwrap();
        let (r, sioe) = do_execute!(
            &env,
            &[
                "-e",
                "[^:]+:[^:]+:([0-9.]+):",
                "--according-to",
                "version",
                "--color",
                "always"
            ],
            &in_w,
        );
        assert_eq!(buff!(sioe, serr), "");
        assert_eq!(
            buff!(sioe, sout),
            concat!(
                "Orange:222:<S>1.1.2<E>:good:Jan\n",
                "Kiwi:1111:<S>1.1.11<E>:good:Jun\n",
                "Apple:33:<S>3.3<E>:good:Mar\n",
                "Cherry:4:<S>4<E>:good:Oct\n",
            )
        );
        assert!(r.is_ok());
    }
    //
    #[test]
    fn test_t4() {
        let env = env_1!();
        let in_w = std::fs::read_to_string(fixture_fruit!()).unwrap();
        let (r, sioe) = do_execute!(
            &env,
            &[
                "-e",
                "[^:]+:[^:]+:([0-9.]+):",
                "--according-to",
                "version",
                "-r",
                "--color",
                "always"
            ],
            &in_w,
        );
        assert_eq!(buff!(sioe, serr), "");
        assert_eq!(
            buff!(sioe, sout),
            concat!(
                "Cherry:4:<S>4<E>:good:Oct\n",
                "Apple:33:<S>3.3<E>:good:Mar\n",
                "Kiwi:1111:<S>1.1.11<E>:good:Jun\n",
                "Orange:222:<S>1.1.2<E>:good:Jan\n",
            )
        );
        assert!(r.is_ok());
    }
    //
    #[test]
    fn test_t5() {
        let env = env_1!();
        let in_w = std::fs::read_to_string(fixture_fruit!()).unwrap();
        let in_w = in_w.to_string() + &in_w;
        let (r, sioe) = do_execute!(
            &env,
            &[
                "-e",
                "[^:]+:[^:]+:([0-9.]+):",
                "--according-to",
                "version",
                "--color",
                "always"
            ],
            &in_w,
        );
        assert_eq!(buff!(sioe, serr), "");
        assert_eq!(
            buff!(sioe, sout),
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
        assert!(r.is_ok());
    }
    //
    #[test]
    fn test_t6_header() {
        let env = env_1!();
        let in_w = std::fs::read_to_string(fixture_fruit_header!()).unwrap();
        let (r, sioe) = do_execute!(
            &env,
            &[
                "-e",
                "[^:]+:[^:]+:([0-9.]+):",
                "--according-to",
                "version",
                "-h",
                "1",
                "--color",
                "always"
            ],
            &in_w,
        );
        assert_eq!(buff!(sioe, serr), "");
        assert_eq!(
            buff!(sioe, sout),
            concat!(
                "name:number:version:nice:month\n",
                "Orange:222:<S>1.1.2<E>:good:Jan\n",
                "Kiwi:1111:<S>1.1.11<E>:good:Jun\n",
                "Apple:33:<S>3.3<E>:good:Mar\n",
                "Cherry:4:<S>4<E>:good:Oct\n",
            )
        );
        assert!(r.is_ok());
    }
    //
    #[test]
    fn test_t6_footer() {
        let env = env_1!();
        let in_w = std::fs::read_to_string(fixture_fruit_footer!()).unwrap();
        let (r, sioe) = do_execute!(
            &env,
            &[
                "-e",
                "[^:]+:[^:]+:([0-9.]+):",
                "--according-to",
                "version",
                "-t",
                "1",
                "--color",
                "always"
            ],
            &in_w,
        );
        assert_eq!(buff!(sioe, serr), "");
        assert_eq!(
            buff!(sioe, sout),
            concat!(
                "Orange:222:<S>1.1.2<E>:good:Jan\n",
                "Kiwi:1111:<S>1.1.11<E>:good:Jun\n",
                "Apple:33:<S>3.3<E>:good:Mar\n",
                "Cherry:4:<S>4<E>:good:Oct\n",
                "This is footer line. 1\n",
            )
        );
        assert!(r.is_ok());
    }
}

mod test_1_month_s {
    use libaki_resort::*;
    use runnel::medium::stringio::{StringErr, StringIn, StringOut};
    use runnel::RunnelIoe;
    use std::io::Write;
    //
    #[test]
    fn test_t1() {
        let in_w = std::fs::read_to_string(fixture_fruit!()).unwrap();
        let (r, sioe) = do_execute!(&["--according-to", "month"], &in_w);
        assert_eq!(
            buff!(sioe, serr),
            concat!(
                program_name!(),
                ": (0,21):\'Apple:33:3.3:good:Mar\': invalid month strings\n",
            )
        );
        assert_eq!(buff!(sioe, sout), "");
        assert!(r.is_err());
    }
    //
    #[test]
    fn test_t2() {
        let in_w = std::fs::read_to_string(fixture_fruit!()).unwrap();
        let (r, sioe) = do_execute!(&["--according-to", "month", "-r"], &in_w);
        assert_eq!(
            buff!(sioe, serr),
            concat!(
                program_name!(),
                ": (0,21):\'Apple:33:3.3:good:Mar\': invalid month strings\n",
            )
        );
        assert_eq!(buff!(sioe, sout), "");
        assert!(r.is_err());
    }
    //
    #[test]
    fn test_t3() {
        let in_w = std::fs::read_to_string(fixture_fruit!()).unwrap();
        let (r, sioe) = do_execute!(&["-e", ":([^:]+)$", "--according-to", "month"], &in_w);
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
        assert!(r.is_ok());
    }
    //
    #[test]
    fn test_t4() {
        let in_w = std::fs::read_to_string(fixture_fruit!()).unwrap();
        let (r, sioe) = do_execute!(&["-e", ":([^:]+)$", "--according-to", "month", "-r"], &in_w);
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
        assert!(r.is_ok());
    }
    //
    #[test]
    fn test_t5() {
        let in_w = std::fs::read_to_string(fixture_fruit!()).unwrap();
        let in_w = in_w.to_string() + &in_w;
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
        assert!(r.is_ok());
    }
    //
    #[test]
    fn test_t6_header() {
        let in_w = std::fs::read_to_string(fixture_fruit_header!()).unwrap();
        let (r, sioe) = do_execute!(
            &["-e", ":([^:]+)$", "--according-to", "month", "-h", "1"],
            &in_w,
        );
        assert_eq!(buff!(sioe, serr), "");
        assert_eq!(
            buff!(sioe, sout),
            concat!(
                "name:number:version:nice:month\n",
                "Orange:222:1.1.2:good:Jan\n",
                "Apple:33:3.3:good:Mar\n",
                "Kiwi:1111:1.1.11:good:Jun\n",
                "Cherry:4:4:good:Oct\n",
            )
        );
        assert!(r.is_ok());
    }
    //
    #[test]
    fn test_t6_footer() {
        let in_w = std::fs::read_to_string(fixture_fruit_footer!()).unwrap();
        let (r, sioe) = do_execute!(
            &["-e", ":([^:]+)$", "--according-to", "month", "-t", "1"],
            &in_w,
        );
        assert_eq!(buff!(sioe, serr), "");
        assert_eq!(
            buff!(sioe, sout),
            concat!(
                "Orange:222:1.1.2:good:Jan\n",
                "Apple:33:3.3:good:Mar\n",
                "Kiwi:1111:1.1.11:good:Jun\n",
                "Cherry:4:4:good:Oct\n",
                "This is footer line. 1\n",
            )
        );
        assert!(r.is_ok());
    }
}

mod test_1_month_color_s {
    use libaki_resort::*;
    use runnel::medium::stringio::{StringErr, StringIn, StringOut};
    use runnel::RunnelIoe;
    use std::io::Write;
    //
    #[test]
    fn test_t1() {
        let env = env_1!();
        let in_w = std::fs::read_to_string(fixture_fruit!()).unwrap();
        let (r, sioe) = do_execute!(
            &env,
            &["--according-to", "month", "--color", "always"],
            &in_w,
        );
        assert_eq!(
            buff!(sioe, serr),
            concat!(
                program_name!(),
                ": (0,21):\'Apple:33:3.3:good:Mar\': invalid month strings\n",
            )
        );
        assert_eq!(buff!(sioe, sout), "");
        assert!(r.is_err());
    }
    //
    #[test]
    fn test_t2() {
        let env = env_1!();
        let in_w = std::fs::read_to_string(fixture_fruit!()).unwrap();
        let (r, sioe) = do_execute!(
            &env,
            &["--according-to", "month", "-r", "--color", "always"],
            &in_w,
        );
        assert_eq!(
            buff!(sioe, serr),
            concat!(
                program_name!(),
                ": (0,21):\'Apple:33:3.3:good:Mar\': invalid month strings\n",
            )
        );
        assert_eq!(buff!(sioe, sout), "");
        assert!(r.is_err());
    }
    //
    #[test]
    fn test_t3() {
        let env = env_1!();
        let in_w = std::fs::read_to_string(fixture_fruit!()).unwrap();
        let (r, sioe) = do_execute!(
            &env,
            &[
                "-e",
                ":([^:]+)$",
                "--according-to",
                "month",
                "--color",
                "always"
            ],
            &in_w,
        );
        assert_eq!(buff!(sioe, serr), "");
        assert_eq!(
            buff!(sioe, sout),
            concat!(
                "Orange:222:1.1.2:good:<S>Jan<E>\n",
                "Apple:33:3.3:good:<S>Mar<E>\n",
                "Kiwi:1111:1.1.11:good:<S>Jun<E>\n",
                "Cherry:4:4:good:<S>Oct<E>\n",
            )
        );
        assert!(r.is_ok());
    }
    //
    #[test]
    fn test_t4() {
        let env = env_1!();
        let in_w = std::fs::read_to_string(fixture_fruit!()).unwrap();
        let (r, sioe) = do_execute!(
            &env,
            &[
                "-e",
                ":([^:]+)$",
                "--according-to",
                "month",
                "-r",
                "--color",
                "always"
            ],
            &in_w,
        );
        assert_eq!(buff!(sioe, serr), "");
        assert_eq!(
            buff!(sioe, sout),
            concat!(
                "Cherry:4:4:good:<S>Oct<E>\n",
                "Kiwi:1111:1.1.11:good:<S>Jun<E>\n",
                "Apple:33:3.3:good:<S>Mar<E>\n",
                "Orange:222:1.1.2:good:<S>Jan<E>\n",
            )
        );
        assert!(r.is_ok());
    }
    //
    #[test]
    fn test_t5() {
        let env = env_1!();
        let in_w = std::fs::read_to_string(fixture_fruit!()).unwrap();
        let in_w = in_w.to_string() + &in_w;
        let (r, sioe) = do_execute!(
            &env,
            &[
                "-e",
                ":([^:]+)$",
                "--according-to",
                "month",
                "--color",
                "always"
            ],
            &in_w,
        );
        assert_eq!(buff!(sioe, serr), "");
        assert_eq!(
            buff!(sioe, sout),
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
        assert!(r.is_ok());
    }
    //
    #[test]
    fn test_t6_header() {
        let env = env_1!();
        let in_w = std::fs::read_to_string(fixture_fruit_header!()).unwrap();
        let (r, sioe) = do_execute!(
            &env,
            &[
                "-e",
                ":([^:]+)$",
                "--according-to",
                "month",
                "-h",
                "1",
                "--color",
                "always"
            ],
            &in_w,
        );
        assert_eq!(buff!(sioe, serr), "");
        assert_eq!(
            buff!(sioe, sout),
            concat!(
                "name:number:version:nice:month\n",
                "Orange:222:1.1.2:good:<S>Jan<E>\n",
                "Apple:33:3.3:good:<S>Mar<E>\n",
                "Kiwi:1111:1.1.11:good:<S>Jun<E>\n",
                "Cherry:4:4:good:<S>Oct<E>\n",
            )
        );
        assert!(r.is_ok());
    }
    //
    #[test]
    fn test_t6_footer() {
        let env = env_1!();
        let in_w = std::fs::read_to_string(fixture_fruit_footer!()).unwrap();
        let (r, sioe) = do_execute!(
            &env,
            &[
                "-e",
                ":([^:]+)$",
                "--according-to",
                "month",
                "-t",
                "1",
                "--color",
                "always"
            ],
            &in_w,
        );
        assert_eq!(buff!(sioe, serr), "");
        assert_eq!(
            buff!(sioe, sout),
            concat!(
                "Orange:222:1.1.2:good:<S>Jan<E>\n",
                "Apple:33:3.3:good:<S>Mar<E>\n",
                "Kiwi:1111:1.1.11:good:<S>Jun<E>\n",
                "Cherry:4:4:good:<S>Oct<E>\n",
                "This is footer line. 1\n",
            )
        );
        assert!(r.is_ok());
    }
}

mod test_1_time_s {
    use libaki_resort::*;
    use runnel::medium::stringio::{StringErr, StringIn, StringOut};
    use runnel::RunnelIoe;
    use std::io::Write;
    //
    #[test]
    fn test_t1() {
        let in_w = std::fs::read_to_string(fixture_fruit!()).unwrap();
        let (r, sioe) = do_execute!(&["--according-to", "time"], &in_w);
        assert_eq!(
            buff!(sioe, serr),
            concat!(
                program_name!(),
                ": (0,21):\'Apple:33:3.3:good:Mar\': unexpected character \'A\' while parsing time\n",
            )
        );
        assert_eq!(buff!(sioe, sout), "");
        assert!(r.is_err());
    }
    //
    #[test]
    fn test_t2() {
        let in_w = std::fs::read_to_string(fixture_fruit!()).unwrap();
        let (r, sioe) = do_execute!(&["--according-to", "time", "-r"], &in_w);
        assert_eq!(
            buff!(sioe, serr),
            concat!(
                program_name!(),
                ": (0,21):\'Apple:33:3.3:good:Mar\': unexpected character \'A\' while parsing time\n",
            )
        );
        assert_eq!(buff!(sioe, sout), "");
        assert!(r.is_err());
    }
    //
    #[test]
    fn test_t3() {
        let in_w = std::fs::read_to_string(fixture_fruit!()).unwrap();
        let (r, sioe) = do_execute!(
            &[
                "-e",
                "([0-9]+:([0-9]+:)?[0-9]+(.[0-9]+)?)",
                "--according-to",
                "time"
            ],
            &in_w,
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
        assert!(r.is_ok());
    }
    //
    #[test]
    fn test_t4() {
        let in_w = std::fs::read_to_string(fixture_fruit!()).unwrap();
        let (r, sioe) = do_execute!(
            &[
                "-e",
                "([0-9]+:([0-9]+:)?[0-9]+(.[0-9]+)?)",
                "--according-to",
                "time",
                "-r",
            ],
            &in_w,
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
        assert!(r.is_ok());
    }
    //
    #[test]
    fn test_t5() {
        let in_w = std::fs::read_to_string(fixture_fruit!()).unwrap();
        let in_w = in_w.to_string() + &in_w;
        let (r, sioe) = do_execute!(
            &[
                "-e",
                "([0-9]+:([0-9]+:)?[0-9]+(.[0-9]+)?)",
                "--according-to",
                "time"
            ],
            &in_w,
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
        assert!(r.is_ok());
    }
    //
    #[test]
    fn test_t6_header() {
        let in_w = std::fs::read_to_string(fixture_fruit_header!()).unwrap();
        let (r, sioe) = do_execute!(
            &[
                "-e",
                "([0-9]+:([0-9]+:)?[0-9]+(.[0-9]+)?)",
                "--according-to",
                "time",
                "-h",
                "1"
            ],
            &in_w,
        );
        assert_eq!(buff!(sioe, serr), "");
        assert_eq!(
            buff!(sioe, sout),
            concat!(
                "name:number:version:nice:month\n",
                "Cherry:4:4:good:Oct\n",
                "Apple:33:3.3:good:Mar\n",
                "Orange:222:1.1.2:good:Jan\n",
                "Kiwi:1111:1.1.11:good:Jun\n",
            )
        );
        assert!(r.is_ok());
    }
    //
    #[test]
    fn test_t6_footer() {
        let in_w = std::fs::read_to_string(fixture_fruit_footer!()).unwrap();
        let (r, sioe) = do_execute!(
            &[
                "-e",
                "([0-9]+:([0-9]+:)?[0-9]+(.[0-9]+)?)",
                "--according-to",
                "time",
                "-t",
                "1"
            ],
            &in_w,
        );
        assert_eq!(buff!(sioe, serr), "");
        assert_eq!(
            buff!(sioe, sout),
            concat!(
                "Cherry:4:4:good:Oct\n",
                "Apple:33:3.3:good:Mar\n",
                "Orange:222:1.1.2:good:Jan\n",
                "Kiwi:1111:1.1.11:good:Jun\n",
                "This is footer line. 1\n",
            )
        );
        assert!(r.is_ok());
    }
    //
    #[test]
    fn test_t7() {
        let in_w = std::fs::read_to_string(fixture_time!()).unwrap();
        let (r, sioe) = do_execute!(
            &[
                "-e",
                "([0-9]+:([0-9]+:)?[0-9]+(.[0-9]+)?)",
                "--according-to",
                "time",
            ],
            &in_w,
        );
        assert_eq!(buff!(sioe, serr), "");
        assert_eq!(
            buff!(sioe, sout),
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
        assert!(r.is_ok());
    }
}

mod test_1_time_color_s {
    use libaki_resort::*;
    use runnel::medium::stringio::{StringErr, StringIn, StringOut};
    use runnel::RunnelIoe;
    use std::io::Write;
    //
    #[test]
    fn test_t1() {
        let env = env_1!();
        let in_w = std::fs::read_to_string(fixture_fruit!()).unwrap();
        let (r, sioe) = do_execute!(
            &env,
            &["--according-to", "time", "--color", "always"],
            &in_w
        );
        assert_eq!(
            buff!(sioe, serr),
            concat!(
                program_name!(),
                ": (0,21):\'Apple:33:3.3:good:Mar\': unexpected character \'A\' while parsing time\n",
            )
        );
        assert_eq!(buff!(sioe, sout), "");
        assert!(r.is_err());
    }
    //
    #[test]
    fn test_t2() {
        let env = env_1!();
        let in_w = std::fs::read_to_string(fixture_fruit!()).unwrap();
        let (r, sioe) = do_execute!(
            &env,
            &["--according-to", "time", "-r", "--color", "always"],
            &in_w,
        );
        assert_eq!(
            buff!(sioe, serr),
            concat!(
                program_name!(),
                ": (0,21):\'Apple:33:3.3:good:Mar\': unexpected character \'A\' while parsing time\n",
            )
        );
        assert_eq!(buff!(sioe, sout), "");
        assert!(r.is_err());
    }
    //
    #[test]
    fn test_t3() {
        let env = env_1!();
        let in_w = std::fs::read_to_string(fixture_fruit!()).unwrap();
        let (r, sioe) = do_execute!(
            &env,
            &[
                "-e",
                "([0-9]+:([0-9]+:)?[0-9]+(.[0-9]+)?)",
                "--according-to",
                "time",
                "--color",
                "always"
            ],
            &in_w,
        );
        assert_eq!(buff!(sioe, serr), "");
        assert_eq!(
            buff!(sioe, sout),
            concat!(
                "Cherry:<S>4:4<E>:good:Oct\n",
                "Apple:<S>33:3.3<E>:good:Mar\n",
                "Orange:<S>222:1.1<E>.2:good:Jan\n",
                "Kiwi:<S>1111:1.1<E>.11:good:Jun\n",
            )
        );
        assert!(r.is_ok());
    }
    //
    #[test]
    fn test_t4() {
        let env = env_1!();
        let in_w = std::fs::read_to_string(fixture_fruit!()).unwrap();
        let (r, sioe) = do_execute!(
            &env,
            &[
                "-e",
                "([0-9]+:([0-9]+:)?[0-9]+(.[0-9]+)?)",
                "--according-to",
                "time",
                "-r",
                "--color",
                "always"
            ],
            &in_w,
        );
        assert_eq!(buff!(sioe, serr), "");
        assert_eq!(
            buff!(sioe, sout),
            concat!(
                "Kiwi:<S>1111:1.1<E>.11:good:Jun\n",
                "Orange:<S>222:1.1<E>.2:good:Jan\n",
                "Apple:<S>33:3.3<E>:good:Mar\n",
                "Cherry:<S>4:4<E>:good:Oct\n",
            )
        );
        assert!(r.is_ok());
    }
    //
    #[test]
    fn test_t5() {
        let env = env_1!();
        let in_w = std::fs::read_to_string(fixture_fruit!()).unwrap();
        let in_w = in_w.to_string() + &in_w;
        let (r, sioe) = do_execute!(
            &env,
            &[
                "-e",
                "([0-9]+:([0-9]+:)?[0-9]+(.[0-9]+)?)",
                "--according-to",
                "time",
                "--color",
                "always"
            ],
            in_w.as_str(),
        );
        assert_eq!(buff!(sioe, serr), "");
        assert_eq!(
            buff!(sioe, sout),
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
        assert!(r.is_ok());
    }
    //
    #[test]
    fn test_t6_header() {
        let env = env_1!();
        let in_w = std::fs::read_to_string(fixture_fruit_header!()).unwrap();
        let (r, sioe) = do_execute!(
            &env,
            &[
                "-e",
                "([0-9]+:([0-9]+:)?[0-9]+(.[0-9]+)?)",
                "--according-to",
                "time",
                "-h",
                "1",
                "--color",
                "always"
            ],
            in_w.as_str(),
        );
        assert_eq!(buff!(sioe, serr), "");
        assert_eq!(
            buff!(sioe, sout),
            concat!(
                "name:number:version:nice:month\n",
                "Cherry:<S>4:4<E>:good:Oct\n",
                "Apple:<S>33:3.3<E>:good:Mar\n",
                "Orange:<S>222:1.1<E>.2:good:Jan\n",
                "Kiwi:<S>1111:1.1<E>.11:good:Jun\n",
            )
        );
        assert!(r.is_ok());
    }
    //
    #[test]
    fn test_t6_footer() {
        let env = env_1!();
        let in_w = std::fs::read_to_string(fixture_fruit_footer!()).unwrap();
        let (r, sioe) = do_execute!(
            &env,
            &[
                "-e",
                "([0-9]+:([0-9]+:)?[0-9]+(.[0-9]+)?)",
                "--according-to",
                "time",
                "-t",
                "1",
                "--color",
                "always"
            ],
            in_w.as_str(),
        );
        assert_eq!(buff!(sioe, serr), "");
        assert_eq!(
            buff!(sioe, sout),
            concat!(
                "Cherry:<S>4:4<E>:good:Oct\n",
                "Apple:<S>33:3.3<E>:good:Mar\n",
                "Orange:<S>222:1.1<E>.2:good:Jan\n",
                "Kiwi:<S>1111:1.1<E>.11:good:Jun\n",
                "This is footer line. 1\n",
            )
        );
        assert!(r.is_ok());
    }
}

mod test_2_s {
    use libaki_resort::*;
    use runnel::medium::stringio::{StringErr, StringIn, StringOut};
    use runnel::RunnelIoe;
    use std::io::Write;
    //
    #[test]
    fn test_max_buffer() {
        let in_w = std::fs::read_to_string(fixture_fruit!()).unwrap();
        let (r, sioe) = do_execute!(&["--max-buffer", "20"], &in_w);
        assert_eq!(
            buff!(sioe, serr),
            concat!(program_name!(), ": over max buffer size: 20\n")
        );
        assert_eq!(buff!(sioe, sout), "");
        assert!(r.is_err());
    }
    //
    #[test]
    fn test_uniq() {
        let in_w = std::fs::read_to_string(fixture_fruit!()).unwrap();
        let in_w = in_w.to_string() + &in_w;
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
        assert!(r.is_ok());
    }
    //
    #[test]
    fn test_uniq_color() {
        let env = env_1!();
        let in_w = std::fs::read_to_string(fixture_fruit!()).unwrap();
        let in_w = in_w.to_string() + &in_w;
        let (r, sioe) = do_execute!(&env, &["-u", "--color", "always"], in_w.as_str(),);
        assert_eq!(buff!(sioe, serr), "");
        assert_eq!(
            buff!(sioe, sout),
            concat!(
                "<S>Apple:33:3.3:good:Mar<E>\n",
                "<S>Cherry:4:4:good:Oct<E>\n",
                "<S>Kiwi:1111:1.1.11:good:Jun<E>\n",
                "<S>Orange:222:1.1.2:good:Jan<E>\n",
            )
        );
        assert!(r.is_ok());
    }
}
/*
mod test_3_s {
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
mod test_4_s {
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
