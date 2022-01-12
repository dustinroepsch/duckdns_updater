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

fn update_dns_and_print_msg(dns: &DuckDns) {
    if let Err(err) = dns.update() {
        eprintln!("Error updating dns: {}", err);
    } else {
        println!("Successfully updated dns!")
    }
}

fn main() {
    let dns = DuckDns::new(TOKEN).domains(DOMAIN);
    let mut scheduler = Scheduler::new();
    let interval = INTERVAL_MINUTES.minutes();

    update_dns_and_print_msg(&dns);

    scheduler.every(interval).run(move || {
        update_dns_and_print_msg(&dns);
    });

    loop {
        scheduler.run_pending();
        thread::sleep(Duration::from_secs(30));
    }
}
