/*
 File: crypto.rs
 Created Date: 07 Sep 2022
 Author: realbacon
 -----
 Last Modified: 8/09/2022 08:56:55
 Modified By: realbacon
 -----
 License  : MIT
 -----
*/
use hex::decode_to_slice;
use hmac::{Hmac, Mac};
use sha2::Sha256;
type HmacSha256 = Hmac<Sha256>;

pub fn decodeb64(b64: &String) -> String {
    let decoded = base64::decode(b64).unwrap();
    let decoded = String::from_utf8(decoded).unwrap();
    return decoded;
}

pub fn verify_signature(
    hash: &String,
    payload: String,
    key: String,
) -> Result<(bool, String), std::io::Error> {
    let mut mac = HmacSha256::new_from_slice(key.as_bytes()).expect("HMAC need a valid key");

    mac.update(payload.as_bytes());

    let mut code_bytes = [0u8; 32];
    decode_to_slice(hash, &mut code_bytes as &mut [u8]).expect("Invalid hash");
    Ok((mac.verify_slice(&code_bytes[..]).is_ok(), payload))
}
