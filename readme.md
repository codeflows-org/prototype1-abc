## Prototype 1 of Akasha BlockChain (ABC)

The 1st prototype of ABC using a Rust implementation.

_state: WIP_

For now (and most probably just temporary), it is based on [this](https://hackernoon.com/rusty-chains-a-basic-blockchain-implementation-written-in-pure-rust-gk2m3uri) article.

<br/>

### Repo Structure

The repo follows the standard Cargo's workspace structure. It includes:

- `blockchain` as a reusable library
- `aio-node` as a all-in-one node
  - storing (in-memory) the blockchain
  - and mining new blocks

<br/>

### Examples

There are two usage examples:

- An example of creating and using it from the outside (or as a user of `blockchain` module) exists in `aio-node/src/main.rs` file.
- An example of crating, using, and altering it to showcase that any tampering is easily detectable exists in `blockchain/src/blockchain/chain.rs` file, as a (unit) test.
