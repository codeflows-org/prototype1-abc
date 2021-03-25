use chrono::prelude::*;

use crate::block::Block;

#[derive(Debug)]
pub struct Blockchain {
    pub chain: Vec<Block>,
}

impl Blockchain {
    pub fn new() -> Blockchain {
        let genesis_block = Block {
            index: 1,
            timestamp: Utc::now().timestamp_millis(),
            proof: 1,
            previous_block_hash: [0; 32],
            data: vec![],
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
