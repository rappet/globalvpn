Crypto Handshake
----------------

Both sides:
Send handshake packet.

| Type     | Content                                                |
| -------- | ------------------------------------------------------ |
| u16      | Magic Number 0xf00f                                    |
| u8       | len `n` of supported cryptography protocols            |
| `n` x u8 | id      of supported cryptography protocol             |
| u8       | len `m` of additional data fields                      |
| -        | `m` [addidtional data fields](#additional-data-fields) |

### Additional data fields

Each data field is prepended in a header which describes its type and length.

| Type  | Content                |
| ----- | ---------------------- |
| u16   | id of additional field |
| u16   | len of field           |
| `len` | content of field       |

#### node NaCl public signing key (ID 1)

Public key, crypto_sign_PUBLICKEYBYTES bytes long

#### public session key (ID 2)

Public session key, signed using

```c
crypto_sign(signed_key, &signed_key_len,
signing_key, &sizeof(signing_key), sk);
```

### NaCl key exchange (protocol 1)

- Send own public signing key information (both sides) + signed
public encryption key with extension 1 and 2
- Send encrypted symmetric key to other side.
