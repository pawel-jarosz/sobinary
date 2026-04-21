use log::{error, warn, info, debug};
use sobinary::filesystem::ElfFinder;


pub fn dependency_finder_main(input: &str, sysroots: &[String]) {
    info!("Finding dependencies for: {}", input);
    info!("Using sysroots: {}", sysroots.join(", "));

    let traveler = ElfFinder::new();
    for sysroot in sysroots {
        traveler.find_files(sysroot);
    }
}