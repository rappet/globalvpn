//! Signed data of a client that can get distributed in directories

use bytes::Bytes;
use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use std::net::{SocketAddrV4, SocketAddrV6, Ipv4Addr, Ipv6Addr};
use crate::protocol::NodeId;

pub struct SignableDataset<T> {
    content: T,
    created: NaiveDateTime,
}

pub struct SignedDataset<T> {
    content: Bytes,
    signature: Bytes,
    _phantom: std::marker::PhantomData<T>,
}
