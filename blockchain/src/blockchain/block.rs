use std::fmt;

use blake2::{Blake2b, Digest};

use crate::Transaction;

const _HASH_BYTE_SIZE: usize = 32;

pub type _Sha256Hash = [u8; _HASH_BYTE_SIZE];

#[derive(Clone)]
pub struct Block {
    /// Actions that this block includes.<br/>
    /// There has to be at least one.
    pub(crate) transactions: Vec<Transaction>,

    /// This actually connects the blocks together
    pub(crate) prev_hash: Option<String>,

    /// We store the hash of the block here also in order to
    /// save the last block from being tampered with later on
    pub(crate) hash: Option<String>,

    /// Some arbitrary number which will be later used for Proof of Work
    nonce: u128,
}

// Custom implementation of the `Debug` trait (insted of using
// the default implementation provided by `#[derive(Debug)]`).
impl fmt::Debug for Block {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Block {{ hash: {:?}, phash: {:?}, nonce: {}, txns: {:?} }}",
            &self.hash, &self.prev_hash, &self.nonce, &self.transactions
        )
    }
}

impl Block {
    pub fn new(prev_hash: Option<String>) -> Self {
        Block {
            nonce: 0,
            hash: None,
            prev_hash,
            transactions: Vec::new(),
        }
    }

    /// Changes the nonce number and updates the hash
    pub fn set_nonce(&mut self, nonce: u128) {
        self.nonce = nonce;
        self.update_hash();
    }

    /// Calculate the hash of the whole block including transactions.<br/>
    /// It is using Blake2 hasher.
    pub fn calculate_hash(&self) -> Vec<u8> {
        let mut hasher = Blake2b::new();

        for transaction in self.transactions.iter() {
            hasher.update(transaction.calculate_hash())
        }

        let block_as_string = format!("{:?}", (&self.prev_hash, &self.nonce));
        hasher.update(&block_as_string);

        return Vec::from(hasher.finalize().as_ref());
    }

    /// Appends a transaction to the queue
    pub fn add_transaction(&mut self, transaction: Transaction) {
        self.transactions.push(transaction);
        self.update_hash();
    }

    /// Will return the amount of transactions
    pub fn get_transaction_count(&self) -> usize {
        self.transactions.len()
    }

    /// Will update the hash field by including all transactions currently inside
    /// the public modifier is only for the demonstration of attacks
    pub(crate) fn update_hash(&mut self) {
        self.hash = Some(byte_vector_to_string(&self.calculate_hash()));
    }

    /// Checks if the hash is set and matches the blocks internals.
    pub fn verify_own_hash(&self) -> bool {
        if self.hash.is_some() && // Hash is set.
            self.hash.as_ref().unwrap().eq(
                &byte_vector_to_string(
                    &self.calculate_hash()))
        {
            // Hash equals calculated hash

            return true;
        }
        false
    }
}

/// Will take an array of bytes and transform it into a string by interpreting every byte
/// as an character
fn byte_vector_to_string(arr: &Vec<u8>) -> String {
    arr.iter().map(|&c| c as char).collect()
}
