#![deny(clippy::pedantic)]

pub mod options;

use crate::options::{DOMAIN, INTERVAL_MINUTES, TOKEN};
use anyhow::Error;

use clokwerk::{Scheduler, TimeUnits};
use duckdns::DuckDns;

use log::{error, info};

use std::thread;
use std::time::Duration;

const SCHEDULER_POLLING_INTERVAL: Duration = Duration::from_secs(30);

fn update_dns_and_log(dns: &DuckDns) {
    if let Err(err) = dns.update() {
        error!("DuckDNS ({:?}) Error: {}", dns, err);
    } else {
        info!("Successfully updated DNS for {}.duckdns.org", DOMAIN);
    }
}

#[allow(clippy::missing_errors_doc)]
pub fn run() -> Result<(), Error> {
    let dns = DuckDns::new(TOKEN).domains(DOMAIN);
    let mut scheduler = Scheduler::new();
    let interval = INTERVAL_MINUTES.minutes();

    update_dns_and_log(&dns);

    scheduler.every(interval).run(move || {
        update_dns_and_log(&dns);
    });

    loop {
        scheduler.run_pending();
        thread::sleep(SCHEDULER_POLLING_INTERVAL);
    }
}
