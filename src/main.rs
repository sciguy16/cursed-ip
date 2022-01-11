// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use std::net::{IpAddr, Ipv6Addr};
use uuid::Uuid;

fn main() {
    let arg = std::env::args().nth(1).unwrap_or_else(|| {
        eprintln!("Usage: cargo run -- IP|UUID");
        std::process::exit(1);
    });

    process(&arg);
}

fn process(arg: &str) {
    if let Ok(ip) = arg.parse::<IpAddr>() {
        // IP: render as UUID
        let ip = match ip {
            IpAddr::V6(addr) => addr,
            IpAddr::V4(addr) => addr.to_ipv6_mapped(),
        };
        let octets = ip.octets();
        let u = Uuid::from_slice(&octets).unwrap();
        println!("{}", u);
    } else if let Ok(u) = arg.parse::<Uuid>() {
        // UUID: render as IP
        let num = u.as_u128();
        let addr = Ipv6Addr::from(num);
        println!("{}", addr);
    } else {
        eprintln!("Unable to parse input as IP address or UUID");
        std::process::exit(2);
    }
}
