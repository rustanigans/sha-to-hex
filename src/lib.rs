use hex::encode;
use ring::hmac::{sign, Key, HMAC_SHA256};
/// Encode msg using secret return hex string
///
/// # Usage
/// `get_sha256_hmac_as_hex_str(msg: &str, secret: &str) -> String`
///
/// # Example
/// ```
/// use sha_to_hex::get_sha256_hmac_as_hex_str as encode;
///
/// let msg = "The quick brown fox jumps over the lazy dog";
/// let secret = "543jklefdsjio342jiofdsoij5234orfjdso";
///
/// assert_eq!(encode(msg, secret),
///            "277288e6f134b423756870fbf222f56c8e17d59d9a2df0f8c535a5ab3f84a9bb")
/// ```
pub fn get_sha256_hmac_as_hex_str(msg: &str, secret: &str) -> String
{
    let key = Key::new(HMAC_SHA256, secret.as_bytes());
    let sign = sign(&key, msg.as_bytes());
    encode(sign.as_ref())
}
