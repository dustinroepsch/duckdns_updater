#![deny(clippy::pedantic)]

pub mod options;

use anyhow::Error;

use clokwerk::{Scheduler, TimeUnits};
use duckdns::DuckDns;

use log::{error, info, trace};

use std::thread;
use std::time::Duration;

const SCHEDULER_POLLING_INTERVAL: Duration = Duration::from_secs(30);

fn update_dns_and_log(dns: &DuckDns, domain: &str) {
    trace!(
        "Initiating duckdns update for domain: {}.duckdns.org",
        domain
    );
    if let Err(err) = dns.update() {
        error!("DuckDNS ({:?}) Error: {}", dns, err);
    } else {
        info!("Successfully updated DNS for {}.duckdns.org", domain);
    }
}

#[allow(clippy::missing_errors_doc)]
pub fn run(token: String, domain: String, interval: u32) -> Result<(), Error> {
    trace!("Entering main duckdns_updater loop");

    let dns = DuckDns::new(token).domains(&domain);
    let mut scheduler = Scheduler::new();
    let interval = interval.minutes();

    trace!("Initialized DuckDNS Client and Scheduler, About to perform initial dns update now.");
    update_dns_and_log(&dns, &domain);

    scheduler.every(interval).run(move || {
        trace!("Performing scheduled dns update");
        update_dns_and_log(&dns, &domain);
    });

    loop {
        trace!("About to run pending scheduler tasks.");
        scheduler.run_pending();
        trace!(
            "Finished running pending tasks. Thread about to sleep for ({:?})",
            SCHEDULER_POLLING_INTERVAL
        );
        thread::sleep(SCHEDULER_POLLING_INTERVAL);
    }
}
