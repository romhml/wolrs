#![feature(proc_macro_hygiene, decl_macro)]
#[cfg(feature = "server")]
#[macro_use]
extern crate rocket;
use mac_address::MacAddress;
use std::str::FromStr;

use rocket::http::Status;

use wolrs::MagicPacket;

#[post("/wol?<addr>")]
fn wol(addr: String) -> Status {
    match MacAddress::from_str(&addr) {
        Ok(mac_addr) => {
            MagicPacket::new(mac_addr).send().unwrap();
            Status::Ok
        }
        Err(_) => Status::BadRequest,
    }
}

fn main() {
    rocket::ignite().mount("/", routes![wol]).launch();
}
