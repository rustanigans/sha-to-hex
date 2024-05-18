use const_hex::encode as hex_encode;
use ring::hmac::{sign, Key, HMAC_SHA256};
/// Encode a message using secret with HMAC-SHA256 encryption
/// 
/// returns a hex string
///
/// ### Usage
/// `encode(msg: &str, secret: &str) -> String`
///
/// ### Example
/// ```
/// use sha_to_hex::encode;
///
/// let msg = "The quick brown fox jumps over the lazy dog";
/// let secret = "543jklefdsjio342jiofdsoij5234orfjdso";
///
/// assert_eq!(encode(msg, secret),
///            "277288e6f134b423756870fbf222f56c8e17d59d9a2df0f8c535a5ab3f84a9bb")
/// ```
#[inline(always)]
pub fn encode(msg: &str, secret: &str) -> String
{
    encode_bytes(msg.as_bytes(), secret.as_bytes())
}

/// Encode a message using secret with HMAC-SHA256 encryption
/// 
/// returns a hex string
/// 
/// ### Usage
/// `encode_bytes(msg: &[u8], secret: &[u8]) -> String`
#[inline(always)]
pub fn encode_bytes(msg: &[u8], secret: &[u8]) -> String
{
    hex_encode(&sign(&Key::new(HMAC_SHA256, secret), msg))
}

/// [Deprecated] Encode msg using secret return hex string
/// This is just alias of `encode` for backward compatibility
/// 
/// use `encode` instead
#[inline(always)]
pub fn get_sha256_hmac_as_hex_str(msg: &str, secret: &str) -> String
{
    encode(msg, secret)
}
