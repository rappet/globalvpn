use bytes::Bytes;

/// Frame
/// ------
///
/// | Type  | Name        | Comment                  |
/// | ----- | ----------- | ------------------------ |
/// | u16   | Length      | describes payload length |
/// | u8    | Packet Type |                          |
/// | bytes | Payload     |                          |
///
/// The type must be defined in a specification.
/// Implementation specific packet types can be added via custom packet type.
/// A packet payload should not be larger than 65536 bytes.
pub struct Frame {
    packet_type: u8,
    payload: Bytes,
}
