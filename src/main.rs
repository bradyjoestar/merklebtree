extern crate merklebtree;
use merklebtree::merklebtree::{MerkleBTree, Nodes};

mod bean;
use bean::Item;

fn main() {
    println!("Hello, world!");

    let mut nodes = Nodes {
        nodes: Default::default(),
        number: 0,
    };
    let mut tree = MerkleBTree::new_with(5, Item { key: 1, value: 4 });

    for i in 0..4 {
        let item = Item { key: i, value: i };
        tree.put(item, &mut nodes);
    }

    match tree.get_content() {
        None => println!("no data in the root"),
        Some(T) => {
            println!("have data in the root");
            for i in T.iter() {
                println!("data is {:?}", i);
            }
        }
    }
}
