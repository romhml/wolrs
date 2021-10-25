use mac_address::MacAddress;
use std::str::FromStr;
use wolrs::MagicPacket;

use structopt::StructOpt;

/// A basic example
#[derive(StructOpt, Debug)]
#[structopt(name = "wolrs")]
struct Opt {
    /// Output file
    #[structopt()]
    addr: String,
}

fn main() {
    let opt = Opt::from_args();
    match MacAddress::from_str(&opt.addr) {
        Ok(mac_addr) => {
            MagicPacket::new(mac_addr).send().unwrap();
        }
        Err(_) => panic!("Invalid MAC Address"),
    }
}
