use sha2::{Digest, Sha256};

use crate::tx::Tx;

#[derive(Debug)]
pub struct Block {
    pub height: u64,
    pub hash: String,
    pub parent_hash: String,
    pub txs: Vec<Tx>,
    pub nonce: u64,
}

impl Block {
    /// Genesis
    pub fn init(txs: &[Tx]) -> anyhow::Result<Self> {
        Self::new("0", 0, txs)
    }

    /// Normal
    pub fn next(prev: &Block, txs: &[Tx]) -> anyhow::Result<Self> {
        Self::new(prev.hash.as_str(), prev.height + 1, txs)
    }

    fn new(parent_hash: &str, height: u64, txs: &[Tx]) -> anyhow::Result<Self> {
        let mut selph = Self {
            height,
            txs: txs.to_vec(),
            parent_hash: parent_hash.to_string(),
            hash: String::new(),
            nonce: 0,
        };
        selph.hash = selph.to_hash()?;
        Ok(selph)
    }

    pub fn to_hash(&self) -> anyhow::Result<String> {
        let Self {
            height,
            txs,
            parent_hash,
            hash: _,
            nonce,
        } = self;
        let data = serde_json::to_string(&(height, parent_hash, nonce, txs))?;
        let mut hasher = Sha256::new();
        hasher.update(data.as_bytes());
        let hex_digest = format!("{:x}", hasher.finalize());
        Ok(hex_digest)
    }

    pub fn mine(&mut self, difficulty: usize) -> anyhow::Result<()> {
        let target = "0".repeat(difficulty);
        while &self.hash[0..difficulty] != target {
            self.nonce += 1;
            self.hash = self.to_hash()?;
        }
        tracing::debug!(hash = self.hash, "Block mined.");
        Ok(())
    }
}
