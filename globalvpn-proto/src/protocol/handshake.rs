//! Cryptographic stream handshake
//!
//! Currently, there is one implementation available for a handshake, which uses libsodium
//!
//! # Libsodium handshake
//!
//! The handshake is an 1-RTT handshake. The public keys of both partners
//! are exchanged and used to sign the data for a key exchange.
//!
//! ```
//! Initiator                                                    Acceptor
//! ---------------------------------------------------------------------
//! client public key
//! signed key exchange public Key  ----->
//!                                                     server public key
//!                                        signed key exchange public key
//! session data                    <---->                   session data
//! ```
//!
//! Session Handshake
//! ----------------
//!
//! Used to establish a cryptographic connection.
//!
//! Both sides:
//! Send handshake packet.
//!
//! | Type     | Content                                                |
//! | -------- | ------------------------------------------------------ |
//! | u16      | Magic Number 0xf00f                                    |
//! | u8       | len `n` of supported cryptography protocols            |
//! | `n` x u8 | id      of supported cryptography protocol             |
//! | u8       | len `m` of additional data fields                      |
//! | -        | `m` [addidtional data fields](#additional-data-fields) |
//!
//! ### Additional data fields
//!
//! Each data field is prepended in a header which describes its type and length.
//!
//! | Type  | Content                     |
//! | ----- | --------------------------- |
//! | u16   | id type of additional field |
//! | u16   | len of field                |
//! | `len` | content of field            |
//!
//! #### Sodium extra data (ID 1)
//!
//! Public key, crypto_sign_PUBLICKEYBYTES bytes long
//!
//! crypto kx sessionkey. signed using
//!
//! ```c
//! crypto_sign(signed_key, &signed_key_len,
//! signing_key, &sizeof(signing_key), sk);
//! ```
//!
//! ### Sodium key exchange (protocol 1)
//!
//! - Send own public signing key information (both sides) + signed
//! public encryption key with extension 1 and 2
//! - Send encrypted symmetric key to other side.

use sodiumoxide::crypto::{kx, sign};
use thiserror::Error;
use std::convert::TryFrom;

pub struct SessionHandshake {
    supported_protocols: Vec<CryptoProtocol>,
    sodium_extra: Option<SessionHandshakeSodiumExtraData>,
}

pub enum CryptoProtocol {
    Sodium,
    Unknown(u8),
}

impl CryptoProtocol {
    pub fn is_unknown(&self) -> bool {
        matches!(self, CryptoProtocol::Unknown(_))
    }
}


/// First step in libsodium handshake *client* -> *data*
///
/// The public key of the client and the public key of the key exchange are transmittet
/// The public key of the key exchange is signed using the private key of the client
pub struct SessionHandshakeSodiumExtraData {
    signing_pk: sign::PublicKey,
    // libsodium kx publickey
    session_pk: kx::PublicKey,
}

impl SessionHandshakeSodiumExtraData {
    fn new(signing_pk: &sign::PublicKey, session_pk: kx::PublicKey) -> SessionHandshakeSodiumExtraData{
        SessionHandshakeSodiumExtraData {
            signing_pk: *signing_pk,
            session_pk: session_pk,
        }
    }

    fn sign_and_serialize(&self, secret: &sign::SecretKey) -> Vec<u8> {
        let mut out = Vec::with_capacity(sign::PUBLICKEYBYTES + sign::SIGNATUREBYTES + kx::PUBLICKEYBYTES);
        out.append(&mut Vec::from(self.signing_pk.as_ref()));

        let mut signed_pk = sign::sign(self.session_pk.as_ref(), secret);
        out.append(&mut signed_pk);

        out
    }
}

impl TryFrom<&[u8]> for SessionHandshakeSodiumExtraData {
    type Error = HandshakeError;

    fn try_from(value: &[u8]) -> HandshakeResult<Self> {
        if sign::PUBLICKEYBYTES + sign::SIGNATUREBYTES + kx::PUBLICKEYBYTES > value.len() {
            Err(HandshakeError::SodiumExtraDataToShort)
        } else {
            let signing_key_bytes = &value[0..sign::PUBLICKEYBYTES];
            let signed_public_key = &value[sign::PUBLICKEYBYTES..];

            let public_signing_key =
                sign::PublicKey::from_slice(signing_key_bytes).ok_or(HandshakeError::InvalidData)?;
            let public_key_bytes = sign::verify(signed_public_key, &public_signing_key)
                .map_err(|_| HandshakeError::WrongSignature)?;
            let public_session_key =
                kx::PublicKey::from_slice(&public_key_bytes).ok_or(HandshakeError::InvalidData)?;
            Ok(SessionHandshakeSodiumExtraData {
                signing_pk: public_signing_key,
                session_pk: public_session_key,
            })
        }
    }
}

pub type HandshakeResult<T> = Result<T, HandshakeError>;

#[derive(Error, Debug, Clone)]
pub enum HandshakeError {
    #[error("Signature check failed.")]
    WrongSignature,
    #[error("Invalid data.")]
    InvalidData,
    #[error("libsodium extra data in handshake is to short")]
    SodiumExtraDataToShort,
}

#[cfg(test)]
mod tests {
    use crate::protocol::handshake::SessionHandshakeSodiumExtraData;
    use sodiumoxide::crypto::{kx, sign};
    use std::convert::TryFrom;

    #[test]
    pub fn sodium_extra_data() {
        let (sign_pk, sign_sk) = sign::gen_keypair();
        let (kx_pk, kx_sk) = kx::gen_keypair();

        let extra = SessionHandshakeSodiumExtraData::new(&sign_pk, kx_pk);
        let mut signed = extra.sign_and_serialize(&sign_sk);

        SessionHandshakeSodiumExtraData::try_from(signed.as_slice()).unwrap();
    }
}
