use libaki_resort::execute;
use runnel::RunnelIoeBuilder;
use std::io::Write;

fn main() {
    memx_cdy::memx_init(); // fast mem operation.
    //
    let mut env_args: Vec<String> = std::env::args().collect();
    let _program = env_args.remove(0);
    let program = env!("CARGO_PKG_NAME");
    let env_args: Vec<&str> = env_args.iter().map(std::string::String::as_str).collect();
    //
    let sioe = RunnelIoeBuilder::new().build();
    //
    match execute(&sioe, &program, &env_args) {
        Ok(_) => {}
        Err(err) => {
            #[rustfmt::skip]
            let _ = sioe.perr().lock()
                .write_fmt(format_args!("{}: {:#}\n", program, err));
            std::process::exit(1);
        }
    };
    //
    std::process::exit(0);
}
