// build.rs
use rustc_version as rs_v;

fn main() {
    let rt_version = rs_v::version().unwrap();
    //
    if rt_version < rs_v::Version::parse("1.42.0").unwrap() {
        println!("cargo:rustc-cfg=has_not_matches");
    }
}
