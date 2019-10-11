use merklebtree::merklebtree::{MerkleBTree, Nodes};
use merklebtree::node::Node;
use std::cmp::Ordering;
use std::collections::HashMap;
use std::fmt::Debug;

mod utils;
use utils::*;

#[test]
fn test_btree_remove_1() {
    let mut nodes_map: HashMap<i32, Node<Item3>> = HashMap::new();
    let mut nodes = Nodes {
        nodes_map,
        size: 0,
        content_size: 0,
        root_id: 0,
        next_id: 0,
        m: 0,
    };
    let mut tree = MerkleBTree::new_empty(3, &mut nodes);
    tree.remove(Item3 { key: 1 }, &mut nodes);
    assertValidTree(&nodes, 0);
}
#[test]
fn test_btree_remove_2() {
    let mut nodes_map: HashMap<i32, Node<Item3>> = HashMap::new();
    let mut nodes = Nodes {
        nodes_map,
        size: 0,
        content_size: 0,
        root_id: 0,
        next_id: 0,
        m: 0,
    };
    let mut tree = MerkleBTree::new_empty(3, &mut nodes);
    tree.put(Item3 { key: 1 }, &mut nodes);
    tree.put(Item3 { key: 2 }, &mut nodes);

    tree.remove(Item3 { key: 1 }, &mut nodes);
    assertValidTree(&nodes, 1);
    assertValidTreeNodeItem3(&vec![0], 1, 0, &vec![2], false, &nodes);

    tree.remove(Item3 { key: 2 }, &mut nodes);
    assertValidTree(&nodes, 0);
}
