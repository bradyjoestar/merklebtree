extern crate merklebtree;

use merklebtree::merklebtree::MerkleBTree;

fn main() {
    println!("Hello, world!");

    merklebtree::tree::test();

    merklebtree::merklebtree::test();

    MerkleBTree::new_with(3, 5);
}
