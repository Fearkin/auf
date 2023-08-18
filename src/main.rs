use clap::Parser;
use whois_rust::{WhoIs, WhoIsLookupOptions};

#[derive(Parser)]
#[command(name = "auf")]
#[command(version = "0.1.0")]
#[command(about ="", long_about="Cross-platform console application for viewing DNS records for a domain, and more!")]
struct Cli {
    #[arg(short, long, help="Domain name to lookup")]
    domain: Option<String>,
    /*#[arg(short, long, help="DNS-resolver to use (default is 1.1.1.1)")]
    resolver: String,
    #[arg(short, long, help="Get information on what technologies site is based on")]
    tech: String,*/
}

fn main() {
    let cli = Cli::parse();
    let whois = WhoIs::from_path("servers.json").unwrap();
    if let Some(domain) = cli.domain.as_deref() {
        let result: String = whois.lookup(WhoIsLookupOptions::from_string(domain).unwrap()).unwrap();
        print!("{result}");
    }

}