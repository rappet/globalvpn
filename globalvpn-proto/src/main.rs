#[macro_use]
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

use log::info;

mod data;
mod protocol;

fn main() {
    env_logger::init();

    info!("Hello, world!");
}
