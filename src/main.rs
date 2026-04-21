use log::{debug, error, info, trace, warn};
use simplelog::*;
use clap::Parser;

mod args;
mod dependency_finder_app;
mod summary_app;

fn configure_logger(logger_level: args::DebugLevel) {
    let level_filter = match logger_level {
        args::DebugLevel::Trace => LevelFilter::Trace,
        args::DebugLevel::Debug => LevelFilter::Debug,
        args::DebugLevel::Info => LevelFilter::Info,
        args::DebugLevel::Warn => LevelFilter::Warn,
        args::DebugLevel::Error =>LevelFilter::Error,
    };

    CombinedLogger::init(
        vec![
            TermLogger::new(level_filter, Config::default(), TerminalMode::Mixed, ColorChoice::Auto),
        ]
    ).unwrap();
}

fn main() {
    let cli = args::Cli::parse();
    configure_logger(cli.debug);

    match cli.command {
        args::Commands::Summary => {
            summary_app::summary_main(&cli.sysroot);
        },
        args::Commands::FindDependency { name } => {
            dependency_finder_app::dependency_finder_main(&name, &cli.sysroot);
        }       
    }
}
