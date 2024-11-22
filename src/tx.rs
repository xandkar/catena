use ed25519_dalek::{ed25519::signature::SignerMut, Signature, SigningKey, VerifyingKey};

use crate::key;

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct Tx {
    pub op: Op,
    pub src: String,
    sig: SerializableSignature,
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub enum Op {
    SetDifficulty(usize),
    Msg(String),
    Coinbase { dst: String, amount: u64 },
    Pay { dst: String, amount: u64 },
}

impl Tx {
    pub fn new(src_key: &mut SigningKey, op: Op) -> anyhow::Result<Self> {
        let src_addr = key::pub_to_hex(&src_key.verifying_key());
        let msg = signable(&src_addr, &op)?;
        let sig = src_key.sign(&msg[..]);
        let sig = SerializableSignature(sig.to_vec());
        let src = src_addr.to_string();
        Ok(Self { op, src, sig })
    }

    pub fn is_valid(&self, admin: &VerifyingKey) -> bool {
        self.validate(admin).is_ok()
    }

    pub fn validate(&self, admin: &VerifyingKey) -> anyhow::Result<()> {
        let sig = Signature::try_from(&self.sig)?;
        let src_pubkey = match &self.op {
            Op::SetDifficulty(_) | Op::Msg(_) | Op::Coinbase { .. } => admin.clone(),
            Op::Pay { .. } => {
                let bytes = hex::decode(&self.src)?;
                let bytes: [u8; 32] = bytes.as_slice().try_into()?;
                let pubkey = VerifyingKey::from_bytes(&bytes)?;
                pubkey
            }
        };
        let msg = signable(&self.src, &self.op)?;
        src_pubkey.verify_strict(&msg[..], &sig)?;
        Ok(())
    }
}

fn signable(src: &str, op: &Op) -> anyhow::Result<Vec<u8>> {
    let msg = serde_json::to_vec(&(src, op))?;
    Ok(msg)
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct SerializableSignature(#[serde(with = "serde_bytes")] Vec<u8>);

impl From<Signature> for SerializableSignature {
    fn from(sig: Signature) -> Self {
        SerializableSignature(sig.to_bytes().to_vec())
    }
}

impl TryFrom<&SerializableSignature> for Signature {
    type Error = anyhow::Error;

    fn try_from(wrapper: &SerializableSignature) -> Result<Self, Self::Error> {
        let bytes: &[u8] = wrapper.0.as_slice();
        let sig = Signature::from_slice(bytes)?;
        Ok(sig)
    }
}
