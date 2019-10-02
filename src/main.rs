extern crate merklebtree;

use merklebtree::merklebtree::MerkleBTree;

fn main() {
    println!("Hello, world!");

    let mut tree = MerkleBTree::new_with(3, 5);
    //
    //    merklebtree::node::test();
    //
    //    merklebtree::merklebtree::test();
    //
    //    let mut tree = MerkleBTree::new_with(3, 5);
    //
    tree.put(5);
    tree.put(4);
    //
    match tree.get_content() {
        None => println!("no data in the root"),
        Some(T) => {
            println!("have data in the root");
            for i in T.iter() {
                println!("data is {}", i);
            }
        }
    }
}
