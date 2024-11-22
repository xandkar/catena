use std::collections::HashMap;

use anyhow::bail;
use ed25519_dalek::VerifyingKey;

use crate::{block::Block, tx};

#[derive(Default, Debug)]
pub struct Ledger {
    pub height: u64,
    pub accounts: HashMap<String, u64>,
    pub messages: Vec<String>,
    pub difficulty: usize,
    admin: VerifyingKey,
}

impl Ledger {
    pub fn new(admin: VerifyingKey) -> Self {
        let mut selph = Self::default();
        selph.admin = admin;
        selph
    }

    pub fn update(&mut self, block: &Block) -> anyhow::Result<()> {
        match (block.height, self.height) {
            (0, 0) => {}
            (block, ledger) if block == ledger + 1 => {}
            (block, ledger) => {
                bail!(
                    "Invalid block height: {}. ledger={}, expected={}.",
                    block,
                    ledger,
                    ledger + 1,
                );
            }
        }
        for tx in &block.txs {
            if !tx.is_valid(&self.admin) {
                bail!("Invalid signature: {tx:?}");
            }
            match &tx.op {
                tx::Op::Msg(msg) => self.messages.push(msg.to_string()),
                tx::Op::SetDifficulty(n) => self.difficulty = *n,
                tx::Op::Coinbase { .. } if block.height > 0 => {
                    bail!("Coinbase is only allowed at Genesis.")
                }
                tx::Op::Coinbase { dst, amount } => {
                    self.accounts.insert(dst.to_string(), *amount);
                }
                tx::Op::Pay { dst, amount } => match self.accounts.get_mut(&tx.src) {
                    None => {
                        bail!("src account not found: {:?}", &tx.src);
                    }
                    Some(src_balance) if *src_balance >= *amount => {
                        *src_balance -= *amount;
                        self.accounts
                            .entry(dst.to_string())
                            .and_modify(|dst_balance| *dst_balance += *amount)
                            .or_insert(*amount);
                    }
                    Some(balance) => {
                        bail!(
                            "insufficient funds in src account: \
                            src={:?}, \
                            balance={balance:?}, \
                            amount={amount:?}.",
                            &tx.src
                        );
                    }
                },
            }
        }
        self.height = block.height;
        Ok(())
    }
}
