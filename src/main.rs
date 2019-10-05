extern crate merklebtree;
use merklebtree::merklebtree::{MerkleBTree, Nodes};

mod bean;
use bean::Item;

fn main() {
    println!("Hello, world!");

    let mut nodes = Nodes {
        nodes_map: Default::default(),
        size: 0,
        root_id: 0,
        next_id: 0,
    };
    let mut tree = MerkleBTree::new_with(5, Item { key: 1, value: 4 }, &mut nodes);

    for i in 0..4 {
        let item = Item { key: i, value: i };
        tree.put(item, &mut nodes);
    }

    let node = nodes.nodes_map.get_mut(&nodes.root_id).unwrap();

    match node.get_content() {
        None => println!("no data in the root"),
        Some(T) => {
            println!("have data in the root");
            for i in T.iter() {
                println!("data is {:?}", i);
            }
        }
    }
}
