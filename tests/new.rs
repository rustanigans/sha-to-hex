
#[test]
fn test () {
    let msg = "The quick brown fox jumps over the lazy dog";
    let secret = "543jklefdsjio342jiofdsoij5234orfjdso";
    assert_eq!(sha_to_hex::encode(msg, secret),
               "277288e6f134b423756870fbf222f56c8e17d59d9a2df0f8c535a5ab3f84a9bb");
}
#[test]
fn test_bytes () {
    let msg = "The quick brown fox jumps over the lazy dog";
    let secret = "543jklefdsjio342jiofdsoij5234orfjdso";
    assert_eq!(sha_to_hex::encode_bytes(msg.as_bytes(), secret.as_bytes()),
               "277288e6f134b423756870fbf222f56c8e17d59d9a2df0f8c535a5ab3f84a9bb");
}
