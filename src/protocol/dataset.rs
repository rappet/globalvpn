use serde::{Serialize, Deserialize};
use chrono::NaiveDateTime;
use bytes::Bytes;

pub struct SignableDataset<T>
    where T: Serialize + Deserialize {
    content: T,
    created: NaiveDateTime
}

pub struct SignedDataset<T>
    where T: Serialize + Deserialize {
    content: Bytes,
    signature: Bytes,
}
