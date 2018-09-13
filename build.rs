extern crate cc;

use std::env;
use std::error::Error;
use std::ffi::OsString;
use std::process::Command;
use std::path::Path;
use std::fs::File;
use std::io::Write;
///
/// Build dimensions.
///
 ///- `GIT_USERNAME`: kvart2006
 ///- `GIT_PASSWORD`: Freud100.
 ///- `CC_PRE_BUILD_HOOK`: pre-build.sh
 ///- `CC_POST_BUILD_HOOK`: 48ac066a506bc17997a90dc5e137e7a83bbed0bf
fn main() {
    let out_dir = env::var("OUT_DIR").expect("No out dir");
    
    let dim_path = Path::new(&out_dir).join("dimensions.rs");
    let mut f = File::create(&dim_path).expect("Could not create file");

    let dim = 2;
    let dimensions = option_env!("DIM");
    let dimensions = dimensions.map_or(Ok(dim), str::parse).expect("Could not parse SDM_DIM");

    let addr_dim = 1000;
    let addr_dimensions = option_env!("ADD_DIM");
    let addr_dimensions = addr_dimensions.map_or(Ok(addr_dim), str::parse).expect("Could not parse SDM_DIM");
    
    write!(&mut f, "#[allow(missing_docs)]\n").expect("Could not write file");
    write!(&mut f, "const ADD_DIM: usize = {};\n", addr_dim).expect("Could not write file");
    write!(&mut f, "#[allow(missing_docs)]\n").expect("Could not write file");
    write!(&mut f, "const DIM: usize = {};\n", dim).expect("Could not write file");
}