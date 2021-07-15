//! Signed data of a client that can get distributed in directories

use bytes::Bytes;
use chrono::NaiveDateTime;

pub struct SignableDataset<T> {
    content: T,
    created: NaiveDateTime,
}

pub struct SignedDataset<T> {
    content: Bytes,
    signature: Bytes,
    _phantom: std::marker::PhantomData<T>,
}
