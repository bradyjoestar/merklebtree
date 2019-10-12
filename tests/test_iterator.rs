use merklebtree::merklebtree::{MerkleBTree, Nodes};
use merklebtree::node::Node;
use std::cmp::Ordering;
use std::collections::HashMap;
use std::fmt::Debug;

mod utils;
use utils::*;

#[test]
fn test_btree_height() {
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

    let mut actual_value = 0;
    let mut expect_value = 0;

    tree.put(Item2 { key: 1, value: 0 }, &mut nodes);
    actual_value = tree.height(&nodes);
    expect_value = 1;
    if actual_value != expect_value {
        panic!("Got {} expected {}", actual_value, expect_value)
    }

    tree.put(Item2 { key: 2, value: 1 }, &mut nodes);
    actual_value = tree.height(&nodes);
    expect_value = 1;
    if actual_value != expect_value {
        panic!("Got {} expected {}", actual_value, expect_value)
    }

    tree.put(Item2 { key: 3, value: 2 }, &mut nodes);
    nodes.iterator();
    actual_value = tree.height(&nodes);
    expect_value = 2;
    if actual_value != expect_value {
        panic!("Got {} expected {}", actual_value, expect_value)
    }

    tree.put(Item2 { key: 4, value: 2 }, &mut nodes);
    actual_value = tree.height(&nodes);
    expect_value = 2;
    if actual_value != expect_value {
        panic!("Got {} expected {}", actual_value, expect_value)
    }

    tree.put(Item2 { key: 5, value: 2 }, &mut nodes);
    actual_value = tree.height(&nodes);
    expect_value = 2;
    if actual_value != expect_value {
        panic!("Got {} expected {}", actual_value, expect_value)
    }

    tree.put(Item2 { key: 6, value: 2 }, &mut nodes);
    actual_value = tree.height(&nodes);
    expect_value = 2;
    if actual_value != expect_value {
        panic!("Got {} expected {}", actual_value, expect_value)
    }

    tree.put(Item2 { key: 7, value: 2 }, &mut nodes);
    actual_value = tree.height(&nodes);
    expect_value = 3;
    if actual_value != expect_value {
        panic!("Got {} expected {}", actual_value, expect_value)
    }
}

#[test]
fn test_btree_left_and_right() {
    let mut nodes_map: HashMap<i32, Node<Item>> = HashMap::new();
    let mut nodes = Nodes {
        nodes_map,
        size: 0,
        content_size: 0,
        root_id: 0,
        next_id: 0,
        m: 0,
    };
    let mut tree = MerkleBTree::new_empty(3, &mut nodes);
    let mut actual_value = 0;

    actual_value = tree.left(nodes.root_id, &nodes);
    if actual_value != -1 {
        panic!("Got {} expected {}", actual_value, -1);
    }

    actual_value = tree.right(nodes.root_id, &nodes);
    if actual_value != -1 {
        panic!("Got {} expected {}", actual_value, -1);
    }

    tree.put(
        Item {
            key: 1,
            value: String::from("a"),
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
            key: 6,
            value: String::from("f"),
        },
        &mut nodes,
    );
    tree.put(
        Item {
            key: 7,
            value: String::from("g"),
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
            key: 1,
            value: String::from("x"),
        },
        &mut nodes,
    ); // overwrite
    tree.put(
        Item {
            key: 2,
            value: String::from("b"),
        },
        &mut nodes,
    );

    let mut expected_item = Item {
        key: 1,
        value: String::from("x"),
    };

    let mut item = tree.leftItem(nodes.root_id, &mut nodes).unwrap();
    if item.value != expected_item.value || item.key != expected_item.key {
        panic!("Got {:?} expected {:?}", item, expected_item);
    }

    item = tree.rightItem(nodes.root_id,&mut nodes).unwrap();
    expected_item = Item {
        key: 7,
        value: String::from("g"),
    };
    if item.value != expected_item.value || item.key != expected_item.key {
        panic!("Got {:?} expected {:?}", item, expected_item);
    }
}
