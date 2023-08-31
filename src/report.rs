use std::fmt::{Display, Formatter, self};

use trust_dns_client::rr::Record;

pub struct Report {
    pub whois_info: String,
    pub records: Vec<Record>,
}

impl Display for Report {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        for record in &self.records {
            writeln!(f,
                "Found {}", record)?;
        }
        writeln!(f, "\nWHOIS Info:\n{}", self.whois_info)?;
        Ok(())
    }
}

