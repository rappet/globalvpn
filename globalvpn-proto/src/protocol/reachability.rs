use crate::protocol::NodeId;
use std::net::{SocketAddrV4, SocketAddrV6};

use crate::prelude::*;

/// All reachability information about a specific node stored
/// in the public node directory.
///
/// Additional information can get accessed using secondary directories or asking the node itself
#[derive(Serialize, Deserialize)]
#[derive(Debug, Clone, Ord, PartialOrd, Eq, PartialEq)]
pub struct ReachabilityInformation {
    /// Public IPv4 address to connect to the metadata channel of the node
    pub ipv4: Option<SocketAddrV4>,
    /// Public IPv6 address to connect to the metadata channel of the node
    pub ipv6: Option<SocketAddrV6>,
    /// [NodeId](crate::protocol::NodeId) of the proxy node used by the node
    pub proxy: Option<NodeId>,
    // /// Flags of a node
    //pub flags: NodeFlags,
}

impl ReachabilityInformation {
    pub fn encode(&self) -> Vec<u8> {
        rmp_serde::encode::to_vec(self).expect("cannot encode reachability information")
    }
}

bitflags! {
    struct NodeFlags: u32 {
        /// Node is part of the distributed dictionary
        const DICTIONARY = 0x01;
        /// Node will proxy metadata
        const METADATA = 0x02;
        /// Node will proxy datapackets
        const PROXY = 0x04;
    }
}
