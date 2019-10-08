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
        m: 0,
    };
    let mut tree = MerkleBTree::new_with(5, Item { key: 1, value: 4 }, &mut nodes);

    nodes.m = tree.m;

    for i in 0..7 {
        let item = Item { key: i, value: i };
        tree.put(item, &mut nodes);
        println!("total node:{}", nodes.size);
    }

    println!("-----------------------------------");
    let node = nodes.nodes_map.get_mut(&nodes.root_id).unwrap();

    match node.get_content() {
        None => println!("no data in the root"),
        Some(T) => {
            println!("nodeid:{}", nodes.root_id);
            println!("have data in the root");
            for i in T.iter() {
                println!("data is {:?}", i);
            }
        }
    }

    println!("-----------------------------------");
    let node = nodes.nodes_map.get_mut(&3).unwrap();

    match node.get_content() {
        None => println!("no data in the node"),
        Some(T) => {
            println!("nodeid:{}", 3);
            println!("have data in the node");
            for i in T.iter() {
                println!("data is {:?}", i);
            }
        }
    }

    println!("-----------------------------------");
    let node = nodes.nodes_map.get_mut(&4).unwrap();

    match node.get_content() {
        None => println!("no data in the node"),
        Some(T) => {
            println!("nodeid:{}", 4);
            println!("have data in the node");
            for i in T.iter() {
                println!("data is {:?}", i);
            }
        }
    }

    tree.put(Item { key: 0, value: 1 }, &mut nodes);
    println!("-----------------------------------");
    let node = nodes.nodes_map.get_mut(&1).unwrap();

    match node.get_content() {
        None => println!("no data in the node"),
        Some(T) => {
            println!("nodeid:{}", 1);
            println!("have data in the node");
            for i in T.iter() {
                println!("data is {:?}", i);
            }
        }
    }

    println!("--------------remove the content from internal node---------------------");
    tree.remove(nodes.root_id, Item { key: 2, value: 2 }, &mut nodes);

    println!("-----------------------------------");
    let node = nodes.nodes_map.get_mut(&0).unwrap();

    match node.get_content() {
        None => println!("no data in the node"),
        Some(T) => {
            println!("nodeid:{}", 0);
            println!("have data in the node");
            for i in T.iter() {
                println!("data is {:?}", i);
            }
        }
    }

    let node = nodes.nodes_map.get_mut(&1).unwrap();

    match node.get_content() {
        None => println!("no data in the node"),
        Some(T) => {
            println!("nodeid:{}", 1);
            println!("have data in the node");
            for i in T.iter() {
                println!("data is {:?}", i);
            }
        }
    }

    println!("--------------remove the content from leaf node---------------------");
    tree.remove(nodes.root_id, Item { key: 0, value: 1 }, &mut nodes);

    println!("-----------------------------------");
    let node = nodes.nodes_map.get_mut(&1).unwrap();

    match node.get_content() {
        None => println!("no data in the node"),
        Some(T) => {
            println!("nodeid:{}", 1);
            println!("have data in the node");
            for i in T.iter() {
                println!("data is {:?}", i);
            }
        }
    }
}
