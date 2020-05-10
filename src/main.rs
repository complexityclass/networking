extern crate pnet;

use pnet::datalink::{self};
use pnet::ipnetwork::IpNetwork;

#[derive(Debug)]
struct IfAddr {
    name: String,
    addrs: Vec<String>,
}

impl std::fmt::Display for IfAddr {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "({:5}{:?})", self.name, self.addrs)
    }
}

fn main() {
    let interfaces = datalink::interfaces();
    let ifaces: Vec<IfAddr> = interfaces
        .into_iter()
        .map(|iface| {
            let ips: Vec<(String, String)> = iface
                .ips
                .into_iter()
                .map(|ipnet| {
                    let family: (String, String) = match ipnet {
                        IpNetwork::V4(v4) => (String::from("IPv4"), v4.ip().to_string()),
                        IpNetwork::V6(v6) => (String::from("IPv6"), v6.ip().to_string()),
                    };
                    family
                })
                .rev()
                .collect();
            IfAddr {
                name: iface.name,
                addrs: ips
                    .into_iter()
                    .map(|s| format!("{:5}{}", s.0, s.1))
                    .rev()
                    .collect(),
            }
        })
        .rev()
        .collect();
    println!("{:?}", ifaces);
}
