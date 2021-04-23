// build.rs
use rust_version_info_file::rust_version_info_file;
use rustc_version as rs_v;

fn main() {
    let rt_version = rs_v::version().unwrap();
    //
    if rt_version < rs_v::Version::parse("1.42.0").unwrap() {
        println!("cargo:rustc-cfg=has_not_matches");
    }
    //
    rust_version_info_file("target/rust-version-info.txt");
}
