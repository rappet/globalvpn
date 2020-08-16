use bytes::Bytes;
use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

pub struct SignableDataset<T> {
    content: T,
    created: NaiveDateTime,
}

pub struct SignedDataset<T> {
    content: Bytes,
    signature: Bytes,
    _phantom: std::marker::PhantomData<T>,
}
