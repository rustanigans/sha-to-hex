# Sha-to-hex

Simple function to encode string using secret and return hex string

```
use sha_to_hex::encode;

let encoded = encode(msg, secret);
```

To encode bytes use encode_bytes:
```
use sha_to_hex::encode_bytes;

let encoded = encode_bytes(msg_as_bytes, secret_as_bytes);
```

For full usage see docs.rs: [encode](https://docs.rs/sha-to-hex/latest/sha_to_hex/fn.encode.html)
