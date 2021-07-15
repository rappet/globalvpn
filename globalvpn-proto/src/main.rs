extern crate log;
extern crate env_logger;
extern crate thiserror;
extern crate anyhow;
extern crate derive_more;
extern crate chrono;
extern crate serde;
extern crate bytes;
extern crate smallvec;
extern crate sodiumoxide;
extern crate tokio;
#[macro_use]
extern crate bitflags;
extern crate rmp_serde;
extern crate toml;

use log::{info, LevelFilter};
use crate::protocol::reachability::ReachabilityInformation;

mod data;
mod protocol;
mod prelude;

fn main() {
    env_logger::builder().filter_level(LevelFilter::Info).init();

    let reachability = ReachabilityInformation {
        ipv4: Some("217.230.92.35:1337".parse().unwrap()),
        ipv6: Some("[2003:f9:8f02:100:52a1:d555:d19f:9549]:1337".parse().unwrap()),
        proxy: None
    };

    info!("Reachability: {:?}", reachability.encode());

    info!("Hello, world!");
}
