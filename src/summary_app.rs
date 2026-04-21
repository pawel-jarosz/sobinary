use log::{debug, error, info, trace, warn};

use sobinary::filesystem::ElfFinder;

pub fn summary_main(sysroots: &[String]) {
    info!("Summary");
    info!("Using sysroots: {}", sysroots.join(", "));

    let traveler = ElfFinder::new();
    for sysroot in sysroots {
        traveler.find_files(sysroot);
    }
}