pub fn get_sha256_hmac_as_hex_str(msg: &str, secret: &str) -> String
{
    let key = ring::hmac::Key::new(ring::hmac::HMAC_SHA256, secret.as_bytes());
    let sign = ring::hmac::sign(&key, msg.as_bytes());
    hex::encode(sign.as_ref())
}
