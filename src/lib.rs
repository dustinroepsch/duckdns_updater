#![deny(clippy::pedantic)]

pub mod options;
pub mod util;

use crate::options::{DOMAIN, INTERVAL_MINUTES, LOG_FILE_LOCATION, TOKEN};
use crate::util::warn_error;
use anyhow::{Context, Error};
use chrono::Utc;
use clokwerk::{Scheduler, TimeUnits};
use duckdns::DuckDns;

use std::fs::File;
use std::io::Write;
use std::thread;
use std::time::Duration;

fn update_dns_and_log(dns: &DuckDns, log_file: &mut File) -> Result<(), Error> {
    if let Err(err) = dns.update() {
        write!(log_file, "DuckDNS ({:?}) Error: {}", dns, err)
            .context("Error writing to log file")?;
        println!("DuckDNS ({:?}) Error: {}", dns, err);
    } else {
        let time = Utc::now();

        writeln!(log_file, "Successfully Updated DNS at UTC: ({})", time)
            .expect("Failed to write log.");

        println!("Successfully Updated DNS at UTC: ({})", time);
    }

    Ok(())
}

#[allow(clippy::missing_errors_doc)]
pub fn run() -> Result<(), Error> {
    let mut log_file = File::create(LOG_FILE_LOCATION).context(format!(
        "Error creating/truncating the log file at ({})",
        LOG_FILE_LOCATION
    ))?;

    let dns = DuckDns::new(TOKEN).domains(DOMAIN);
    let mut scheduler = Scheduler::new();
    let interval = INTERVAL_MINUTES.minutes();

    if let Err(err) = update_dns_and_log(&dns, &mut log_file) {
        warn_error(&err);
    }

    scheduler.every(interval).run(move || {
        if let Err(err) = update_dns_and_log(&dns, &mut log_file) {
            warn_error(&err);
        }
    });

    loop {
        scheduler.run_pending();
        thread::sleep(Duration::from_secs(30));
    }
}
