// Declaring the module.
mod blockchain;

// Exporting to the public with a simple path.
pub use blockchain::account::{Account, AccountType};
pub use blockchain::block::Block;
pub use blockchain::chain::Blockchain;
pub use blockchain::transaction::{Transaction, TransactionData};
