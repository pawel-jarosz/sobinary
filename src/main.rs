use log::{debug, error, info, trace, warn};
use simplelog::*;
use clap::Parser;

mod args;
mod dependency_finder;

fn main() {
    println!("Hello, world!");
    CombinedLogger::init(
        vec![
            TermLogger::new(LevelFilter::Trace, Config::default(), TerminalMode::Mixed, ColorChoice::Auto),
        ]
    ).unwrap();

    let cli = args::Cli::parse();
    match cli.debug {
        args::DebugLevel::Trace => log::set_max_level(LevelFilter::Trace),
        args::DebugLevel::Debug => log::set_max_level(LevelFilter::Debug),
        args::DebugLevel::Info => log::set_max_level(LevelFilter::Info),
        args::DebugLevel::Warn => log::set_max_level(LevelFilter::Warn),
        args::DebugLevel::Error => log::set_max_level(LevelFilter::Error),
    }

    match cli.command {
        args::Commands::Summary => {
            info!("Generating summary...");
        },
        args::Commands::FindDependency { name } => {
            dependency_finder::dependency_finder_main(&name, &cli.sysroot);
        }       
    }
}
