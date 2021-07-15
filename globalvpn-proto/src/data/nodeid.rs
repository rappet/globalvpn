use crate::prelude::*;

/// Hash value of the public signing key of a node
///
/// The NodeId is used to identify a node in the Node directionary
#[derive(Debug, Clone, Ord, PartialOrd, Eq, PartialEq)]
#[derive(Serialize, Deserialize)]
#[serde(transparent)]
pub struct NodeId {
    hash: [u8; sodiumoxide::crypto::hash::sha256::DIGESTBYTES],
}
