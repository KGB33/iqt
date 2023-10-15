use std::{fs, net::IpAddr};

use anyhow::anyhow;
use miette::{Diagnostic, Report, SourceSpan};
use thiserror::Error;

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

    /// Specify a file to read subnets from
    #[clap(short, long)]
    inventory: Option<String>,

    /// Specify one or more hostnames
    #[clap(short, long)]
    fqdn: Option<Vec<String>>,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();
    eprintln!(
        "subnets: {:?} - inventory_file: {:?} - query: {:?}",
        cli.subnets.as_deref(),
        cli.inventory.as_deref(),
        cli.query
    );

    if !verify_query(cli.query.clone()) {
        return Err(anyhow!("Query Verification Failed!"));
    }

    let inventory_contents = match cli.inventory {
        Some(fp) => Some(fs::read_to_string(fp)?),
        None => None,
    };
    let ips = generate_ips(cli.subnets.unwrap_or(vec![]), inventory_contents)?;
    let urls = generate_urls(ips, cli.fqdn);
    let client = reqwest::Client::new();
    for url in urls {
        let res = client
            .post(url)
            .body(format!(r#"{{ "query": "{}" }}"#, cli.query.clone()))
            .send()
            .await?;
        println!("{}", res.text().await?)
    }
    Ok(())
}

fn generate_urls(ips: Vec<IpAddr>, hostnames: Option<Vec<String>>) -> Vec<String> {
    let mut collector: Vec<String> = vec![];
    for ip in ips {
        collector.push(format!("http://{}:4807/graphql", ip));
    }
    for host in hostnames.unwrap_or_default() {
        collector.push(format!("http://{}:4807/graphql", host));
    }
    collector
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

fn verify_query(query: String) -> bool {
    let parser = apollo_parser::Parser::new(&query);
    let ast = parser.parse();

    #[derive(Error, Debug, Diagnostic)]
    #[error("{}", self.ty)]
    #[diagnostic(code("apollo-parser parsing error."))]
    struct ApolloParserError {
        ty: String,
        #[source_code]
        src: String,
        #[label("{}", self.ty)]
        span: SourceSpan,
    }

    let mut ok = true;
    for err in ast.errors() {
        let err = Report::new(ApolloParserError {
            src: query.clone(),
            span: (err.index(), err.data().len()).into(), // (offset, length of error token)
            ty: err.message().into(),
        });
        eprintln!("{:?}", err);
        ok = false;
    }
    ok
}

#[cfg(test)]
mod tests {
    // use super::*;

    #[test]
    fn whitespaced_file_parses_to_ipaddrs() {
        todo!();
    }
}
