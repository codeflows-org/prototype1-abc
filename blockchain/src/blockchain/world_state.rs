use crate::{Account, AccountType, Blockchain};

/// Represents the current state of the blockchain after all Blocks are executed
/// A world state is technically not necessary since we always could build the information
/// by iterating through all the blocks. Generally, this doesn't seem like a good option
/// However, we do not force the actual Blockchain to implement a WorldState but rather
/// behave like having one. This trait therefore just defines an expected interface into our Blockchain
/// (Actually it doesn't even care if we the information is stored within a blockchain)
pub trait WorldState {
    /// Will bring us all registered user ids
    fn get_user_ids(&self) -> Vec<String>;

    /// Will return an account given it id if is available (mutable)
    fn get_account_by_id_mut(&mut self, id: &String) -> Option<&mut Account>;

    /// Will return an account given it id if is available
    fn get_account_by_id(&self, id: &String) -> Option<&Account>;

    /// Will add a new account
    fn create_account(&mut self, id: String, account_type: AccountType)
        -> Result<(), &'static str>;
}

impl WorldState for Blockchain {
    fn get_user_ids(&self) -> Vec<String> {
        self.accounts.keys().map(|s| s.clone()).collect()
    }

    fn get_account_by_id_mut(&mut self, id: &String) -> Option<&mut Account> {
        self.accounts.get_mut(id)
    }

    fn get_account_by_id(&self, id: &String) -> Option<&Account> {
        self.accounts.get(id)
    }

    fn create_account(
        &mut self,
        id: String,
        account_type: AccountType,
    ) -> Result<(), &'static str> {
        return if !self.get_user_ids().contains(&id) {
            let acc = Account::new(account_type);
            self.accounts.insert(id, acc);
            Ok(())
        } else {
            Err("User already exists! (Code: 934823094)")
        };
    }
}
