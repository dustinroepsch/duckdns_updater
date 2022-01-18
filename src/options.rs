use structopt::StructOpt;

/// Update the given `DuckDNS` domain to the ip address of the machine running this binary on a given interval.
#[derive(Debug, StructOpt, Clone)]
pub struct Opt {
    /// Your DuckDNS api token. Get one here: https://www.duckdns.org/
    #[structopt(long, short)]
    pub token: String,
    /// The DuckDNS domain you wish to update.
    #[structopt(long, short)]
    pub domain: String,
    /// The DuckDNS domain's IP will be updated every `interval` minutes.
    #[structopt(long, short, default_value = "3")]
    pub interval: u32,
}
