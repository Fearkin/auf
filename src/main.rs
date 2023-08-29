use clap::Parser;
use std::{
    net::{IpAddr, Ipv4Addr},
    str::FromStr,
};
use trust_dns_client::{
    error::ClientError,
    client::{Client, SyncClient},
    rr::{DNSClass, Name, RecordType, Record},
    udp::UdpClientConnection,
};
use trust_dns_resolver::{
    config::{ResolverConfig, ResolverOpts},
    Resolver,
};
use whois_rust::{WhoIs, WhoIsLookupOptions};

use crate::report::Report;
mod report;

#[derive(Parser)]
#[command(name = "auf")]
#[command(version = "0.1.0")]
#[command(
    about = "",
    long_about = "Cross-platform console application for viewing DNS records for a domain, and more!"
)]
struct Cli {
    #[arg(help = "Domain name to lookup")]
    domain: Name,
    #[arg(
        help = "DNS-resolver to use (default is Quad9 - 9.9.9.9)",
        default_value = "9.9.9.9"
    )]
    resolver: Ipv4Addr,
}

fn get_whois_info(domain: Name) -> String {
    //FIXME: remove `expect`
    let domain = domain.to_string();
    let whois =
        WhoIs::from_path("servers.json").expect("Unable to locate required file \"server.json\"");
    whois
        .lookup(WhoIsLookupOptions::from_string(domain).unwrap())
        .expect("No whois server is known for this kind of object")
}

fn query_records(client: &SyncClient<UdpClientConnection>, domain: &Name, record_type: RecordType) -> Result<Vec<Record>, ClientError>{
    let mut records: Vec<Record> = vec![];
    //TODO: add support for other DNS classes
    match client.query(domain, DNSClass::IN, record_type) {
        Ok(response) => {
            println!("Querying {} records:", record_type);
            for record in response.answers() {
                records.push(record.clone());
            }
            Ok(records)
        }
        Err(err) => Err(err)
    }
}

fn reverse_zone(ip_address: IpAddr) -> Name {
    //TODO: remove `unwrap`
    match ip_address {
        IpAddr::V4(ipv4) => {
            let octets: Vec<String> = ipv4.octets().iter().map(|&byte| byte.to_string()).collect();
            let reversed_octets = octets.into_iter().rev().collect::<Vec<_>>().join(".");
            Name::from_str(&format!("{}.in-addr.arpa.", reversed_octets)).unwrap()
        }
        IpAddr::V6(ipv6) => {
            let segments: Vec<String> = ipv6
                .segments()
                .iter()
                .map(|&segment| format!("{:x}", segment))
                .collect();
            let reversed_segments = segments.into_iter().rev().collect::<Vec<_>>().join(".");
            Name::from_str(&format!("{}.ip6.arpa.", reversed_segments)).unwrap()
        }
    }
}

fn main() {
    let cli = Cli::parse();
    let resolver = cli.resolver;
    let domain = cli.domain;
    //FIXME: domain names should always end with dot
    let record_types = [
        RecordType::A,
        RecordType::AAAA,
        RecordType::CNAME,
        RecordType::MX,
        RecordType::NS,
        RecordType::TXT,
        RecordType::CAA,
        RecordType::PTR,
        RecordType::SOA,
        RecordType::SRV,
    ];

    let resolver_addr = format!("{}:53", resolver)
        .parse()
        .expect("Failed to parse resolver address");
    let conn = UdpClientConnection::new(resolver_addr).unwrap();
    let client = SyncClient::new(conn);

    /*  
        FIXME: Refactor usage of trust_dns_resolver and trust_dns_client, better to use only one of them if possible
        TODO: remove `unwrap`
    */
    let mut records: Vec<Record> = vec![];
    for record_type in record_types {
        let mut domain = domain.clone();
        if record_type == RecordType::PTR {
            let resolver = Resolver::new(ResolverConfig::quad9(), ResolverOpts::default()).unwrap();
            let response = resolver.lookup_ip(domain.to_string()).unwrap();
            let address = response.iter().next().expect("no addresses returned!");
            domain = reverse_zone(address);
        }
        match query_records(&client, &domain, record_type) {
            Ok(mut vec) => {
                records.append(&mut vec);
            }
            Err(_err) => ()

        }
    }


    let whois_info = get_whois_info(domain);
    println!("{}", Report{whois_info, records})
    //  TODO: implement a nicer
}
