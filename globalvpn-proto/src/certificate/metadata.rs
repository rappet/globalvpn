use yasna::{ASN1Result, BERDecodable, BERReader, DEREncodable, DERWriter, Tag};

#[derive(Debug, Clone, Eq, PartialEq, Default)]
pub struct NodeMetadata {
    /// Time, how long the certificate should be hold in an operating node
    pub maximum_warm_table_seconds: Option<u64>,
    /// Time, how long the certificate can be used in a freshly
    /// bootet node that does not contain a warm table yet
    pub maximum_cold_table_seconds: Option<u64>,
}

impl DEREncodable for NodeMetadata {
    fn encode_der(&self, writer: DERWriter) {
        writer.write_sequence(|writer| {
            if let Some(maximum_warn_table_seconds) = self.maximum_warm_table_seconds {
                writer.next().write_tagged(Tag::context(0), |writer| {
                    writer.write_u64(maximum_warn_table_seconds);
                });
            }
            if let Some(maximum_cold_table_seconds) = self.maximum_cold_table_seconds {
                writer.next().write_tagged(Tag::context(1), |writer| {
                    writer.write_u64(maximum_cold_table_seconds);
                });
            }
        });
    }
}

impl BERDecodable for NodeMetadata {
    fn decode_ber(reader: BERReader) -> ASN1Result<Self> {
        reader.read_sequence(|reader| {
            let maximum_warm_table_seconds = reader.read_optional(|reader| {
                reader.read_tagged(Tag::context(0), |reader| reader.read_u64())
            })?;
            let maximum_cold_table_seconds = reader.read_optional(|reader| {
                reader.read_tagged(Tag::context(1), |reader| reader.read_u64())
            })?;
            Ok(NodeMetadata {
                maximum_warm_table_seconds,
                maximum_cold_table_seconds,
            })
        })
    }
}

#[cfg(test)]
mod tests {
    use crate::certificate::NodeMetadata;

    #[test]
    fn test_encode_decode_node_metadata() {
        let testvec = vec![
            NodeMetadata {
                maximum_warm_table_seconds: Some(12345),
                maximum_cold_table_seconds: None,
            },
            NodeMetadata {
                maximum_warm_table_seconds: None,
                maximum_cold_table_seconds: Some(12345),
            },
            NodeMetadata {
                maximum_warm_table_seconds: None,
                maximum_cold_table_seconds: None,
            },
            NodeMetadata {
                maximum_warm_table_seconds: Some(12345),
                maximum_cold_table_seconds: Some(54321),
            },
        ];

        for case in testvec {
            let encoded = yasna::encode_der(&case);
            let decoded: NodeMetadata = yasna::decode_der(encoded.as_slice()).unwrap();
            assert_eq!(case, decoded);
        }
    }
}
