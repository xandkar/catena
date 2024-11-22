use crate::{block::Block, ledger::Ledger, tx::Tx};

#[derive(Default, Debug)]
pub struct Chain {
    pub blocks: Vec<Block>,
    pub ledger: Ledger,
}

impl Chain {
    pub fn new(txs: &[Tx]) -> anyhow::Result<Self> {
        let genesis = Block::init(txs)?;
        let mut selph = Self::default();
        selph.update(genesis)?;
        Ok(selph)
    }

    pub fn submit(&mut self, txs: &[Tx]) -> anyhow::Result<()> {
        tracing::debug!(?txs, "Submitting transactions.");
        let parent = self.latest_block();
        let mut child = Block::next(&parent, txs)?;
        child.mine(self.ledger.difficulty)?;
        let height = child.height;
        let hash = child.hash.clone();
        self.update(child)?;
        tracing::debug!(?height, ?hash, "Block addedd.");
        Ok(())
    }

    pub fn is_valid(&self) -> anyhow::Result<bool> {
        let blocks_after_genesis = self.blocks.iter().skip(1);
        let block_relations = self.blocks.iter().zip(blocks_after_genesis);
        for (parent, child) in block_relations {
            if (child.hash != child.to_hash()?) || (child.parent_hash != parent.hash) {
                return Ok(false);
            }
        }
        Ok(true)
    }

    fn update(&mut self, block: Block) -> anyhow::Result<()> {
        self.ledger.update(&block)?;
        // XXX Adding block only AFTER we know ledger accepted it!
        self.blocks.push(block);
        Ok(())
    }

    fn latest_block(&self) -> &Block {
        self.blocks
            .last()
            .unwrap_or_else(|| unreachable!("Empty chain: missing genesis."))
    }
}
