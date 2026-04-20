use log::{error, warn, info, debug};


pub fn dependency_finder_main(input: &str, sysroot: &[String]) {
    info!("Finding dependencies for: {}", input);
    info!("Using sysroots: {}", sysroot.join(", "));
}