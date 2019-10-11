use merklebtree::merklebtree::{MerkleBTree, Nodes};
use merklebtree::node::Node;
use std::cmp::Ordering;
use std::collections::HashMap;
use std::fmt::Debug;

mod utils;
use utils::*;

#[test]
fn test_btree_remove_1() {
    // empty
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
    // leaf node (no underflow)
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

#[test]
fn test_btree_remove_3() {
    // merge with right (underflow)
    {
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
        tree.put(Item3 { key: 3 }, &mut nodes);

        tree.remove(Item3 { key: 1 }, &mut nodes);
        assertValidTree(&nodes, 2);
        assertValidTreeNodeItem3(&vec![0], 2, 0, &vec![2, 3], false, &nodes);
    }
    // merge with left (underflow)
    {
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
        tree.put(Item3 { key: 3 }, &mut nodes);

        tree.remove(Item3 { key: 3 }, &mut nodes);
        assertValidTree(&nodes, 2);
        assertValidTreeNodeItem3(&vec![0], 2, 0, &vec![1, 2], false, &nodes);
    }
}

#[test]
fn test_btree_remove_4() {
    // rotate left (underflow)
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
    tree.put(Item3 { key: 3 }, &mut nodes);
    tree.put(Item3 { key: 4 }, &mut nodes);

    assertValidTree(&nodes, 4);
    assertValidTreeNodeItem3(&vec![0], 1, 2, &vec![2], false, &nodes);
    assertValidTreeNodeItem3(&vec![0, 0], 1, 0, &vec![1], true, &nodes);
    assertValidTreeNodeItem3(&vec![0, 1], 2, 0, &vec![3, 4], true, &nodes);

    tree.remove(Item3 { key: 1 }, &mut nodes);
    assertValidTree(&nodes, 3);
    assertValidTreeNodeItem3(&vec![0], 1, 2, &vec![3], false, &nodes);
    assertValidTreeNodeItem3(&vec![0, 0], 1, 0, &vec![2], true, &nodes);
    assertValidTreeNodeItem3(&vec![0, 1], 1, 0, &vec![4], true, &nodes);
}

#[test]
fn test_btree_remove_5() {
    // rotate right (underflow)
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
    tree.put(Item3 { key: 3 }, &mut nodes);
    tree.put(Item3 { key: 0 }, &mut nodes);

    assertValidTree(&nodes, 4);
    assertValidTreeNodeItem3(&vec![0], 1, 2, &vec![2], false, &nodes);
    assertValidTreeNodeItem3(&vec![0, 0], 2, 0, &vec![0, 1], true, &nodes);
    assertValidTreeNodeItem3(&vec![0, 1], 1, 0, &vec![3], true, &nodes);

    tree.remove(Item3 { key: 3 }, &mut nodes);
    assertValidTree(&nodes, 3);
    assertValidTreeNodeItem3(&vec![0], 1, 2, &vec![1], false, &nodes);
    assertValidTreeNodeItem3(&vec![0, 0], 1, 0, &vec![0], true, &nodes);
    assertValidTreeNodeItem3(&vec![0, 1], 1, 0, &vec![2], true, &nodes);
}

#[test]
fn test_btree_remove_6() {
    // root height reduction after a series of underflows on right side
    // use simulator: https://www.cs.usfca.edu/~galles/visualization/BTree.html
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
    tree.put(Item3 { key: 3 }, &mut nodes);
    tree.put(Item3 { key: 4 }, &mut nodes);
    tree.put(Item3 { key: 5 }, &mut nodes);
    tree.put(Item3 { key: 6 }, &mut nodes);
    tree.put(Item3 { key: 7 }, &mut nodes);
    assertValidTree(&nodes, 7);
}