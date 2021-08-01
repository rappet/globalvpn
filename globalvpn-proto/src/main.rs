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

use crate::certificate::{NodeIpReachability, NodeProxyReachability, NodeReachabilityInformation, NodeMetadata, CertificateData};
use log::{info, LevelFilter};
use std::collections::BTreeSet;
use std::fs::File;
use std::io::Write;
use ring::rand::SystemRandom;

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

    let metadata = NodeMetadata {
        maximum_warm_table_seconds: Some(2600),
        maximum_cold_table_seconds: None,
    };

    let certificate_data = CertificateData {
        reachability,
        metadata
    };
    let mut rng = SystemRandom::new();
    let private_key = ring::signature::Ed25519KeyPair::generate_pkcs8(&mut rng).unwrap();
    let encoded = certificate_data.sign(private_key.as_ref())?;

    let mut out = File::create("out.der")?;
    out.write_all(encoded.der())?;
    drop(out);

    println!("{}", encoded.pem());

    let (rem, cer) = x509_parser::parse_x509_certificate(encoded.der())?;
    let issuer_pk = &cer.tbs_certificate.subject_pki;
    cer.verify_signature(Some(issuer_pk))?;

    info!("Hello, world!");
    Ok(())
}
