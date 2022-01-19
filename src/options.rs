use std::path::PathBuf;
use structopt::StructOpt;

pub mod log {
    use anyhow::{anyhow, Error};
    use log::LevelFilter;
    use std::str::FromStr;

    #[derive(Debug, Clone)]
    pub enum Level {
        // Nothing
        Off,
        // Everything
        Trace,
        /// Everything except trace logs
        Info,
        /// Everything except trace and Info logs
        Warn,
        /// Only error logs
        Error,
    }

    impl FromStr for Level {
        type Err = Error;

        fn from_str(s: &str) -> Result<Self, Self::Err> {
            match s {
                "off" => Ok(Self::Off),
                "trace" => Ok(Self::Trace),
                "info" => Ok(Self::Info),
                "warn" => Ok(Self::Warn),
                "error" => Ok(Self::Error),
                &_ => Err(anyhow!("Invalid Log Level")),
            }
        }
    }

    impl From<Level> for LevelFilter {
        fn from(level: Level) -> Self {
            match level {
                Level::Off => LevelFilter::Off,
                Level::Trace => LevelFilter::Trace,
                Level::Info => LevelFilter::Info,
                Level::Warn => LevelFilter::Warn,
                Level::Error => LevelFilter::Error,
            }
        }
    }
}

/// Update the given `DuckDNS` domain to the ip address of the machine running this binary on a given interval.
#[derive(Debug, StructOpt, Clone)]
pub struct Opt {
    /// The DuckDNS domain you wish to update.
    #[structopt(long, short)]
    pub domain: String,
    /// The DuckDNS domain's IP will be updated every `interval` minutes.
    #[structopt(long, short, default_value = "3")]
    pub interval: u32,

    /// Your DuckDNS api token. Get one here: https://www.duckdns.org/
    #[structopt(long, short)]
    pub token: String,

    #[structopt(long, default_value = "off")]
    /// The log level for the file output, valid options are: (off, trace, info, warn, error)
    pub output_file_log_level: log::Level,

    #[structopt(long, required_unless_one =  &["output-file-log-level", "off"])]
    /// The given file (if any) will be used to store logs
    pub output_file_path: Option<PathBuf>,

    /// The log level for the terminal output, valid options are: (off, trace, info, warn, error)
    #[structopt(long, default_value = "info")]
    pub term_log_level: log::Level,
}
