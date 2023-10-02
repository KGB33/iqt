use clap::Parser;

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

fn main() {
    let cli = Cli::parse();
    println!(
        "subnets: {:?} - inventory_file: {:?} - query: {:?}",
        cli.subnets.as_deref(),
        cli.inventory.as_deref(),
        cli.query
    )
}
