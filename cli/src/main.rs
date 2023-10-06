use std::{fs, net::IpAddr};

use clap::Parser;
use ipnetwork::IpNetwork;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// The graphql query to run
    query: String,

    /// Specify a subnet in CIDR notation (e.g., 10.0.9.0/24)
    #[clap(short, long)]
    subnets: Option<Vec<String>>,

    /// Specify one or more host IP addresses
    #[clap(short, long)]
    inventory: Option<String>,
}

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();
    println!(
        "subnets: {:?} - inventory_file: {:?} - query: {:?}",
        cli.subnets.as_deref(),
        cli.inventory.as_deref(),
        cli.query
    );

    let inventory_contents = match cli.inventory {
        Some(fp) => Some(fs::read_to_string(fp)?),
        None => None,
    };
    let ips = generate_ips(cli.subnets.unwrap_or(vec![]), inventory_contents)?;
    println!("{:?}", ips);
    Ok(())
}

fn generate_ips(subnets: Vec<String>, inventory: Option<String>) -> anyhow::Result<Vec<IpAddr>> {
    let mut collector: Vec<IpAddr> = vec![];
    // Generate ips from subnets.
    parse_subnets(&mut collector, subnets);
    // Generate from inventory file.
    if let Some(inv) = inventory {
        parse_subnets(
            &mut collector,
            inv.trim().split('\n').map(|s| s.to_string()), // This is ugly
        );
    }

    collector.sort();
    collector.dedup();
    Ok(collector)
}

fn parse_subnets<I>(collector: &mut Vec<IpAddr>, subnets: I)
where
    I: IntoIterator<Item = String>,
{
    for ip in subnets {
        let net: IpNetwork = match ip.parse() {
            Ok(addr) => addr,
            Err(e) => {
                println!("{}", e);
                continue;
            }
        };
        collector.extend(net.iter())
    }
}

#[cfg(test)]
mod tests {
    // use super::*;

    #[test]
    fn whitespaced_file_parses_to_ipaddrs() {
        todo!();
    }
}
