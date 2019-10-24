use merklebtree::merklebtree::{MerkleBTree, Nodes};
use merklebtree::node::Node;
use merklebtree::traits::CalculateHash;
use serde::{Deserialize, Serialize};
use std::cmp::Ordering;
use std::collections::HashMap;
use std::fmt::Debug;

mod utils;
use ring::digest;
use utils::*;

#[test]
fn test_clone_search_subnode_from_root() {
    let mut nodes_map: HashMap<i32, Node<Item3>> = HashMap::new();
    let mut nodes = Nodes {
        nodes_map,
        size: 0,
        root_id: 0,
        content_size: 0,
        next_id: 0,
        m: 0,
    };
    let mut tree = MerkleBTree::new_empty(3, &mut nodes);

    tree.put(Item3 { key: 6 }, &mut nodes);
    tree.put(Item3 { key: 5 }, &mut nodes);
    tree.put(Item3 { key: 4 }, &mut nodes);
    tree.put(Item3 { key: 3 }, &mut nodes);
    tree.put(Item3 { key: 2 }, &mut nodes);
    tree.put(Item3 { key: 1 }, &mut nodes);
    tree.put(Item3 { key: 0 }, &mut nodes);
    tree.put(Item3 { key: -1 }, &mut nodes);
    tree.put(Item3 { key: -2 }, &mut nodes);
    tree.put(Item3 { key: -3 }, &mut nodes);
    tree.put(Item3 { key: -4 }, &mut nodes);

    assert_valid_tree(&nodes, 11);
    assert_valid_tree_node_item3(&vec![0], 2, 3, &vec![-1, 3], false, &nodes);
    assert_valid_tree_node_item3(&vec![0, 0], 1, 2, &vec![-3], true, &nodes);
    assert_valid_tree_node_item3(&vec![0, 1], 1, 2, &vec![1], true, &nodes);
    assert_valid_tree_node_item3(&vec![0, 2], 1, 2, &vec![5], true, &nodes);
    assert_valid_tree_node_item3(&vec![0, 0, 0], 1, 0, &vec![-4], true, &nodes);
    assert_valid_tree_node_item3(&vec![0, 0, 1], 1, 0, &vec![-2], true, &nodes);
    assert_valid_tree_node_item3(&vec![0, 1, 0], 1, 0, &vec![0], true, &nodes);
    assert_valid_tree_node_item3(&vec![0, 1, 1], 1, 0, &vec![2], true, &nodes);
    assert_valid_tree_node_item3(&vec![0, 2, 0], 1, 0, &vec![4], true, &nodes);
    assert_valid_tree_node_item3(&vec![0, 2, 1], 1, 0, &vec![6], true, &nodes);

    let mut subnodes = tree.clone_search_subnode_from_root(0, &Item3 { key: -4 }, &mut nodes);

    let mut subtree = MerkleBTree {
        rootid: subnodes.root_id,
        m: subnodes.m,
    };

    let (value, found) = subtree.get(Item3 { key: -4 }, &mut subnodes);
    assert_eq!(true, found);
}

#[test]
fn test_sgxdb_put_clone_from_root() {
    let mut nodes_map: HashMap<i32, Node<Item4>> = HashMap::new();
    let mut nodes = Nodes {
        nodes_map,
        size: 0,
        root_id: 0,
        content_size: 0,
        next_id: 0,
        m: 0,
    };
    let mut tree = MerkleBTree::new_empty(3, &mut nodes);

    tree.put(Item4 { key: 1, value: 0 }, &mut nodes);
    tree.put(Item4 { key: 2, value: 1 }, &mut nodes);
    tree.put(Item4 { key: 3, value: 2 }, &mut nodes);
    tree.put(Item4 { key: 4, value: 2 }, &mut nodes);
    tree.put(Item4 { key: 5, value: 2 }, &mut nodes);
    tree.put(Item4 { key: 6, value: 2 }, &mut nodes);
    tree.put(Item4 { key: 7, value: 2 }, &mut nodes);
    assert_valid_tree(&nodes, 7);
    assert_valid_tree_node_item4(&vec![0], 1, 2, &vec![4], false, &nodes);
    assert_valid_tree_node_item4(&vec![0, 0], 1, 2, &vec![2], true, &nodes);
    assert_valid_tree_node_item4(&vec![0, 1], 1, 2, &vec![6], true, &nodes);
    assert_valid_tree_node_item4(&vec![0, 0, 0], 1, 0, &vec![1], true, &nodes);
    assert_valid_tree_node_item4(&vec![0, 0, 1], 1, 0, &vec![3], true, &nodes);
    assert_valid_tree_node_item4(&vec![0, 1, 0], 1, 0, &vec![5], true, &nodes);
    assert_valid_tree_node_item4(&vec![0, 1, 1], 1, 0, &vec![7], true, &nodes);

    println!("-------------------put before------------------------");
    nodes.iterator();
    println!("nodes.merkleroot: {:?}", nodes.merkleroot());
    println!("{:?}", nodes.nodes_map);
    let mut subnodes = tree.put_clone(Item4 { key: 7, value: 1 }, &mut nodes);

    println!("-------------------put after------------------------");
    println!("nodes.merkleroot: {:?}", nodes.merkleroot());
    println!("subnodes.merkleroot:{:?}", subnodes.merkleroot());
    nodes.iterator();

    println!("-------------------subnodes put------------------------");
    tree.put(Item4 { key: 7, value: 1 }, &mut subnodes);
    println!("subnodes.merkleroot:{:?}", subnodes.merkleroot());
    println!("subnodes.nodemap:{:?}", subnodes.nodes_map);
}

#[test]
fn test_sgxdb_insert_clone_from_root() {
    let mut nodes_map: HashMap<i32, Node<Item4>> = HashMap::new();
    let mut nodes = Nodes {
        nodes_map,
        size: 0,
        root_id: 0,
        content_size: 0,
        next_id: 0,
        m: 0,
    };
    let mut tree = MerkleBTree::new_empty(3, &mut nodes);

    tree.put(Item4 { key: 1, value: 0 }, &mut nodes);
    tree.put(Item4 { key: 2, value: 1 }, &mut nodes);
    tree.put(Item4 { key: 3, value: 2 }, &mut nodes);
    tree.put(Item4 { key: 4, value: 2 }, &mut nodes);
    tree.put(Item4 { key: 5, value: 2 }, &mut nodes);
    tree.put(Item4 { key: 6, value: 2 }, &mut nodes);
    tree.put(Item4 { key: 7, value: 2 }, &mut nodes);
    assert_valid_tree(&nodes, 7);
    assert_valid_tree_node_item4(&vec![0], 1, 2, &vec![4], false, &nodes);
    assert_valid_tree_node_item4(&vec![0, 0], 1, 2, &vec![2], true, &nodes);
    assert_valid_tree_node_item4(&vec![0, 1], 1, 2, &vec![6], true, &nodes);
    assert_valid_tree_node_item4(&vec![0, 0, 0], 1, 0, &vec![1], true, &nodes);
    assert_valid_tree_node_item4(&vec![0, 0, 1], 1, 0, &vec![3], true, &nodes);
    assert_valid_tree_node_item4(&vec![0, 1, 0], 1, 0, &vec![5], true, &nodes);
    assert_valid_tree_node_item4(&vec![0, 1, 1], 1, 0, &vec![7], true, &nodes);

    println!("-------------------put before------------------------");
    nodes.iterator();
    println!("nodes.merkleroot: {:?}", nodes.merkleroot());
    println!("{:?}", nodes.nodes_map);

    let mut subnodes = tree.put_clone(Item4 { key: 8, value: 3 }, &mut nodes);

    println!("-------------------put after------------------------");
    println!("nodes.merkleroot: {:?}", nodes.merkleroot());
    println!("subnodes.merkleroot: {:?}", subnodes.merkleroot());

    nodes.iterator();

    println!("-------------------subnodes put------------------------");
    tree.put(Item4 { key: 8, value: 3 }, &mut subnodes);
    println!("subnodes.merkleroot: {:?}", subnodes.merkleroot());
    println!("subnodes.nodemap: {:?}", subnodes.nodes_map);
}

#[test]
pub fn test_sgxdb_remove_clone_from_root() {
    let mut nodes_map: HashMap<i32, Node<Item4>> = HashMap::new();
    let mut nodes = Nodes {
        nodes_map,
        size: 0,
        root_id: 0,
        content_size: 0,
        next_id: 0,
        m: 0,
    };
    let mut tree = MerkleBTree::new_empty(3, &mut nodes);

    tree.put(Item4 { key: 1, value: 0 }, &mut nodes);
    tree.put(Item4 { key: 2, value: 1 }, &mut nodes);
    tree.put(Item4 { key: 3, value: 2 }, &mut nodes);
    tree.put(Item4 { key: 4, value: 2 }, &mut nodes);
    tree.put(Item4 { key: 5, value: 2 }, &mut nodes);
    tree.put(Item4 { key: 6, value: 2 }, &mut nodes);
    tree.put(Item4 { key: 7, value: 2 }, &mut nodes);
    assert_valid_tree(&nodes, 7);
    assert_valid_tree_node_item4(&vec![0], 1, 2, &vec![4], false, &nodes);
    assert_valid_tree_node_item4(&vec![0, 0], 1, 2, &vec![2], true, &nodes);
    assert_valid_tree_node_item4(&vec![0, 1], 1, 2, &vec![6], true, &nodes);
    assert_valid_tree_node_item4(&vec![0, 0, 0], 1, 0, &vec![1], true, &nodes);
    assert_valid_tree_node_item4(&vec![0, 0, 1], 1, 0, &vec![3], true, &nodes);
    assert_valid_tree_node_item4(&vec![0, 1, 0], 1, 0, &vec![5], true, &nodes);
    assert_valid_tree_node_item4(&vec![0, 1, 1], 1, 0, &vec![7], true, &nodes);

    println!("-------------------remove before------------------------");
    nodes.iterator();
    println!("nodes.merkleroot: {:?}", nodes.merkleroot());
    println!("{:?}", nodes.nodes_map);

    let mut subnodes = tree.remove_clone(Item4 { key: 7, value: 0 }, &mut nodes);

    println!("-------------------remove after------------------------");
    println!("nodes.merkleroot: {:?}", nodes.merkleroot());
    println!("subnodes.merkleroot: {:?}", subnodes.merkleroot());
    nodes.iterator();

    println!("-------------------subnodes remove------------------------");
    tree.remove(Item4 { key: 7, value: 0 }, &mut subnodes);
    println!("subnodes.merkleroot: {:?}", subnodes.merkleroot());
    println!("subnodes.nodemap: {:?}", subnodes.nodes_map);
}
