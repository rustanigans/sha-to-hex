use sha_to_hex::get_sha256_hmac_as_hex_str;

#[test]
fn test () {
    let msg = "The quick brown fox jumps over the lazy dog";
    let secret = "543jklefdsjio342jiofdsoij5234orfjdso";
    assert_eq!(get_sha256_hmac_as_hex_str(msg, secret),
               "277288e6f134b423756870fbf222f56c8e17d59d9a2df0f8c535a5ab3f84a9bb");
}

