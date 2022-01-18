use anyhow::{Context, Error};
use duckdns_updater::run;
use log::LevelFilter;
use simple_logger::SimpleLogger;

fn main() -> Result<(), Error> {
    SimpleLogger::new()
        .with_level(LevelFilter::Info)
        .init()
        .context("Error while creating SimpleLogger.")?;
    run()
}
