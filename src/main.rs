use libaki_resort::execute;
use runnel::RunnelIoeBuilder;

fn main() {
    // fast mem operation.
    #[cfg(not(miri))]
    memx_cdy::memx_init();
    //
    let mut env_args: Vec<String> = std::env::args().collect();
    let _program = env_args.remove(0);
    let program = env!("CARGO_PKG_NAME");
    let env_args: Vec<&str> = env_args.iter().map(String::as_str).collect();
    //
    let sioe = RunnelIoeBuilder::new().build();
    //
    if let Err(err) = execute(&sioe, program, &env_args) {
        let _ = sioe.pg_err().write_line(format!("{program}: {err:#}"));
        std::process::exit(1);
    };
}
