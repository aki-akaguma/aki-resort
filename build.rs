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
    let path = {
        #[cfg(feature = "debian_build")]
        let dir = "target".to_string();
        #[cfg(not(feature = "debian_build"))]
        let dir = std::env::var("OUT_DIR").unwrap();
        //
        format!("{dir}/rust-version-info.txt")
    };
    rust_version_info_file(path.as_str(), "Cargo.toml");
}
