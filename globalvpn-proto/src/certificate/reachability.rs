use std::collections::BTreeSet;
use std::net::IpAddr;
use yasna::{
    ASN1Error, ASN1ErrorKind, ASN1Result, BERDecodable, BERReader, DEREncodable, DERWriter, Tag,
};

/// Complete reachability information for a node
#[derive(Debug, Clone, Ord, PartialOrd, Eq, PartialEq, Default)]
pub struct NodeReachabilityInformation {
    /// reachability of this node over the internet
    pub network_reachability: BTreeSet<NodeIpReachability>,
    /// reachability of this node using a proxy node
    pub proxy_reachability: BTreeSet<NodeProxyReachability>,
}

impl DEREncodable for NodeReachabilityInformation {
    fn encode_der(&self, writer: DERWriter) {
        writer.write_sequence(|writer| {
            writer.next().write_set_of(|writer| {
                for reachability in &self.network_reachability {
                    reachability.encode_der(writer.next());
                }
            });
            writer.next().write_set_of(|writer| {
                for proxy in &self.proxy_reachability {
                    proxy.encode_der(writer.next());
                }
            });
        });
    }
}

impl BERDecodable for NodeReachabilityInformation {
    fn decode_ber(reader: BERReader) -> ASN1Result<Self> {
        reader.read_sequence(|reader| {
            let network_reachability = reader
                .next()
                .collect_set_of(NodeIpReachability::decode_ber)?
                .into_iter()
                .collect();
            let proxy_reachability = reader
                .next()
                .collect_set_of(NodeProxyReachability::decode_ber)?
                .into_iter()
                .collect();
            Ok(NodeReachabilityInformation {
                network_reachability,
                proxy_reachability,
            })
        })
    }
}

/// Information about how to reach a single node over the internet
#[derive(Debug, Clone, Ord, PartialOrd, Eq, PartialEq)]
pub struct NodeIpReachability {
    /// public IP Address
    pub address: IpAddr,
    /// Port of the QUIC socket
    pub quic_port: Option<u16>,
}

impl DEREncodable for NodeIpReachability {
    fn encode_der(&self, writer: DERWriter) {
        writer.write_sequence(|writer| {
            writer
                .next()
                .write_utf8_string(self.address.to_string().as_str());
            if let Some(quic_port) = self.quic_port {
                writer.next().write_tagged(Tag::context(0), |writer| {
                    writer.write_u16(quic_port);
                });
            }
        });
    }
}

impl BERDecodable for NodeIpReachability {
    fn decode_ber(reader: BERReader) -> ASN1Result<Self> {
        reader.read_sequence(|reader| {
            let address_str = reader.next().read_utf8string()?;
            let address = address_str
                .parse()
                .map_err(|_err| ASN1Error::new(ASN1ErrorKind::Invalid))?;

            let quic_port = reader.read_optional(|reader| {
                reader.read_tagged(Tag::context(0), |reader| reader.read_u16())
            })?;

            Ok(NodeIpReachability { address, quic_port })
        })
    }
}

/// Information about how to reach a single node using a proxy node
#[derive(Debug, Clone, Ord, PartialOrd, Eq, PartialEq)]
pub struct NodeProxyReachability {
    /// Hash of the public key of the node
    pub proxy_address: Vec<u8>,
    /// optional IP-Address information to ommit lookup
    pub proxy_reachability: BTreeSet<NodeIpReachability>,
}

impl DEREncodable for NodeProxyReachability {
    fn encode_der(&self, writer: DERWriter) {
        writer.write_sequence(|writer| {
            writer.next().write_bytes(self.proxy_address.as_slice());
            if !self.proxy_reachability.is_empty() {
                writer.next().write_tagged(Tag::context(0), |writer| {
                    writer.write_set_of(|writer| {
                        for reachability in &self.proxy_reachability {
                            reachability.encode_der(writer.next());
                        }
                    });
                });
            }
        });
    }
}

impl BERDecodable for NodeProxyReachability {
    fn decode_ber(reader: BERReader) -> ASN1Result<Self> {
        reader.read_sequence(|reader| {
            let proxy_address = reader.next().read_bytes()?;
            let proxy_reachability = reader
                .read_optional(|reader| {
                    reader.read_tagged(Tag::context(0), |reader| {
                        Ok(reader
                            .collect_set_of(|reader| NodeIpReachability::decode_ber(reader))?
                            .into_iter()
                            .collect())
                    })
                })?
                .unwrap_or_else(|| BTreeSet::new());
            Ok(NodeProxyReachability {
                proxy_address,
                proxy_reachability,
            })
        })
    }
}

#[cfg(test)]
mod tests {
    use crate::certificate::{
        NodeIpReachability, NodeProxyReachability, NodeReachabilityInformation,
    };
    use std::collections::BTreeSet;

    #[test]
    fn test_encode_decode_node_reachability_information() {
        let testvec = vec![NodeReachabilityInformation {
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
        }];
    }

    #[test]
    fn test_encode_decode_node_ip_reachability() {
        let testvec = vec![
            NodeIpReachability {
                address: "127.0.0.1".parse().unwrap(),
                quic_port: Some(1337),
            },
            NodeIpReachability {
                address: "2a0e:46c6::2".parse().unwrap(),
                quic_port: Some(1337),
            },
            NodeIpReachability {
                address: "127.0.0.1".parse().unwrap(),
                quic_port: None,
            },
        ];

        for case in testvec {
            let encoded = yasna::encode_der(&case);
            let decoded: NodeIpReachability = yasna::decode_der(encoded.as_slice()).unwrap();
            assert_eq!(case, decoded);
        }
    }

    #[test]
    fn test_encode_decode_node_proxy_reachability() {
        let testvec = vec![
            NodeProxyReachability {
                proxy_address: vec![123, 34, 54, 96, 34],
                proxy_reachability: BTreeSet::new(),
            },
            NodeProxyReachability {
                proxy_address: vec![123, 34, 54, 96, 34],
                proxy_reachability: vec![NodeIpReachability {
                    address: "2a0e:46c6::2".parse().unwrap(),
                    quic_port: Some(1337),
                }]
                .into_iter()
                .collect(),
            },
        ];

        for case in testvec {
            let encoded = yasna::encode_der(&case);
            let decoded: NodeProxyReachability = yasna::decode_der(encoded.as_slice()).unwrap();
            assert_eq!(case, decoded);
        }
    }
}
