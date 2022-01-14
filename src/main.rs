use crate::duckdns_constants::{DOMAIN, INTERVAL_MINUTES, LOG_FILE_LOCATION, TOKEN};
use anyhow::{Context, Error};
use chrono::Utc;
use clokwerk::{Scheduler, TimeUnits};
use duckdns::DuckDns;
use std::fs::File;
use std::io::Write;
use std::thread;
use std::time::Duration;

mod duckdns_constants {
    pub const TOKEN: &str = include_str!("../token.txt");
    pub const DOMAIN: &str = "dustinpc";
    pub const LOG_FILE_LOCATION: &str = "C:\\Program Files\\DuckDNSUpdater\\log.txt";
    pub const INTERVAL_MINUTES: u32 = 3;
}

fn update_dns_and_log(dns: &DuckDns, log_file: &mut File) -> Result<(), Error> {
    if let Err(err) = dns.update() {
        write!(log_file, "DuckDNS ({:?}) Error: {}", dns, err)
            .context("Error writing to log file")?;
        println!("DuckDNS ({:?}) Error: {}", dns, err)
    } else {
        let time = Utc::now();

        writeln!(log_file, "Successfully Updated DNS at UTC: ({})", time)
            .expect("Failed to write log.");

        println!("Successfully Updated DNS at UTC: ({})", time);
    }

    Ok(())
}

fn warn_error(err: Error) {
    println!("Warning!! Error encountered: {}", err);
}

fn main() -> Result<(), Error> {
    let mut log_file = File::create(LOG_FILE_LOCATION).context(format!(
        "Error creating/truncating the log file at ({})",
        LOG_FILE_LOCATION
    ))?;
    let dns = DuckDns::new(TOKEN).domains(DOMAIN);
    let mut scheduler = Scheduler::new();
    let interval = INTERVAL_MINUTES.minutes();

    if let Err(err) = update_dns_and_log(&dns, &mut log_file) {
        warn_error(err);
    }

    scheduler.every(interval).run(move || {
        if let Err(err) = update_dns_and_log(&dns, &mut log_file) {
            warn_error(err);
        }
    });

    loop {
        scheduler.run_pending();
        thread::sleep(Duration::from_secs(30));
    }
}
