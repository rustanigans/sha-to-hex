use hex::encode;
use ring::hmac::{sign, Key, HMAC_SHA256};

pub fn get_sha256_hmac_as_hex_str(msg: &str, secret: &str) -> String
{
    let key = Key::new(HMAC_SHA256, secret.as_bytes());
    let sign = sign(&key, msg.as_bytes());
    encode(sign.as_ref())
}
