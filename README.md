## DuckDNS Auto Updater

This is a small tool written in rust that will periodically point your duckdns domain to the pc running this program. I use it! 

## Building

It's a regular old rust program. Build it with:
`cargo build --release`

## Usage

```
duckdns_updater 2.0.0
Update the given `DuckDNS` domain to the ip address of the machine running this binary on a given interval

USAGE:
    duckdns_updater.exe [OPTIONS] --domain <domain> --token <token>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -d, --domain <domain>        The DuckDNS domain you wish to update
    -i, --interval <interval>    The DuckDNS domain's IP will be updated every `interval` minutes [default: 3]
    -t, --token <token>          Your DuckDNS api token. Get one here: https://www.duckdns.org/
```
