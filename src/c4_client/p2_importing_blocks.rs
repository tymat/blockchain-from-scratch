//! We being implementing our client with the most fundamental task, which is importing
//! blocks and headers. Full clients import entire blocks while light clients only import headers.

use super::{Block, Consensus, FullClient, StateMachine};

/// A trait that represents the ability to import complete blocks of the chain.
///
/// The main method here is `import_block` but several other methods are provided
/// to access data about imported blocks.
pub trait ImportBlock<C: Consensus, SM: StateMachine> {
    /// Attempt to import a block.
    /// Returns whether the import was successful or not.
    fn import_block(&mut self, _: Block<C, SM>) -> bool;

    /// Retrieve the full body of an imported block.
    /// Returns None if the block is not known.
    fn get_block(&self, block_hash: u64) -> Option<Block<C, SM>>;

    /// Retrieve the state associated with a given block.
    /// Returns None if the block is not known.
    fn get_state(&self, block_hash: u64) -> Option<SM::State>;

    /// Check whether a given block is a leaf (aka tip) of the chain.
    /// A leaf block has no known children.
    /// Returns None if the block is not known.
    fn is_leaf(&self, block_hash: u64) -> Option<bool>;

    /// Get a list of all the leaf nodes in the chain.
    fn all_leaves(&self) -> Vec<u64>;
}

impl<C, SM, FC, P> ImportBlock<C, SM> for FullClient<C, SM, FC, P>
where
    C: Consensus,
    SM: StateMachine,
{
    fn import_block(&mut self, _: Block<C, SM>) -> bool {
        todo!("Exercise 1")
    }

    fn get_block(&self, block_hash: u64) -> Option<Block<C, SM>> {
        todo!("Exercise 2")
    }

    fn get_state(&self, block_hash: u64) -> Option<<SM as StateMachine>::State> {
        todo!("Exercise 3")
    }

    fn is_leaf(&self, block_hash: u64) -> Option<bool> {
        todo!("Exercise 4")
    }

    fn all_leaves(&self) -> Vec<u64> {
        todo!("Exercise 5")
    }
}

// TODO Write these tests.

// Test ideas:
// import valid block
// import block with unknown parent
// import block with invalid height
// import block with invalid state root
// import block with invalid transactions root
// import block with invalid seal

// Try to get_block genesis block
// Try to get_block an unknown block
// Import a valid block then make sure you can get it

// Check whether genesis is a leaf right away
// Import a block then check whether genesis is a leaf
// Import two blocks in a single chain and make sure the leaf statuses is right.
// Import a forked chain and make sure both leaves' statuses are right.

// Same previous 4 scenarios except with the `all_leaves` method.
