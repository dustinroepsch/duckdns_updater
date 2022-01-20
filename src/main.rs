use anyhow::{Context, Error};
use duckdns_updater::options::Opt;
use duckdns_updater::run;

use log::trace;
use simplelog::{ColorChoice, CombinedLogger, Config, TermLogger, TerminalMode, WriteLogger};
use std::fs::File;
use structopt::StructOpt;

fn main() -> Result<(), Error> {
    let opt = Opt::from_args();
    let Opt {
        token,
        domain,
        interval,
        output_file_path,
        output_file_log_level,
        term_log_level,
    } = opt;

    let mut loggers: Vec<Box<dyn simplelog::SharedLogger>> = vec![TermLogger::new(
        term_log_level.into(),
        Config::default(),
        TerminalMode::Mixed,
        ColorChoice::Auto,
    )];

    if let Some(out_file) = output_file_path {
        loggers.push(WriteLogger::new(
            output_file_log_level.into(),
            Config::default(),
            File::create(&out_file).context(format!(
                "Error creating log output file with path: ({:?})",
                out_file
            ))?,
        ));
    }

    CombinedLogger::init(loggers).context("Error initializing combined logger")?;
    trace!("Initialized logging!");

    run(token, domain, interval)
}
