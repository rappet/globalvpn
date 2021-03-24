use crate::protocol::NodeId;
use std::net::SocketAddr;

#[derive(Debug, Clone, Ord, PartialOrd, Eq, PartialEq)]
pub struct Reachability {
    address: Option<SocketAddr>,
    relay: Option<NodeId>,
}
