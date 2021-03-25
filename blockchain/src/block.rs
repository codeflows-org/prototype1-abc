use std::fmt;

const HASH_BYTE_SIZE: usize = 32;

pub type Sha256Hash = [u8; HASH_BYTE_SIZE];

// #[derive(Debug)]
pub struct Block {
    /// The index of the block. The first block (aka _Genesis block_) has index `1`.
    pub index: u128,

    /// The moment (Unix time) the block has been created.
    pub timestamp: i64,

    /// The previous block's hash.
    pub previous_block_hash: Sha256Hash,

    /// The proof (aka nonce) of the block.
    pub proof: u32,

    /// Block data (no explicit transaction here, at least not yet)
    pub data: Vec<u8>,
}

// Custom implementation of the `Debug` trait.
impl fmt::Debug for Block {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Block {{ idx: {}, ts: {}, prf: {}, pbh: '{}' }}",
            &self.index,
            &self.timestamp,
            &self.proof,
            pretty(&self.previous_block_hash)
        )
    }
}

fn pretty(h: &Sha256Hash) -> String {
    String::from_utf8_lossy(h).to_string()
}
