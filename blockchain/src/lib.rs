use chrono::prelude::*;

pub struct Block {
    /// The index of the block. The first block (aka _Genesis block_) has index `1`.
    pub index: u128,
    /// The moment (Unix time) the block has been created.
    pub timestamp: i64,
    /// The proof (aka nonce) of the block.
    pub proof: u32,
    /// The hash of the previous block.
    pub previous_hash: [char; 64],
}

pub struct Blockchain {
    pub chain: Vec<Block>,
}

impl Blockchain {
    pub fn new() -> Blockchain {
        let genesis_block = Block {
            index: 1,
            timestamp: Utc::now().timestamp_millis(),
            proof: 1,
            previous_hash: ['0'; 64],
        };
        Blockchain {
            chain: vec![genesis_block],
        }
    }
}

// -------------------------
//        unit tests
// -------------------------

#[cfg(test)]
mod tests {

    use crate::Blockchain;

    #[test]
    fn creation() {
        let bc = Blockchain::new();
        assert_eq!(1, bc.chain[0].index);
    }
}
