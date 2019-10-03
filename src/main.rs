extern crate merklebtree;
use merklebtree::merklebtree::MerkleBTree;

mod bean;
use bean::Item;

fn main() {
    println!("Hello, world!");

    let mut tree = MerkleBTree::new_with(3, Item { key: 1, value: 3 });

    for i in 0..5 {
        let item = Item { key: i, value: i };
        tree.put(item);
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
