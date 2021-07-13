use serde::{Deserialize, Serialize};
use sodiumoxide::crypto::sign;
use std::borrow::Borrow;

/// A complete keyset used to sign data originating from a node.
/// This contains **private** information
pub struct Signkey {
    seed: sign::Seed,
    sk: sign::SecretKey,
    pk: sign::PublicKey,
}

impl From<sign::Seed> for Signkey {
    fn from(seed: sign::Seed) -> Self {
        let (pk, sk) = sign::keypair_from_seed(seed.borrow());
        Signkey { seed, sk, pk }
    }
}

#[derive(Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(tag = "version")]
pub enum VersionedSignkey {
    V1(SignkeyV1),
}

#[derive(Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct SignkeyV1 {
    seed: String,
}

#[cfg(test)]
mod test {
    use crate::data::signkey::{SignkeyV1, VersionedSignkey};

    #[test]
    fn deserialize() {
        let s = r#"# THIS IS A PRIVATE KEY! KEEP SECRET!
version = "V1"
seed = "FOO"
"#;

        let file: VersionedSignkey = toml::from_str(s).unwrap();
        assert_eq!(file, VersionedSignkey::V1(SignkeyV1 { seed: "FOO".into() }));
    }
}
