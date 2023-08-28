use clap::Parser;
use std::net::{Ipv4Addr, SocketAddr};
use trust_dns_client::{udp::UdpClientConnection, client::{SyncClient, Client}, rr::{Name, DNSClass, RecordType, Record}};
use whois_rust::{WhoIs, WhoIsLookupOptions};


#[derive(Parser)]
#[command(name = "auf")]
#[command(version = "0.1.0")]
#[command(about ="", long_about="Cross-platform console application for viewing DNS records for a domain, and more!")]
struct Cli {
    #[arg(short, long, help="Domain name to lookup")]
    domain: Name,
    #[arg(short, long, help="DNS-resolver to use (default is 1.1.1.1)", default_value = "1.1.1.1")]
    resolver: Ipv4Addr,
    /*#[arg(short, long, help="Get information on what technologies site is based on")]
    tech: String,*/
}

fn get_whois_info(domain: Name) -> String {
    //TODO: refactor `expect`
    let domain = domain.to_string();
    let whois = WhoIs::from_path("servers.json").expect("Unable to locate required file \"server.json\"");
    let result = whois.lookup(WhoIsLookupOptions::from_string(domain).unwrap()).expect("No whois server is known for this kind of object");
    result
}


fn query_records(client: &SyncClient<UdpClientConnection>, domain: &Name, record_type: RecordType){
    match client.query(domain, DNSClass::IN, record_type) {
        Ok(response) => {
            println!("Querying {} records:", record_type);
            for record in response.answers() {
                println!("{}", record);
            }
        }
        Err(err) => {
            eprintln!("Error querying {} records: {:?}", record_type, err);
        }
    }
}


fn main() {
    let cli = Cli::parse();
    let resolver = cli.resolver;
    let domain = cli.domain;
    let record_types = [
        RecordType::A,
        RecordType::AAAA,
        RecordType::CNAME,
        RecordType::MX,
        RecordType::NS,
        RecordType::TXT,
        RecordType::CAA,
        RecordType::DS,
        RecordType::PTR,
        RecordType::SOA,
        RecordType::SRV,
    ];

    let resolver_ip = format!("{}:53", resolver).parse().expect("Failed to parse resolver address");
    let conn = UdpClientConnection::new(resolver_ip).unwrap();
    let client = SyncClient::new(conn);
    
    for record_type in record_types {
        query_records(&client, &domain, record_type);
    }

    println!("{}", get_whois_info(domain));

}