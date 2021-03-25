// Registering to the module tree.
mod block;
mod chain;

// Exporting to the public with a simple path.
pub use block::Block;
pub use chain::Blockchain;
