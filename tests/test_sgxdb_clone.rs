use merklebtree::merklebtree::{MerkleBTree, Nodes};
use merklebtree::node::Node;
use std::cmp::Ordering;
use std::collections::HashMap;
use std::fmt::Debug;

mod utils;
use ring::digest;
use utils::*;

#[test]
fn test_sgxdb_insert_clone() {
    let mut nodes_map: HashMap<i32, Node<Item2>> = HashMap::new();
    let mut nodes = Nodes {
        nodes_map,
        size: 0,
        root_id: 0,
        content_size: 0,
        next_id: 0,
        m: 0,
    };
    let mut tree = MerkleBTree::new_empty(3, &mut nodes);

    tree.put(Item2 { key: 1, value: 0 }, &mut nodes);

    tree.put(Item2 { key: 2, value: 1 }, &mut nodes);

    tree.put(Item2 { key: 3, value: 2 }, &mut nodes);

    tree.put(Item2 { key: 4, value: 2 }, &mut nodes);

    tree.put(Item2 { key: 5, value: 2 }, &mut nodes);

    tree.put(Item2 { key: 6, value: 2 }, &mut nodes);

    tree.put(Item2 { key: 7, value: 2 }, &mut nodes);
    assert_valid_tree(&nodes, 7);
    assert_valid_tree_node(&vec![0], 1, 2, &vec![4], false, &nodes);
    assert_valid_tree_node(&vec![0, 0], 1, 2, &vec![2], true, &nodes);
    assert_valid_tree_node(&vec![0, 1], 1, 2, &vec![6], true, &nodes);
    assert_valid_tree_node(&vec![0, 0, 0], 1, 0, &vec![1], true, &nodes);
    assert_valid_tree_node(&vec![0, 0, 1], 1, 0, &vec![3], true, &nodes);
    assert_valid_tree_node(&vec![0, 1, 0], 1, 0, &vec![5], true, &nodes);
    assert_valid_tree_node(&vec![0, 1, 1], 1, 0, &vec![7], true, &nodes);
    assert_eq!(
        nodes.merkleroot(),
        nodes.recalculate_merkleroot().merkleroot()
    );
}