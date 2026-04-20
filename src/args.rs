use std::fmt::Debug;

use clap::{Parser, Subcommand, ValueEnum};

#[derive(Parser)]
#[command(name = "sobinary", version, about, long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
    #[arg(short, long, default_value_t = DebugLevel::Info)]
    pub debug: DebugLevel,
    #[arg(short, long)]
    pub sysroot: Vec<String>,
    #[arg(short, long, default_value_t = false)]
    pub json: bool,
}

#[derive(ValueEnum, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum DebugLevel {
    Trace,
    Debug,
    Info,
    Warn,
    Error,
}

impl ToString for DebugLevel {
    fn to_string(&self) -> String {
        match self {
            DebugLevel::Trace => "trace".to_string(),
            DebugLevel::Debug => "debug".to_string(),
            DebugLevel::Info => "info".to_string(),
            DebugLevel::Warn => "warn".to_string(),
            DebugLevel::Error => "error".to_string(),
        }
    }
}

#[derive(Subcommand)]
pub enum Commands {
    Summary,
    FindDependency {
        #[arg(short, long)]
        name: String,
    }
}
