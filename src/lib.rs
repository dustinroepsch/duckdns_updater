#![deny(clippy::pedantic)]

pub mod options;

use anyhow::Error;

use clokwerk::{Scheduler, TimeUnits};
use duckdns::DuckDns;

use log::{error, info};

use std::thread;
use std::time::Duration;

const SCHEDULER_POLLING_INTERVAL: Duration = Duration::from_secs(30);

fn update_dns_and_log(dns: &DuckDns, domain: &str) {
    if let Err(err) = dns.update() {
        error!("DuckDNS ({:?}) Error: {}", dns, err);
    } else {
        info!("Successfully updated DNS for {}.duckdns.org", domain);
    }
}

#[allow(clippy::missing_errors_doc)]
pub fn run(token: String, domain: String, interval: u32) -> Result<(), Error> {
    let dns = DuckDns::new(token).domains(&domain);
    let mut scheduler = Scheduler::new();
    let interval = interval.minutes();

    update_dns_and_log(&dns, &domain);

    scheduler.every(interval).run(move || {
        update_dns_and_log(&dns, &domain);
    });

    loop {
        scheduler.run_pending();
        thread::sleep(SCHEDULER_POLLING_INTERVAL);
    }
}
