use anyhow::{Context, Error};
use duckdns_updater::options::Opt;
use duckdns_updater::run;
use log::LevelFilter;
use simple_logger::SimpleLogger;
use structopt::StructOpt;

fn main() -> Result<(), Error> {
    let opt = Opt::from_args();

    SimpleLogger::new()
        .with_level(LevelFilter::Info)
        .init()
        .context("Error while creating SimpleLogger.")?;

    run(opt)
}
