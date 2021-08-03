//! Globalvpn certificate creation/verification
//!
//! Self signed X.509 certificates are used as the data format the node directory
//! to distribute reachability information and the public key of each node.
//!
//! The certificate has to contain a Ed25519 public key. This is subject to be changed later.
//! Certificates are exchanged in DER format.
//! A conversion to PEM format is used in configuration files.
//!
//! # Custom Extensions
//!
//! The reachability information (Socket-Address, Proxy Node ID, ...) are stored
//! in a custom extension.
//!
//! ## OID Identifier
//!
//! `1.3.6.1.4.1.57716.2.1` is used as the parent OID Identifier for X.509 extensions
//!
//! ## List of extensions
//!
//! | OID                       | Content           |
//! | ------------------------- | ----------------- |
//! | `1.3.6.1.4.1.57716.2.1.1` | NodeReachability |
//! | `1.3.6.1.4.1.57716.2.1.2` | NodeMetadata     |
//!
//! The content of each extension is a DER encoded ASN.1 Sequence as specified below
//!
//! ## NodeReachability
//!
//! ```asn.1
//! NodeReachability DEFINITIONS ::= BEGIN
//!
//!     NodeReachabilityInformation ::= SEQUENCE {
//!         networkReachability SET OF SingleNodeReachability,
//!         proxyReachability  SET OF OCTET STRING
//!     }
//!
//!     NodeIpReachability ::= SEQUENCE {
//!         address         UTF8String,
//!         quicPort    [0] EXPLICIT INTEGER OPTIONAL
//!     }
//!
//!     NodeProxyReachability ::= SEQUENCE {
//!         proxyAddress            OCTET STRING
//!         proxyReachability   [0] EXPLICIT SET OF NodeIpReachability
//!     }
//! END
//! ```
//!
//! ## NodeMetadata
//!
//! ```asn.1
//! NodeMetadata DEFINITIONS ::= BEGIN
//!
//!     NodeMetadata ::= SEQUENCE {
//!         maximumWarmTableSeconds [0] EXPLICIT INTEGER
//!         maximumColdTableSeconds [1] EXPLICIT INTEGER
//!     }
//!
//! END
//! ```

mod metadata;
mod reachability;

pub use metadata::NodeMetadata;
pub use reachability::{NodeIpReachability, NodeProxyReachability, NodeReachabilityInformation};

use chrono::{Duration, Utc};
use log::{error, warn};
use pem::Pem;
use rcgen::{Certificate, CertificateParams, CustomExtension, DistinguishedName};
use std::convert::TryFrom;
use std::net::{SocketAddrV4, SocketAddrV6};
use x509_parser::der_parser::oid::Oid;
use x509_parser::error::X509Error;

pub const OID_GLOBALVPN_X509_REACHABILITY: &[u64] = &[1, 3, 6, 1, 4, 1, 57716, 2, 1, 1];
pub const OID_GLOBALVPN_X509_METADATA: &[u64] = &[1, 3, 6, 1, 4, 1, 57716, 2, 1, 2];

const OID_X509_ED25519: &[u64] = &[1, 3, 101, 112];

/// Node reachability information
#[derive(Debug, Clone, Eq, PartialEq, Default)]
#[non_exhaustive]
pub struct ReachabilityInformation {
    /// Remote IPv4 address and port of node
    ///
    /// OID `1.3.6.1.4.1.123456.1.1`
    pub ipv4_endpoint: Option<SocketAddrV4>,

    /// Remote IPv6 address and port of node
    ///
    /// OID `1.3.6.1.4.1.123456.1.2`
    pub ipv6_endpoint: Option<SocketAddrV6>,

    /// Proxy node ID
    ///
    /// OID `1.3.6.1.4.1.123456.1.3`
    pub proxy_node: Option<Vec<u8>>,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct CertificateData {
    pub reachability: NodeReachabilityInformation,
    pub metadata: NodeMetadata,
}

impl CertificateData {
    pub fn sign(&self, private_key_der: &[u8]) -> CertificateResult<RawCertificate> {
        let mut params = CertificateParams::default();
        params.alg = &rcgen::PKCS_ED25519;
        params.not_before = Utc::today().and_hms(0, 0, 0);
        params.not_after = (Utc::today() + Duration::days(7)).and_hms(0, 0, 0);
        //params.key_pair =
        //    Some(KeyPair::from_der(private_key_der).map_err(|_err| CertificateError::ParseDer)?);
        params.distinguished_name = {
            let mut name = DistinguishedName::new();
            //name.push(DnType::CommonName, "globalvpn self signed node cert");
            name
        };

        params
            .custom_extensions
            .push(CustomExtension::from_oid_content(
                OID_GLOBALVPN_X509_REACHABILITY,
                yasna::encode_der(&self.reachability),
            ));
        params
            .custom_extensions
            .push(CustomExtension::from_oid_content(
                OID_GLOBALVPN_X509_METADATA,
                yasna::encode_der(&self.metadata),
            ));

        let certificate = Certificate::from_params(params)
            .map_err(|_err| CertificateError::GeneratingCertificate)?;
        let encoded_der = certificate
            .serialize_der()
            .map_err(|_err| CertificateError::GeneratingCertificate)?;
        Ok(RawCertificate { encoded_der })
    }
}

impl TryFrom<RawCertificate> for CertificateData {
    type Error = CertificateError;

    fn try_from(value: RawCertificate) -> Result<Self, Self::Error> {
        let (_, certificate) = x509_parser::parse_x509_certificate(value.der())?;

        let certificate_oid = certificate.signature_algorithm.algorithm;
        let x509_ed25519_oid =
            x509_parser::der_parser::oid::Oid::from(OID_X509_ED25519).expect("invalid OID");

        if certificate_oid != x509_ed25519_oid {
            return Err(CertificateError::InvalidSignatureAlgorithm);
        }
        // TODO check certificate
        warn!("X.509 ceritificate verification is not implemented, until x509_parser supports Ed25519");

        let extensions = certificate.tbs_certificate.extensions();
        let reachability = yasna::decode_der(
            extensions
                .get(&Oid::from(OID_GLOBALVPN_X509_REACHABILITY).unwrap())
                .ok_or(CertificateError::MissingReachabilityInformation)?
                .value,
        )
            .map_err(|_err| CertificateError::DecodeReachabilityInformation)?;
        let metadata = yasna::decode_der(
            extensions
                .get(&Oid::from(OID_GLOBALVPN_X509_METADATA).unwrap())
                .ok_or(CertificateError::MissingNodeMetadata)?
                .value,
        )
            .map_err(|_err| CertificateError::DecodeNodeMetadata)?;

        Ok(CertificateData {
            reachability,
            metadata,
        })
    }
}

#[derive(Debug, Clone, Ord, PartialOrd, Eq, PartialEq)]
pub struct RawCertificate {
    pub(crate) encoded_der: Vec<u8>,
}

impl RawCertificate {
    pub fn der(&self) -> &[u8] {
        self.encoded_der.as_slice()
    }

    pub fn pem(&self) -> String {
        let pem = Pem {
            tag: "CERTIFICATE".to_string(),
            contents: self.encoded_der.clone(),
        };
        pem::encode(&pem)
    }
}

#[derive(thiserror::Error, Debug)]
#[non_exhaustive]
pub enum CertificateError {
    /// error during certificate generation
    #[error("could not generate certificate")]
    GeneratingCertificate,
    /// malformat DER private key
    #[error("private key is not a valid DER encoded key")]
    ParseDer,
    /// invalid signature algorithm
    #[error("certificate has invalid signature algorithm")]
    InvalidSignatureAlgorithm,
    /// X.509 parsing error
    #[error("decoding X.509: {0}")]
    X509(#[from] x509_parser::nom::Err<X509Error>),
    /// decoding node metadata extension
    #[error("decoding node metadata")]
    DecodeNodeMetadata,
    /// missing node metadata extension
    #[error("missing node metadata")]
    MissingNodeMetadata,
    /// decoding node metadata extension
    #[error("decoding reachability information")]
    DecodeReachabilityInformation,
    /// missing node metadata extension
    #[error("missing reachability information")]
    MissingReachabilityInformation,
}

pub type CertificateResult<T> = Result<T, CertificateError>;

#[cfg(test)]
mod tests {
    use crate::certificate::{
        CertificateData, NodeIpReachability, NodeMetadata, NodeProxyReachability,
        NodeReachabilityInformation,
    };
    use ring::rand::SystemRandom;
    use std::collections::BTreeSet;
    use std::convert::{TryFrom, TryInto};

    #[test]
    fn roundtrip() {
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
            metadata,
        };
        let mut rng = SystemRandom::new();
        let private_key = ring::signature::Ed25519KeyPair::generate_pkcs8(&mut rng).unwrap();

        let encoded = certificate_data.sign(private_key.as_ref()).unwrap();
        let decoded: CertificateData = encoded.try_into().unwrap();
        assert_eq!(certificate_data, decoded);
    }
}
