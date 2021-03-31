use std::collections::HashMap;

use crate::{Account, Block, Transaction};

#[derive(Debug, Clone)]
pub struct Blockchain {
    /// The blocks which were accepted in the blockchain.
    pub blocks: Vec<Block>,

    /// Lookup from AccountID (will be a public key later) to Account.
    /// Effectively, this represents the WorldState
    pub accounts: HashMap<String, Account>,

    /// Will store transactions which should be added to the chain
    /// but aren't yet
    pending_transactions: Vec<Transaction>,
}

impl Blockchain {
    /// C'tor.
    pub fn new() -> Self {
        Blockchain {
            blocks: Vec::new(),
            accounts: HashMap::new(),
            pending_transactions: Vec::new(),
        }
    }

    /// Will add a block to the Blockchain
    /// @TODO every simple step could be refactored into a separate function for
    /// better testability and code-reusability
    pub fn append_block(&mut self, block: Block) -> Result<(), String> {
        // The genesis block may create user out of nowhere,
        // and also may do some other things
        let is_genesis = self.len() == 0;

        // Check if the hash matches the transactions
        if !block.verify_own_hash() {
            return Err("The block hash is mismatching! (Code: 93820394)".into());
        }

        // Check if the newly added block is meant to be appended onto the last block
        if !(block.prev_hash == self.get_last_block_hash()) {
            return Err("The new block has to point to the previous block (Code: 3948230)".into());
        }

        // There has to be at least one transaction inside the queue
        if block.get_transaction_count() == 0 {
            return Err("There has to be at least one transaction \
                inside the block! (Code: 9482930)"
                .into());
        }

        // Reject block having nonces that are already used (Prevent reply attacks etc.)
        // @Todo (Will skip that for simplicity)

        // This is expensive and just used for rollback if some transactions succeed whilst
        // others don't (prevent inconsistent states)
        // Arguably, that could be implemented more resource-aware
        let old_state = self.accounts.clone();

        // Execute each transaction
        for (i, transaction) in block.transactions.iter().enumerate() {
            // Execute the transaction
            if let Err(err) = transaction.execute(self, &is_genesis) {
                // Recover state on failure.
                self.accounts = old_state;

                // ... and reject the block
                return Err(format!(
                    "Could not execute transaction {} due to `{}`. Rolling back \
                    (Code: 38203984)",
                    i + 1,
                    err
                ));
            }
        }

        // Everything went fine... append the block
        self.blocks.push(block);

        Ok(())
    }

    /// Will return the amount of blocks currently stored
    pub fn len(&self) -> usize {
        self.blocks.len()
    }

    /// Will return the hash of the last block
    pub fn get_last_block_hash(&self) -> Option<String> {
        if self.len() == 0 {
            return None;
        }

        self.blocks[self.len() - 1].hash.clone()
    }

    /// Checks if the blockchain was tempered with
    /// It will check until the first error happens and return a description of the problem
    /// if everything is fine it will return Ok
    pub fn check_validity(&self) -> Result<(), String> {
        for (block_num, block) in self.blocks.iter().enumerate() {
            // Check if block saved hash matches to calculated hash
            if !block.verify_own_hash() {
                return Err(format!(
                    "Stored hash for Block #{} \
                        does not match calculated hash (Code: 665234234)",
                    block_num + 1
                )
                .into());
            }

            // Check previous black hash points to actual previous block
            if block_num == 0 {
                // Genesis block should point to nowhere
                if block.prev_hash.is_some() {
                    return Err("The genesis block has a previous hash set which \
                         it shouldn't Code :394823098"
                        .into());
                }
            } else {
                // Non genesis blocks should point to previous blocks hash (which is validated before)
                if block.prev_hash.is_none() {
                    return Err(format!("Block #{} has no previous hash set", block_num + 1).into());
                }

                // Store the values locally to use them within the error message on failure
                let prev_hash_proposed = block.prev_hash.as_ref().unwrap();
                let prev_hash_actual = self.blocks[block_num - 1].hash.as_ref().unwrap();

                if !(&block.prev_hash == &self.blocks[block_num - 1].hash) {
                    return Err(format!(
                        "Block #{} is not connected to previous block (Hashes do \
                        not match. Should be `{}` but is `{}`)",
                        block_num, prev_hash_proposed, prev_hash_actual
                    )
                    .into());
                }
            }

            // Check if transactions are signed correctly
            for (transaction_num, transaction) in block.transactions.iter().enumerate() {
                // Careful! With that implementation an unsigned message will always
                // be valid! You may remove the first check to only accept signed transactions
                if transaction.is_signed() && !transaction.check_signature() {
                    return Err(format!(
                        "Transaction #{} for Block #{} has an invalid signature \
                        (Code: 4398239048)",
                        transaction_num + 1,
                        block_num + 1
                    ));
                }
            }
        }
        Ok(())
    }
}

// -------------------------
//        unit tests
// -------------------------

#[cfg(test)]
mod tests {

    use std::borrow::BorrowMut;

    use crate::{Block, Blockchain, Transaction, TransactionData};

    #[test]
    fn creation() {
        let bc = Blockchain::new();
        assert_eq!(0, bc.blocks.len());
    }

    #[test]
    fn create_and_alter() {
        // Create a new Blockchain
        let mut bc = Blockchain::new();

        // Create an empty block (first block has no prev_block)
        let mut genesis = Block::new(None);

        let initial_users = vec!["alice", "bob"];

        for user in initial_users {
            let create_account_txn = Transaction::new(
                user.into(),
                TransactionData::CreateUserAccount(user.into()),
                0,
            );

            let create_token_txn = Transaction::new(
                user.into(),
                TransactionData::CreateTokens {
                    receiver: user.into(),
                    amount: 100_000_000,
                },
                0,
            );

            genesis.add_transaction(create_account_txn);

            genesis.add_transaction(create_token_txn);
        }

        let mut res = bc.append_block(genesis);
        println!("Genesis block successfully added: {:?}", res);
        println!("Full blockchain printout");
        println!("{:#?}", bc);

        // Transfer 1 token from alice to bob
        let mut block2 = Block::new(bc.get_last_block_hash());
        block2.add_transaction(Transaction::new(
            "alice".into(),
            TransactionData::TransferTokens {
                to: "bob".into(),
                amount: 1,
            },
            0,
        ));

        res = bc.append_block(block2);
        println!("Block added: {:?}", res);
        println!("Full blockchain printout");
        println!("{:#?}", bc);
        println!("Blockchain valid: {:?}", bc.check_validity());

        // Everything is fine until here

        // Attack I: changing a transaction
        // Let's tamper the block chain. Maybe bob was not satisfied with the amount of coins alice sent
        // him, so he will tamper the blockchains transaction to transmit 100 Coins instead of 1

        // let's clone the current blockchain before tempering
        let mut bc_attack_1 = bc.clone();
        // get the transaction as mutable (second block, first transaction; the token transfer)
        let transaction_data = bc_attack_1.blocks[1].transactions[0].borrow_mut();

        // change the amount value of the transaction INSIDE the chain
        match transaction_data.record.borrow_mut() {
            &mut TransactionData::TransferTokens {
                to: _,
                ref mut amount,
            } => {
                *amount = 100; // Actually change the value in place
            }

            _ => {} // We know that that recors is a TransferToken Action so we ignore the rest
        }

        println!("Changed transaction: {:?}", transaction_data.record);

        // Will print an error, since the blocks hash changes for the
        println!(
            "Is the Blockchain still valid? {:#?}",
            bc_attack_1.check_validity()
        );

        // Attack II: Changing transaction + updating the hash (increasing initial tokens in create
        // user action)
        let mut bc_attack_2 = bc.clone();

        // Alice tokens
        let transaction_data = bc_attack_2.blocks[0].transactions[1].borrow_mut();

        // change tokens
        match transaction_data.record.borrow_mut() {
            &mut TransactionData::CreateTokens {
                receiver: _,
                ref mut amount,
            } => {
                *amount = 100_000_000_000; // Let's dont be small on that
            }
            _ => {} // We know that that record is a Token Create Action so we ignore the rest
        }

        // If we execute now, we'll see the same error as above, hashes dont match (this time 1st block)

        // Will print an error, since the blocks hash changes for the
        println!(
            "Is the Blockchain still valid? {:#?}",
            bc_attack_2.check_validity()
        );

        // But alice was smart, she also updated the first blocks' hash
        bc_attack_2.blocks[0].update_hash();

        // So the hash is correct now, however, block2 points now to sth which does not exists
        // Again, the blockchain is invalid but for a different reason
        println!(
            "Is the Blockchain still valid? {:#?}",
            bc_attack_2.check_validity()
        );
    }
}
