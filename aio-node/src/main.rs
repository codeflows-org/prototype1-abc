use blockchain::{Block, Blockchain, Transaction, TransactionData};

fn main() {
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
}
