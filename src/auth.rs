use errors::*;
use hex::encode;
use ring::hmac;
use std::time::{SystemTime, UNIX_EPOCH};

pub fn sign_payload(secret: &[u8], payload: &[u8]) -> Result<String> {
    let signed_key = hmac::Key::new(hmac::HMAC_SHA384, secret);
    let signature = encode(hmac::sign(&signed_key, payload).as_ref());

    Ok(signature)
}

pub fn generate_nonce() -> Result<String> {
    let start = SystemTime::now();
    let since_epoch = start.duration_since(UNIX_EPOCH)?;

    let timestamp = since_epoch.as_millis() * 1000 as u128;
    Ok(timestamp.to_string())
}