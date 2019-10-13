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
        merkleroot_hash: "".to_string(),
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
        merkleroot_hash: "".to_string(),
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
            merkleroot_hash: "".to_string(),
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
            merkleroot_hash: "".to_string(),
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
        merkleroot_hash: "".to_string(),
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
        merkleroot_hash: "".to_string(),
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
        merkleroot_hash: "".to_string(),
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
    assertValidTreeNodeItem3(&vec![0], 1, 2, &vec![4], false, &nodes);
    assertValidTreeNodeItem3(&vec![0, 0], 1, 2, &vec![2], true, &nodes);
    assertValidTreeNodeItem3(&vec![0, 1], 1, 2, &vec![6], true, &nodes);
    assertValidTreeNodeItem3(&vec![0, 0, 0], 1, 0, &vec![1], true, &nodes);
    assertValidTreeNodeItem3(&vec![0, 0, 1], 1, 0, &vec![3], true, &nodes);
    assertValidTreeNodeItem3(&vec![0, 1, 0], 1, 0, &vec![5], true, &nodes);
    assertValidTreeNodeItem3(&vec![0, 1, 1], 1, 0, &vec![7], true, &nodes);

    tree.remove(Item3 { key: 7 }, &mut nodes);
    assertValidTree(&nodes, 6);
    assertValidTreeNodeItem3(&vec![0], 2, 3, &vec![2, 4], false, &nodes);
    assertValidTreeNodeItem3(&vec![0, 0], 1, 0, &vec![1], true, &nodes);
    assertValidTreeNodeItem3(&vec![0, 1], 1, 0, &vec![3], true, &nodes);
    assertValidTreeNodeItem3(&vec![0, 2], 2, 0, &vec![5, 6], true, &nodes);
}

#[test]
fn test_btree_remove_7() {
    let mut nodes_map: HashMap<i32, Node<Item3>> = HashMap::new();
    let mut nodes = Nodes {
        nodes_map,
        size: 0,
        content_size: 0,
        root_id: 0,
        next_id: 0,
        m: 0,
        merkleroot_hash: "".to_string(),
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
    assertValidTreeNodeItem3(&vec![0], 1, 2, &vec![4], false, &nodes);
    assertValidTreeNodeItem3(&vec![0, 0], 1, 2, &vec![2], true, &nodes);
    assertValidTreeNodeItem3(&vec![0, 1], 1, 2, &vec![6], true, &nodes);
    assertValidTreeNodeItem3(&vec![0, 0, 0], 1, 0, &vec![1], true, &nodes);
    assertValidTreeNodeItem3(&vec![0, 0, 1], 1, 0, &vec![3], true, &nodes);
    assertValidTreeNodeItem3(&vec![0, 1, 0], 1, 0, &vec![5], true, &nodes);
    assertValidTreeNodeItem3(&vec![0, 1, 1], 1, 0, &vec![7], true, &nodes);

    tree.remove(Item3 { key: 1 }, &mut nodes); // series of underflows
    assertValidTree(&nodes, 6);
    assertValidTreeNodeItem3(&vec![0], 2, 3, &vec![4, 6], false, &nodes);
    assertValidTreeNodeItem3(&vec![0, 0], 2, 0, &vec![2, 3], true, &nodes);
    assertValidTreeNodeItem3(&vec![0, 1], 1, 0, &vec![5], true, &nodes);
    assertValidTreeNodeItem3(&vec![0, 2], 1, 0, &vec![7], true, &nodes);

    // clear all remaining
    tree.remove(Item3 { key: 2 }, &mut nodes);
    assertValidTree(&nodes, 5);
    assertValidTreeNodeItem3(&vec![0], 2, 3, &vec![4, 6], false, &nodes);
    assertValidTreeNodeItem3(&vec![0, 0], 1, 0, &vec![3], true, &nodes);
    assertValidTreeNodeItem3(&vec![0, 1], 1, 0, &vec![5], true, &nodes);
    assertValidTreeNodeItem3(&vec![0, 2], 1, 0, &vec![7], true, &nodes);

    tree.remove(Item3 { key: 3 }, &mut nodes);
    assertValidTree(&nodes, 4);
    assertValidTreeNodeItem3(&vec![0], 1, 2, &vec![6], false, &nodes);
    assertValidTreeNodeItem3(&vec![0, 0], 2, 0, &vec![4, 5], true, &nodes);
    assertValidTreeNodeItem3(&vec![0, 1], 1, 0, &vec![7], true, &nodes);

    tree.remove(Item3 { key: 4 }, &mut nodes);
    assertValidTree(&nodes, 3);
    assertValidTreeNodeItem3(&vec![0], 1, 2, &vec![6], false, &nodes);
    assertValidTreeNodeItem3(&vec![0, 0], 1, 0, &vec![5], true, &nodes);
    assertValidTreeNodeItem3(&vec![0, 1], 1, 0, &vec![7], true, &nodes);

    tree.remove(Item3 { key: 5 }, &mut nodes);
    assertValidTree(&nodes, 2);
    assertValidTreeNodeItem3(&vec![0], 2, 0, &vec![6, 7], false, &nodes);

    tree.remove(Item3 { key: 6 }, &mut nodes);
    assertValidTree(&nodes, 1);
    assertValidTreeNodeItem3(&vec![0], 1, 0, &vec![7], false, &nodes);

    tree.remove(Item3 { key: 7 }, &mut nodes);
    assertValidTree(&nodes, 0);
}

#[test]
fn test_btree_remove_8() {
    let mut nodes_map: HashMap<i32, Node<Item3>> = HashMap::new();
    let mut nodes = Nodes {
        nodes_map,
        size: 0,
        content_size: 0,
        root_id: 0,
        next_id: 0,
        m: 0,
        merkleroot_hash: "".to_string(),
    };
    let mut tree = MerkleBTree::new_empty(3, &mut nodes);
    tree.put(Item3 { key: 1 }, &mut nodes);
    tree.put(Item3 { key: 2 }, &mut nodes);
    tree.put(Item3 { key: 3 }, &mut nodes);
    tree.put(Item3 { key: 4 }, &mut nodes);
    tree.put(Item3 { key: 5 }, &mut nodes);
    tree.put(Item3 { key: 6 }, &mut nodes);
    tree.put(Item3 { key: 7 }, &mut nodes);
    tree.put(Item3 { key: 8 }, &mut nodes);
    tree.put(Item3 { key: 9 }, &mut nodes);

    assertValidTree(&nodes, 9);
    assertValidTreeNodeItem3(&vec![0], 1, 2, &vec![4], false, &nodes);
    assertValidTreeNodeItem3(&vec![0, 0], 1, 2, &vec![2], true, &nodes);
    assertValidTreeNodeItem3(&vec![0, 1], 2, 3, &vec![6, 8], true, &nodes);
    assertValidTreeNodeItem3(&vec![0, 0, 0], 1, 0, &vec![1], true, &nodes);
    assertValidTreeNodeItem3(&vec![0, 0, 1], 1, 0, &vec![3], true, &nodes);
    assertValidTreeNodeItem3(&vec![0, 1, 0], 1, 0, &vec![5], true, &nodes);
    assertValidTreeNodeItem3(&vec![0, 1, 1], 1, 0, &vec![7], true, &nodes);
    assertValidTreeNodeItem3(&vec![0, 1, 2], 1, 0, &vec![9], true, &nodes);

    tree.remove(Item3 { key: 1 }, &mut nodes);
    assertValidTree(&nodes, 8);
    assertValidTreeNodeItem3(&vec![0], 1, 2, &vec![6], false, &nodes);
    assertValidTreeNodeItem3(&vec![0, 0], 1, 2, &vec![4], true, &nodes);
    assertValidTreeNodeItem3(&vec![0, 1], 1, 2, &vec![8], true, &nodes);
    assertValidTreeNodeItem3(&vec![0, 0, 0], 2, 0, &vec![2, 3], true, &nodes);
    assertValidTreeNodeItem3(&vec![0, 0, 1], 1, 0, &vec![5], true, &nodes);
    assertValidTreeNodeItem3(&vec![0, 1, 0], 1, 0, &vec![7], true, &nodes);
    assertValidTreeNodeItem3(&vec![0, 1, 1], 1, 0, &vec![9], true, &nodes);
}

#[test]
fn test_btree_remove_9() {
    let max = 1000;
    let orders = vec![3, 4, 5, 6, 7, 8, 9, 10, 20, 100, 500, 1000, 5000, 10000];

    for order in orders.iter() {
        {
            let mut nodes_map: HashMap<i32, Node<Item2>> = HashMap::new();
            let mut nodes = Nodes {
                nodes_map,
                size: 0,
                content_size: 0,
                root_id: 0,
                next_id: 0,
                m: 0,
                merkleroot_hash: "".to_string(),
            };
            let mut tree = MerkleBTree::new_empty(*order, &mut nodes);
            for i in 1..max + 1 {
                tree.put(Item2 { key: i, value: i }, &mut nodes);
            }
            assertValidTree(&nodes, max);

            for i in 1..max + 1 {
                let (value, found) = tree.get(Item2 { key: i, value: 0 }, &mut nodes);
                if !found {
                    panic!("Not found {:?}", value);
                }
            }
            for i in 1..max + 1 {
                tree.remove(Item2 { key: i, value: i }, &mut nodes);
            }
            assertValidTree(&nodes, 0);
        }
        {
            let mut nodes_map: HashMap<i32, Node<Item2>> = HashMap::new();
            let mut nodes = Nodes {
                nodes_map,
                size: 0,
                content_size: 0,
                root_id: 0,
                next_id: 0,
                m: 0,
                merkleroot_hash: "".to_string(),
            };
            let mut tree = MerkleBTree::new_empty(*order, &mut nodes);

            for i in 0..max {
                tree.put(
                    Item2 {
                        key: max - i,
                        value: max - i,
                    },
                    &mut nodes,
                );
            }
            assertValidTree(&nodes, max);

            for i in 0..max {
                let (value, found) = tree.get(
                    Item2 {
                        key: max - i,
                        value: 0,
                    },
                    &mut nodes,
                );
                if !found {
                    panic!("Not found {:?}", value);
                }
            }

            for i in 0..max {
                tree.remove(
                    Item2 {
                        key: max - i,
                        value: 0,
                    },
                    &mut nodes,
                );
            }
            assertValidTree(&nodes, 0);
        }
    }
}
