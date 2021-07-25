#[derive(Debug, Clone, Eq, PartialEq, Default)]
pub struct NodeMetadata {
    /// Time, how long the certificate should be hold in an operating node
    maximumWarnTableSeconds: Option<u64>,
    /// Time, how long the certificate can be used in a freshly
    /// bootet node that does not contain a warm table yet
    maximumColdTableSeconds: Option<u64>,
}
