use anyhow::Error;
use duckdns_updater::run;

fn main() -> Result<(), Error> {
    run()
}
