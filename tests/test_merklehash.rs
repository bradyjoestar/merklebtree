extern crate ring;
use merklebtree::node::Node;
use ring::digest;
use ring::digest::Digest;

mod utils;
use merklebtree::merklebtree::{MerkleBTree, Nodes};
use merklebtree::traits::CalculateHash;
use std::collections::HashMap;
use utils::*;

#[test]
fn test_merkle_put_0() {
    let hash = digest::digest(&digest::SHA256, b"hello, world");
    let hex_data = hex::encode(hash.as_ref());
    println!("{:?}", hex_data);
    assert_eq!(
        "SHA256:09ca7e4eaa6e8ae9c7d261167129184883644d\
         07dfba7cbfbc4c8a2e08360d5b",
        &format!("{:?}", digest::digest(&digest::SHA256, b"hello, world"))
    );
}

#[test]
fn test_merkle_put_1() {
    let item = Item2 { key: 1, value: 2 };
    let node = Node::new_node(Item2 { key: 1, value: 2 }, 0);
    assert_eq!(
        node.hash,
        String::from("e0bc614e4fd035a488619799853b075143deea596c477b8dc077e309c0fe42e9")
    );
}

#[test]
fn test_merkle_put_2() {
    let mut nodes_map: HashMap<i32, Node<Item2>> = HashMap::new();
    let mut nodes = Nodes {
        nodes_map,
        size: 0,
        content_size: 0,
        root_id: 0,
        next_id: 0,
        m: 0,
    };
    let mut tree = MerkleBTree::new_empty(3, &mut nodes);

    tree.put(Item2 { key: 1, value: 5 }, &mut nodes);
    let merkle_root = nodes.nodes_map.get(&0).unwrap().hash.clone();

    let mut nodes_clone = nodes.recalculate_merkleroot();
    nodes_clone.iterator();
    assert_eq!(merkle_root, nodes_clone.merkleroot());
}

#[test]
fn test_merkle_put_3() {
    let mut nodes_map: HashMap<i32, Node<Item2>> = HashMap::new();
    let mut nodes = Nodes {
        nodes_map,
        size: 0,
        content_size: 0,
        root_id: 0,
        next_id: 0,
        m: 0,
    };
    let mut tree = MerkleBTree::new_empty(3, &mut nodes);

    tree.put(Item2 { key: 1, value: 5 }, &mut nodes);
    tree.put(Item2 { key: 2, value: 5 }, &mut nodes);
    let merkle_root = nodes.nodes_map.get(&0).unwrap().hash.clone();

    let mut nodes_clone = nodes.recalculate_merkleroot();
    nodes_clone.iterator();
    assert_eq!(merkle_root, nodes_clone.merkleroot());
}

#[test]
fn test_merkle_put_4() {
    let mut nodes_map: HashMap<i32, Node<Item2>> = HashMap::new();
    let mut nodes = Nodes {
        nodes_map,
        size: 0,
        content_size: 0,
        root_id: 0,
        next_id: 0,
        m: 0,
    };
    let mut tree = MerkleBTree::new_empty(3, &mut nodes);

    tree.put(Item2 { key: 1, value: 5 }, &mut nodes);
    tree.put(Item2 { key: 2, value: 5 }, &mut nodes);
    tree.put(Item2 { key: 3, value: 5 }, &mut nodes);

    let merkle_root = nodes.nodes_map.get(&0).unwrap().hash.clone();

    let mut nodes_clone = nodes.recalculate_merkleroot();
    nodes_clone.iterator();
    assert_eq!(merkle_root, nodes_clone.merkleroot());
}
