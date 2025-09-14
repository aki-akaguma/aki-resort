#[allow(unused_macros)]
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
                let _ = sioe.pg_err().write_line(format!("{program}: {err:#}"));
            }
        };
        (r, sioe)
    }};
    ($env:expr, $args:expr, $sin:expr,) => {
        do_execute!($env, $args, $sin)
    };
    ($env:expr, $args:expr, $sin:expr) => {{
        let sioe = RunnelIoe::new(
            Box::new(StringIn::with_str($sin)),
            #[allow(clippy::box_default)]
            Box::new(StringOut::default()),
            #[allow(clippy::box_default)]
            Box::new(StringErr::default()),
        );
        let program = env!("CARGO_PKG_NAME");
        let r = execute_with_env(&sioe, &program, $args, $env);
        match r {
            Ok(_) => {}
            Err(ref err) => {
                let _ = sioe.pg_err().write_line(format!("{program}: {err:#}"));
            }
        };
        (r, sioe)
    }};
}

#[allow(unused_macros)]
macro_rules! buff {
    ($sioe:expr, serr) => {
        $sioe.pg_err().lock().buffer_to_string()
    };
    ($sioe:expr, sout) => {
        $sioe.pg_out().lock().buffer_to_string()
    };
}
