use crate::protocol::NodeId;
use std::net::{SocketAddr, SocketAddrV4, SocketAddrV6};

/// All reachability information about a specific node stored
/// in the public node directory.
///
/// Additional information can get accessed using secondary directories or asking the node itself
#[derive(Debug, Clone, Ord, PartialOrd, Eq, PartialEq)]
pub struct ReachabilityInformation {
    flags: NodeFlags,
    /// Public IPv4 address to connect to the metadata channel of the node
    ipv4: Option<SocketAddrV4>,
    /// Public IPv6 address to connect to the metadata channel of the node
    ipv6: Option<SocketAddrV6>,
    /// [NodeId](crate::protocol::NodeId) of the proxy node used by the node
    proxy: Option<NodeId>,
}

bitflags! {
    struct NodeFlags: u32 {
        /// Node is part of the distributed dictionary
        const Dictionary = 0x01;
        /// Node will proxy metadata
        const Metadata = 0x02;
        /// Node will proxy datapackets
        const Proxy = 0x04;
    }
}
