extern crate anyhow;
extern crate bytes;
extern crate chrono;
extern crate derive_more;
extern crate env_logger;
extern crate log;
extern crate serde;
extern crate smallvec;
extern crate sodiumoxide;
extern crate thiserror;
extern crate tokio;
#[macro_use]
extern crate bitflags;
extern crate rmp_serde;
extern crate toml;
extern crate yasna;

use crate::certificate::{NodeIpReachability, NodeProxyReachability, NodeReachabilityInformation};
use log::{info, LevelFilter};
use std::collections::BTreeSet;
use std::fs::File;
use std::io::Write;

pub mod certificate;
mod data;
mod prelude;
mod protocol;

fn main() -> anyhow::Result<()> {
    env_logger::builder().filter_level(LevelFilter::Info).init();

    let reachability = NodeReachabilityInformation {
        network_reachability: vec![
            NodeIpReachability {
                address: "127.0.0.1".parse().unwrap(),
                quic_port: Some(1337),
            },
            NodeIpReachability {
                address: "2a0e:46c6::2".parse().unwrap(),
                quic_port: Some(1337),
            },
        ]
        .into_iter()
        .collect(),
        proxy_reachability: vec![NodeProxyReachability {
            proxy_address: vec![123, 34, 34, 212, 43, 93],
            proxy_reachability: BTreeSet::new(),
        }]
        .into_iter()
        .collect(),
    };

    let mut out = File::create("out.der")?;
    let der = yasna::encode_der(&reachability);
    out.write_all(der.as_slice())?;
    drop(out);

    let decoded: NodeReachabilityInformation = yasna::decode_ber(&der)?;
    assert_eq!(reachability, decoded);

    info!("Hello, world!");
    Ok(())
}
