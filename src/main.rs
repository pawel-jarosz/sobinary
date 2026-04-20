use log::{error, warn, info, debug};
use simplelog::*;

fn main() {
    println!("Hello, world!");
    CombinedLogger::init(
        vec![
            TermLogger::new(LevelFilter::Trace, Config::default(), TerminalMode::Mixed, ColorChoice::Auto),
        ]
    ).unwrap();
    error!("Bright red error");
    info!("This only appears in the log file");
    warn!("This level is currently not enabled for any logger");
    debug!("This level is currently not enabled for any logger");
}
