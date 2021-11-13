use mac_address::MacAddress;
use std::str::FromStr;
use wolrs::{MagicPacket, Result};

use clap::Parser;

/// A basic example
#[derive(Parser, Debug)]
#[clap(version = "1.0.0")]
struct Opt {
    /// Output file
    #[clap()]
    addr: String,
}

fn main() -> Result<()> {
    let opt = Opt::parse();
    let mac_addr = MacAddress::from_str(&opt.addr)?;
    MagicPacket::new(mac_addr).send()?;
    Ok(())
}
