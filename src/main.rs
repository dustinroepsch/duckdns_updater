use crate::duckdns_constants::{DOMAIN, TOKEN};
use clokwerk::{Scheduler, TimeUnits};
use duckdns::DuckDns;
use std::thread;
use std::time::Duration;

mod duckdns_constants {
    pub const TOKEN: &str = include_str!("../token.txt");
    pub const DOMAIN: &str = "dustinpc";
}

const INTERVAL_MINUTES: u32 = 10;

fn main() {
    let dns = DuckDns::new(TOKEN).domains(DOMAIN);
    let mut scheduler = Scheduler::new();
    let interval = INTERVAL_MINUTES.minutes();

    scheduler.every(interval).run(move || {
        if let Err(err) = dns.update() {
            eprintln!("Error updating dns: {}", err);
        } else {
            println!("Successfully updated dns!")
        }
    });

    loop {
        scheduler.run_pending();
        thread::sleep(Duration::from_secs(30));
    }
}
