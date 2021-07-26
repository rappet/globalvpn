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

use std::net::{SocketAddrV4, SocketAddrV6};

pub const OID_GLOBALVPN_X509_REACHABILITY: &[u64] = &[1, 3, 6, 1, 4, 1, 57716, 2, 1, 1];
pub const OID_GLOBALVPN_X509_METADATA: &[u64] = &[1, 3, 6, 1, 4, 1, 57716, 2, 1, 2];

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

/*impl ReachabilityInformation {
    /// Encode reachability information as X.509 and sign them
    pub fn sign_as_der(&self, private_key_der: &[u8]) -> CertificateResult<Vec<u8>> {
        let certificate = self.build_certificate(private_key_der)?;
        Ok(certificate.serialize_der().map_err(|_err| CertificateError::GeneratingCertificate)?)
    }

    fn build_certificate(&self, private_key_der: &[u8]) -> CertificateResult<Certificate> {
        let mut params = CertificateParams::default();
        params.alg = &rcgen::PKCS_ED25519;
        params.not_before = Utc::today().and_hms(0, 0, 0);
        params.not_after = (Utc::today() + Duration::days(7)).and_hms(0, 0, 0);
        params.key_pair = Some(KeyPair::from_der(private_key_der).map_err(|_err| CertificateError::ParseDer)?);
        params.distinguished_name = {
            let mut name = DistinguishedName::new();
            //name.push(DnType::CommonName, "globalvpn self signed node cert");
            name
        };

        if let Some(ipv4_endpoint) = self.ipv4_endpoint {
            params.custom_extensions.push(
                CustomExtension::from_oid_content(
                    OID_IPV4_ENDPOINT,
                    ipv4_endpoint.to_string().into_bytes()
                )
            );
        }

        if let Some(ipv6_endpoint) = self.ipv6_endpoint {
            params.custom_extensions.push(
                CustomExtension::from_oid_content(
                    OID_IPV6_ENDPOINT,
                    ipv6_endpoint.to_string().into_bytes()
                )
            );
        }

        if let Some(proxy_node) = &self.proxy_node {
            params.custom_extensions.push(
                CustomExtension::from_oid_content(
                    OID_PROXY_NODE,
                    base64::encode(proxy_node.as_slice()).into_bytes()
                )
            );
        }

        Ok(Certificate::from_params(params).map_err(|_err| CertificateError::GeneratingCertificate)?)
    }
}

impl TryFrom<&HashMap<Oid<'_>, X509Extension<'_>>> for ReachabilityInformation {
    type Error = CertificateError;

    fn try_from(extensions: &HashMap<Oid<'_>, X509Extension<'_>>) -> Result<Self, Self::Error> {
        let mut reachability_information: ReachabilityInformation = Default::default();

        if let Some(extension) = extensions.get(
            &Oid::from(OID_IPV4_ENDPOINT).expect("Invalid OID for IPv4 reachability")
        ) {
            let ipv4_str = std::str::from_utf8(extension.value)
                .map_err(|_err| CertificateError::InvalidIpv4ReachabilityInformation)?;
            reachability_information.ipv4_endpoint = Some(
                ipv4_str.parse()
                .map_err(|_err| CertificateError::InvalidIpv4ReachabilityInformation)?
            );
        }

        if let Some(extension) = extensions.get(
            &Oid::from(OID_IPV6_ENDPOINT).expect("Invalid OID for IPv6 reachability")
        ) {
            let ipv4_str = std::str::from_utf8(extension.value)
                .map_err(|_err| CertificateError::InvalidIpv6ReachabilityInformation)?;
            reachability_information.ipv6_endpoint = Some(
                ipv4_str.parse()
                    .map_err(|_err| CertificateError::InvalidIpv6ReachabilityInformation)?
            );
        }

        Ok(reachability_information)
    }
}*/

/// Reachability information and other metadata
#[derive(Debug, Clone, Eq, PartialEq)]
#[non_exhaustive]
pub struct GlobalvpnCertificate {
    /// Ed25519 public key
    public_key: Vec<u8>,

    reachability_information: ReachabilityInformation,
}

#[derive(thiserror::Error, Debug, Copy, Clone, Eq, PartialEq)]
#[non_exhaustive]
pub enum CertificateError {
    /// IPv4 reachability information is not a valid IPv4 socket address
    #[error("malformat IPv4 socket address")]
    InvalidIpv4ReachabilityInformation,
    /// IPv6 reachability information is not a valid IPv6 socket address
    #[error("malformat IPv6 socket address")]
    InvalidIpv6ReachabilityInformation,
    /// proxy node address is not valid
    #[error("invalid proxy node address")]
    InvalidProxyNode,
    /// error during certificate generation
    #[error("could not generate certificate")]
    GeneratingCertificate,
    /// malformat DER private key
    #[error("private key is not a valid DER encoded key")]
    ParseDer,
}

pub type CertificateResult<T> = Result<T, CertificateError>;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
