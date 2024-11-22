use std::collections::HashMap;

use anyhow::bail;

use crate::{block::Block, tx::Tx};

#[derive(Default, Debug)]
pub struct Ledger {
    pub height: u64,
    pub accounts: HashMap<String, u64>,
    pub messages: Vec<String>,
    pub difficulty: usize,
}

impl Ledger {
    pub fn new() -> Self {
        Self::default()
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
            match tx {
                Tx::Msg(msg) => self.messages.push(msg.to_string()),
                Tx::SetDifficulty(n) => self.difficulty = *n,
                Tx::Coinbase { .. } if block.height > 0 => {
                    bail!("Coinbase is only allowed at Genesis.")
                }
                Tx::Coinbase { dst, amount } => {
                    self.accounts.insert(dst.to_string(), *amount);
                }
                Tx::Pay { src, dst, amount } => match self.accounts.get_mut(src) {
                    None => {
                        bail!("src account not found: {src:?}");
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
                            src={src:?}, \
                            balance={balance:?}, \
                            amount={amount:?}."
                        );
                    }
                },
            }
        }
        self.height = block.height;
        Ok(())
    }
}
