use clap::Parser;
use std::path::PathBuf;

/// DevTools MCP Server
#[derive(Parser)]
#[command(version, about, long_about = None)]
pub struct Args {
    /// Custom kv pair config file
    ///
    /// if "config.yaml" is existed in the same directory, it will be used as the default config.
    #[arg(short, long, value_name = "FILE")]
    pub config: Option<PathBuf>,

    /// Log config str
    ///
    /// if file does not exist, logging will be disabled.
    /// config str format:
    /// [[level][@[dir][/filename_prefix.][filename_suffix][:[rotation]#[max_log_files]]]]
    /// example:
    /// Info@log/default.log:Daily#5
    #[arg(short, long, value_name = "LOG_STR", default_value = "")]
    pub log_str: String,
}
