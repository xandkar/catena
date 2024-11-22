use ed25519_dalek::VerifyingKey;

pub fn pub_to_hex(pubkey: &VerifyingKey) -> String {
    pubkey
        .as_bytes()
        .iter()
        .map(|byte| format!("{byte:02x}"))
        .collect()
}
