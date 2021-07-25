use std::net::IpAddr;

#[derive(Debug, Clone, Eq, PartialEq, Default)]
pub struct NodeReachabilityInformation {
    pub network_reachability: Vec<NodeIpReachability>,
    pub proxy_reachability: Vec<NodeProxyReachability>,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct NodeIpReachability {
    pub address: IpAddr,
    pub quicPort: Option<u16>,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct NodeProxyReachability {
    /// Hash of the public key of the node
    proxyAddress: Vec<u8>,
    /// optional IP-Address information to ommit lookup
    proxyReachability: Vec<NodeIpReachability>,
}
