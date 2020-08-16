use crate::protocol::NodeId;
use async_std::net::SocketAddr;

#[derive(Debug, Clone, Ord, PartialOrd, Eq, PartialEq)]
pub struct Reachability {
    address: Option<SocketAddr>,
    relay: Option<NodeId>,
}
