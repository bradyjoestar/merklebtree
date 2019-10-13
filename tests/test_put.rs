use merklebtree::merklebtree::{MerkleBTree, Nodes};
use merklebtree::node::Node;
use std::cmp::Ordering;
use std::collections::HashMap;
use std::fmt::Debug;

mod utils;
use utils::*;

#[test]
fn test_btree_get_1() {
    let mut nodes_map: HashMap<i32, Node<Item>> = HashMap::new();
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
    let mut testdata: Vec<Vec<(Item, bool)>> = Vec::new();

    let mut test_subdata1 = Vec::new();
    test_subdata1.push((
        Item {
            key: 0,
            value: String::from("m"),
        },
        false,
    ));
    testdata.push(test_subdata1);

    for i in 1..8 {
        let string = String::from_utf8(vec![(i + 96) as u8]).unwrap();
        let mut test_subdata = Vec::new();
        test_subdata.push((
            Item {
                key: i,
                value: string.clone(),
            },
            true,
        ));
        testdata.push(test_subdata);
        tree.put(
            Item {
                key: i,
                value: string.clone(), //'a'
            },
            &mut nodes,
        );
    }

    let mut test_subdata2 = Vec::new();
    test_subdata2.push((
        Item {
            key: 8,
            value: String::from("n"),
        },
        false,
    ));
    testdata.push(test_subdata2);

    for test_vec in testdata.iter() {
        for test_item in test_vec.iter() {
            println!("{:?}", test_item);
            let (value, found) = tree.get(test_item.0.clone(), &mut nodes);
            assert_eq!(value, test_item.0);
            assert_eq!(found, test_item.1);
        }
    }
}

#[test]
fn test_btree_get_2() {
    let mut nodes_map: HashMap<i32, Node<Item>> = HashMap::new();
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
    let mut testdata: Vec<Vec<(Item, bool)>> = Vec::new();

    tree.put(
        Item {
            key: 7,
            value: String::from("g"),
        },
        &mut nodes,
    );
    tree.put(
        Item {
            key: 9,
            value: String::from("i"),
        },
        &mut nodes,
    );
    tree.put(
        Item {
            key: 10,
            value: String::from("j"),
        },
        &mut nodes,
    );
    tree.put(
        Item {
            key: 6,
            value: String::from("f"),
        },
        &mut nodes,
    );
    tree.put(
        Item {
            key: 3,
            value: String::from("c"),
        },
        &mut nodes,
    );
    tree.put(
        Item {
            key: 4,
            value: String::from("d"),
        },
        &mut nodes,
    );
    tree.put(
        Item {
            key: 5,
            value: String::from("e"),
        },
        &mut nodes,
    );
    tree.put(
        Item {
            key: 8,
            value: String::from("h"),
        },
        &mut nodes,
    );
    tree.put(
        Item {
            key: 2,
            value: String::from("b"),
        },
        &mut nodes,
    );
    tree.put(
        Item {
            key: 1,
            value: String::from("a"),
        },
        &mut nodes,
    );

    let mut test_subdata1 = Vec::new();
    test_subdata1.push((
        Item {
            key: 0,
            value: String::from("m"),
        },
        false,
    ));
    testdata.push(test_subdata1);

    for i in 1..11 {
        let string = String::from_utf8(vec![(i + 96) as u8]).unwrap();
        let mut test_subdata = Vec::new();
        test_subdata.push((
            Item {
                key: i,
                value: string.clone(),
            },
            true,
        ));
        testdata.push(test_subdata);
    }

    let mut test_subdata2 = Vec::new();
    test_subdata2.push((
        Item {
            key: 11,
            value: String::from("n"),
        },
        false,
    ));
    testdata.push(test_subdata2);

    for test_vec in testdata.iter() {
        for test_item in test_vec.iter() {
            println!("{:?}", test_item);
            let (value, found) = tree.get(test_item.0.clone(), &mut nodes);
            assert_eq!(value, test_item.0);
            assert_eq!(found, test_item.1);
        }
    }
}

#[test]
fn test_btree_put_1() {
    let mut nodes_map: HashMap<i32, Node<Item2>> = HashMap::new();
    let mut nodes = Nodes {
        nodes_map,
        size: 0,
        root_id: 0,
        content_size: 0,
        next_id: 0,
        m: 0,
        merkleroot_hash: "".to_string(),
    };
    let mut tree = MerkleBTree::new_empty(3, &mut nodes);
    assertValidTree(&nodes, 0);

    tree.put(Item2 { key: 1, value: 0 }, &mut nodes);
    assertValidTree(&nodes, 1);
    assertValidTreeNode(&vec![0], 1, 0, &vec![1], false, &nodes);

    tree.put(Item2 { key: 2, value: 1 }, &mut nodes);
    assertValidTree(&nodes, 2);
    assertValidTreeNode(&vec![0], 2, 0, &vec![1, 2], false, &nodes);

    tree.put(Item2 { key: 3, value: 2 }, &mut nodes);
    assertValidTree(&nodes, 3);
    assertValidTreeNode(&vec![0], 1, 2, &vec![2], false, &nodes);
    assertValidTreeNode(&vec![0, 0], 1, 0, &vec![1], true, &nodes);
    assertValidTreeNode(&vec![0, 1], 1, 0, &vec![3], true, &nodes);

    tree.put(Item2 { key: 4, value: 2 }, &mut nodes);
    assertValidTree(&nodes, 4);
    assertValidTreeNode(&vec![0], 1, 2, &vec![2], false, &nodes);
    assertValidTreeNode(&vec![0, 0], 1, 0, &vec![1], true, &nodes);
    assertValidTreeNode(&vec![0, 1], 2, 0, &vec![3, 4], true, &nodes);

    tree.put(Item2 { key: 5, value: 2 }, &mut nodes);
    assertValidTree(&nodes, 5);
    assertValidTreeNode(&vec![0], 2, 3, &vec![2, 4], false, &nodes);
    assertValidTreeNode(&vec![0, 0], 1, 0, &vec![1], true, &nodes);
    assertValidTreeNode(&vec![0, 1], 1, 0, &vec![3], true, &nodes);
    assertValidTreeNode(&vec![0, 2], 1, 0, &vec![5], true, &nodes);

    tree.put(Item2 { key: 6, value: 2 }, &mut nodes);
    assertValidTree(&nodes, 6);
    assertValidTreeNode(&vec![0], 2, 3, &vec![2, 4], false, &nodes);
    assertValidTreeNode(&vec![0, 0], 1, 0, &vec![1], true, &nodes);
    assertValidTreeNode(&vec![0, 1], 1, 0, &vec![3], true, &nodes);
    assertValidTreeNode(&vec![0, 2], 2, 0, &vec![5, 6], true, &nodes);

    tree.put(Item2 { key: 7, value: 2 }, &mut nodes);
    assertValidTree(&nodes, 7);
    assertValidTreeNode(&vec![0], 1, 2, &vec![4], false, &nodes);
    assertValidTreeNode(&vec![0, 0], 1, 2, &vec![2], true, &nodes);
    assertValidTreeNode(&vec![0, 1], 1, 2, &vec![6], true, &nodes);
    assertValidTreeNode(&vec![0, 0, 0], 1, 0, &vec![1], true, &nodes);
    assertValidTreeNode(&vec![0, 0, 1], 1, 0, &vec![3], true, &nodes);
    assertValidTreeNode(&vec![0, 1, 0], 1, 0, &vec![5], true, &nodes);
    assertValidTreeNode(&vec![0, 1, 1], 1, 0, &vec![7], true, &nodes);
}

#[test]
fn test_btree_put_2() {
    let mut nodes_map: HashMap<i32, Node<Item2>> = HashMap::new();
    let mut nodes = Nodes {
        nodes_map,
        size: 0,
        root_id: 0,
        content_size: 0,
        next_id: 0,
        m: 0,
        merkleroot_hash: "".to_string(),
    };
    let mut tree = MerkleBTree::new_empty(4, &mut nodes);
    assertValidTree(&nodes, 0);

    tree.put(Item2 { key: 0, value: 0 }, &mut nodes);
    assertValidTree(&nodes, 1);
    assertValidTreeNode(&vec![0], 1, 0, &vec![0], false, &nodes);

    tree.put(Item2 { key: 2, value: 2 }, &mut nodes);
    assertValidTree(&nodes, 2);
    assertValidTreeNode(&vec![0], 2, 0, &vec![0, 2], false, &nodes);

    tree.put(Item2 { key: 1, value: 1 }, &mut nodes);
    assertValidTree(&nodes, 3);
    assertValidTreeNode(&vec![0], 3, 0, &vec![0, 1, 2], false, &nodes);

    tree.put(Item2 { key: 1, value: 1 }, &mut nodes);
    assertValidTree(&nodes, 3);
    assertValidTreeNode(&vec![0], 3, 0, &vec![0, 1, 2], false, &nodes);

    tree.put(Item2 { key: 3, value: 3 }, &mut nodes);
    assertValidTree(&nodes, 4);
    assertValidTreeNode(&vec![0], 1, 2, &vec![1], false, &nodes);
    assertValidTreeNode(&vec![0, 0], 1, 0, &vec![0], true, &nodes);
    assertValidTreeNode(&vec![0, 1], 2, 0, &vec![2, 3], true, &nodes);

    tree.put(Item2 { key: 4, value: 4 }, &mut nodes);
    assertValidTree(&nodes, 5);
    assertValidTreeNode(&vec![0], 1, 2, &vec![1], false, &nodes);
    assertValidTreeNode(&vec![0, 0], 1, 0, &vec![0], true, &nodes);
    assertValidTreeNode(&vec![0, 1], 3, 0, &vec![2, 3, 4], true, &nodes);

    tree.put(Item2 { key: 5, value: 5 }, &mut nodes);
    assertValidTree(&nodes, 6);
    assertValidTreeNode(&vec![0], 2, 3, &vec![1, 3], false, &nodes);
    assertValidTreeNode(&vec![0, 0], 1, 0, &vec![0], true, &nodes);
    assertValidTreeNode(&vec![0, 1], 1, 0, &vec![2], true, &nodes);
    assertValidTreeNode(&vec![0, 2], 2, 0, &vec![4, 5], true, &nodes);
}

#[test]
fn test_btree_put_3() {
    let mut nodes_map: HashMap<i32, Node<Item2>> = HashMap::new();
    let mut nodes = Nodes {
        nodes_map,
        size: 0,
        root_id: 0,
        content_size: 0,
        next_id: 0,
        m: 0,
        merkleroot_hash: "".to_string(),
    };
    let mut tree = MerkleBTree::new_empty(6, &mut nodes);
    assertValidTree(&nodes, 0);

    tree.put(Item2 { key: 10, value: 0 }, &mut nodes);
    assertValidTree(&nodes, 1);
    assertValidTreeNode(&vec![0], 1, 0, &vec![10], false, &nodes);

    tree.put(Item2 { key: 20, value: 1 }, &mut nodes);
    assertValidTree(&nodes, 2);
    assertValidTreeNode(&vec![0], 2, 0, &vec![10, 20], false, &nodes);

    tree.put(Item2 { key: 30, value: 2 }, &mut nodes);
    assertValidTree(&nodes, 3);
    assertValidTreeNode(&vec![0], 3, 0, &vec![10, 20, 30], false, &nodes);

    tree.put(Item2 { key: 40, value: 3 }, &mut nodes);
    assertValidTree(&nodes, 4);
    assertValidTreeNode(&vec![0], 4, 0, &vec![10, 20, 30, 40], false, &nodes);

    tree.put(Item2 { key: 50, value: 4 }, &mut nodes);
    assertValidTree(&nodes, 5);
    assertValidTreeNode(&vec![0], 5, 0, &vec![10, 20, 30, 40, 50], false, &nodes);

    tree.put(Item2 { key: 60, value: 5 }, &mut nodes);
    assertValidTree(&nodes, 6);
    assertValidTreeNode(&vec![0], 1, 2, &vec![30], false, &nodes);
    assertValidTreeNode(&vec![0, 0], 2, 0, &vec![10, 20], true, &nodes);
    assertValidTreeNode(&vec![0, 1], 3, 0, &vec![40, 50, 60], true, &nodes);

    tree.put(Item2 { key: 70, value: 6 }, &mut nodes);
    assertValidTree(&nodes, 7);
    assertValidTreeNode(&vec![0], 1, 2, &vec![30], false, &nodes);
    assertValidTreeNode(&vec![0, 0], 2, 0, &vec![10, 20], true, &nodes);
    assertValidTreeNode(&vec![0, 1], 4, 0, &vec![40, 50, 60, 70], true, &nodes);

    tree.put(Item2 { key: 80, value: 7 }, &mut nodes);
    assertValidTree(&nodes, 8);
    assertValidTreeNode(&vec![0], 1, 2, &vec![30], false, &nodes);
    assertValidTreeNode(&vec![0, 0], 2, 0, &vec![10, 20], true, &nodes);
    assertValidTreeNode(&vec![0, 1], 5, 0, &vec![40, 50, 60, 70, 80], true, &nodes);

    tree.put(Item2 { key: 90, value: 8 }, &mut nodes);
    assertValidTree(&nodes, 9);
    assertValidTreeNode(&vec![0], 2, 3, &vec![30, 60], false, &nodes);
    assertValidTreeNode(&vec![0, 0], 2, 0, &vec![10, 20], true, &nodes);
    assertValidTreeNode(&vec![0, 1], 2, 0, &vec![40, 50], true, &nodes);
    assertValidTreeNode(&vec![0, 2], 3, 0, &vec![70, 80, 90], true, &nodes);
}

#[test]
fn test_btree_put_4() {
    let mut nodes_map: HashMap<i32, Node<Item3>> = HashMap::new();
    let mut nodes = Nodes {
        nodes_map,
        size: 0,
        root_id: 0,
        content_size: 0,
        next_id: 0,
        m: 0,
        merkleroot_hash: "".to_string(),
    };
    let mut tree = MerkleBTree::new_empty(3, &mut nodes);
    assertValidTree(&nodes, 0);

    tree.put(Item3 { key: 6 }, &mut nodes);
    assertValidTree(&nodes, 1);
    assertValidTreeNodeItem3(&vec![0], 1, 0, &vec![6], false, &nodes);

    tree.put(Item3 { key: 5 }, &mut nodes);
    assertValidTree(&nodes, 2);
    assertValidTreeNodeItem3(&vec![0], 2, 0, &vec![5, 6], false, &nodes);

    tree.put(Item3 { key: 4 }, &mut nodes);
    assertValidTree(&nodes, 3);
    assertValidTreeNodeItem3(&vec![0], 1, 2, &vec![5], false, &nodes);
    assertValidTreeNodeItem3(&vec![0, 0], 1, 0, &vec![4], true, &nodes);
    assertValidTreeNodeItem3(&vec![0, 1], 1, 0, &vec![6], true, &nodes);

    tree.put(Item3 { key: 3 }, &mut nodes);
    assertValidTree(&nodes, 4);
    assertValidTreeNodeItem3(&vec![0], 1, 2, &vec![5], false, &nodes);
    assertValidTreeNodeItem3(&vec![0, 0], 2, 0, &vec![3, 4], true, &nodes);
    assertValidTreeNodeItem3(&vec![0, 1], 1, 0, &vec![6], true, &nodes);

    tree.put(Item3 { key: 2 }, &mut nodes);
    assertValidTree(&nodes, 5);
    assertValidTreeNodeItem3(&vec![0], 2, 3, &vec![3, 5], false, &nodes);
    assertValidTreeNodeItem3(&vec![0, 0], 1, 0, &vec![2], true, &nodes);
    assertValidTreeNodeItem3(&vec![0, 1], 1, 0, &vec![4], true, &nodes);
    assertValidTreeNodeItem3(&vec![0, 2], 1, 0, &vec![6], true, &nodes);

    tree.put(Item3 { key: 1 }, &mut nodes);
    assertValidTree(&nodes, 6);
    assertValidTreeNodeItem3(&vec![0], 2, 3, &vec![3, 5], false, &nodes);
    assertValidTreeNodeItem3(&vec![0, 0], 2, 0, &vec![1, 2], true, &nodes);
    assertValidTreeNodeItem3(&vec![0, 1], 1, 0, &vec![4], true, &nodes);
    assertValidTreeNodeItem3(&vec![0, 2], 1, 0, &vec![6], true, &nodes);

    tree.put(Item3 { key: 0 }, &mut nodes);
    assertValidTree(&nodes, 7);
    assertValidTreeNodeItem3(&vec![0], 1, 2, &vec![3], false, &nodes);
    assertValidTreeNodeItem3(&vec![0, 0], 1, 2, &vec![1], true, &nodes);
    assertValidTreeNodeItem3(&vec![0, 1], 1, 2, &vec![5], true, &nodes);
    assertValidTreeNodeItem3(&vec![0, 0, 0], 1, 0, &vec![0], true, &nodes);
    assertValidTreeNodeItem3(&vec![0, 0, 1], 1, 0, &vec![2], true, &nodes);
    assertValidTreeNodeItem3(&vec![0, 1, 0], 1, 0, &vec![4], true, &nodes);
    assertValidTreeNodeItem3(&vec![0, 1, 1], 1, 0, &vec![6], true, &nodes);

    tree.put(Item3 { key: -1 }, &mut nodes);
    assertValidTree(&nodes, 8);
    assertValidTreeNodeItem3(&vec![0], 1, 2, &vec![3], false, &nodes);
    assertValidTreeNodeItem3(&vec![0, 0], 1, 2, &vec![1], true, &nodes);
    assertValidTreeNodeItem3(&vec![0, 1], 1, 2, &vec![5], true, &nodes);
    assertValidTreeNodeItem3(&vec![0, 0, 0], 2, 0, &vec![-1, 0], true, &nodes);
    assertValidTreeNodeItem3(&vec![0, 0, 1], 1, 0, &vec![2], true, &nodes);
    assertValidTreeNodeItem3(&vec![0, 1, 0], 1, 0, &vec![4], true, &nodes);
    assertValidTreeNodeItem3(&vec![0, 1, 1], 1, 0, &vec![6], true, &nodes);

    tree.put(Item3 { key: -2 }, &mut nodes);
    assertValidTree(&nodes, 9);
    assertValidTreeNodeItem3(&vec![0], 1, 2, &vec![3], false, &nodes);
    assertValidTreeNodeItem3(&vec![0, 0], 2, 3, &vec![-1, 1], true, &nodes);
    assertValidTreeNodeItem3(&vec![0, 1], 1, 2, &vec![5], true, &nodes);
    assertValidTreeNodeItem3(&vec![0, 0, 0], 1, 0, &vec![-2], true, &nodes);
    assertValidTreeNodeItem3(&vec![0, 0, 1], 1, 0, &vec![0], true, &nodes);
    assertValidTreeNodeItem3(&vec![0, 0, 2], 1, 0, &vec![2], true, &nodes);
    assertValidTreeNodeItem3(&vec![0, 1, 0], 1, 0, &vec![4], true, &nodes);
    assertValidTreeNodeItem3(&vec![0, 1, 1], 1, 0, &vec![6], true, &nodes);

    tree.put(Item3 { key: -3 }, &mut nodes);
    assertValidTree(&nodes, 10);
    assertValidTreeNodeItem3(&vec![0], 1, 2, &vec![3], false, &nodes);
    assertValidTreeNodeItem3(&vec![0, 0], 2, 3, &vec![-1, 1], true, &nodes);
    assertValidTreeNodeItem3(&vec![0, 1], 1, 2, &vec![5], true, &nodes);
    assertValidTreeNodeItem3(&vec![0, 0, 0], 2, 0, &vec![-3, -2], true, &nodes);
    assertValidTreeNodeItem3(&vec![0, 0, 1], 1, 0, &vec![0], true, &nodes);
    assertValidTreeNodeItem3(&vec![0, 0, 2], 1, 0, &vec![2], true, &nodes);
    assertValidTreeNodeItem3(&vec![0, 1, 0], 1, 0, &vec![4], true, &nodes);
    assertValidTreeNodeItem3(&vec![0, 1, 1], 1, 0, &vec![6], true, &nodes);

    tree.put(Item3 { key: -4 }, &mut nodes);
    assertValidTree(&nodes, 11);
    assertValidTreeNodeItem3(&vec![0], 2, 3, &vec![-1, 3], false, &nodes);
    assertValidTreeNodeItem3(&vec![0, 0], 1, 2, &vec![-3], true, &nodes);
    assertValidTreeNodeItem3(&vec![0, 1], 1, 2, &vec![1], true, &nodes);
    assertValidTreeNodeItem3(&vec![0, 2], 1, 2, &vec![5], true, &nodes);
    assertValidTreeNodeItem3(&vec![0, 0, 0], 1, 0, &vec![-4], true, &nodes);
    assertValidTreeNodeItem3(&vec![0, 0, 1], 1, 0, &vec![-2], true, &nodes);
    assertValidTreeNodeItem3(&vec![0, 1, 0], 1, 0, &vec![0], true, &nodes);
    assertValidTreeNodeItem3(&vec![0, 1, 1], 1, 0, &vec![2], true, &nodes);
    assertValidTreeNodeItem3(&vec![0, 2, 0], 1, 0, &vec![4], true, &nodes);
    assertValidTreeNodeItem3(&vec![0, 2, 1], 1, 0, &vec![6], true, &nodes);
}
