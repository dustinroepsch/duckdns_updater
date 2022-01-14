use crate::duckdns_constants::{DOMAIN, TOKEN};
use anyhow::Error;
use chrono::{Utc};
use clokwerk::{Scheduler, TimeUnits};
use duckdns::DuckDns;
use std::fs::File;
use std::io::Write;
use std::thread;
use std::time::Duration;

mod duckdns_constants {
    pub const TOKEN: &str = include_str!("../token.txt");
    pub const DOMAIN: &str = "dustinpc";
}

const INTERVAL_MINUTES: u32 = 10;

fn update_dns_and_log(dns: &DuckDns, log_file: &mut File) {
    if let Err(err) = dns.update() {
        write!(log_file, "DuckDNS ({:?}) Error: {}" ,dns, err).expect("Failed to write to log.");
        println!("DuckDNS ({:?}) Error: {}", dns, err)
    } else {
        write!(
            log_file,
            "Successfully Updated DNS at time ({})",
            Utc::now()
        )
        .expect("Failed to write log.");
        println!("Successfully Updated DNS.")
    }
}

fn main() -> Result<(), Error> {
    let mut log_file = File::create("log.txt")?;
    let dns = DuckDns::new(TOKEN).domains(DOMAIN);
    let mut scheduler = Scheduler::new();
    let interval = INTERVAL_MINUTES.minutes();

    update_dns_and_log(&dns, &mut log_file);

    scheduler.every(interval).run(move || {
        update_dns_and_log(&dns, &mut log_file);
    });

    loop {
        scheduler.run_pending();
        thread::sleep(Duration::from_secs(30));
    }
}
