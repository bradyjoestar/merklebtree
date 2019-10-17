use merklebtree::iterator::*;
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

    item = tree.rightItem(nodes.root_id, &mut nodes).unwrap();
    expected_item = Item {
        key: 7,
        value: String::from("g"),
    };
    if item.value != expected_item.value || item.key != expected_item.key {
        panic!("Got {:?} expected {:?}", item, expected_item);
    }
}

#[test]
fn test_btree_iterator_values_and_keys() {
    let mut nodes_map: HashMap<i32, Node<Item>> = HashMap::new();
    let mut nodes = Nodes {
        nodes_map,
        size: 0,
        content_size: 0,
        root_id: 0,
        next_id: 0,
        m: 0,
    };
    let mut tree = MerkleBTree::new_empty(4, &mut nodes);

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
            key: 1,
            value: String::from("a"),
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
            key: 4,
            value: String::from("d"),
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
            value: String::from("x"),
        },
        &mut nodes,
    ); // overwrite

    let mut btree_iterator = new_btree_iterator(&mut nodes, position::begin, &mut tree);
    let content_vec = contents(&mut btree_iterator);
    println!("{:?}", content_vec);

    let expected_value = String::from("xbcdefg");
    let expected_key = String::from("1234567");
    let mut actual_key = String::new();
    let mut actual_value = String::new();
    for i in content_vec.iter() {
        actual_key.push_str(i.key.to_string().as_str());
        actual_value.push_str(i.value.as_str());
    }
    if actual_value != expected_value || actual_key != expected_key {
        panic!(
            "Got {} expected {}; Got {} expected{} ",
            actual_value, expected_value, actual_key, expected_key
        );
    }

    if nodes.content_size != 7 {
        panic!("Got {} expected {}", nodes.content_size, 7);
    }
}

#[test]
fn test_btree_iterator_next_on_empty() {
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
    let mut btree_iterator = new_btree_iterator(&mut nodes, position::begin, &mut tree);

    loop {
        if !next(&mut btree_iterator) {
            break;
        }
        panic!("Shouldn't iterate on empty tree");
    }
}

#[test]
fn test_btree_iterator_prev_on_empty() {
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
    let mut btree_iterator = new_btree_iterator(&mut nodes, position::begin, &mut tree);

    loop {
        if !prev(&mut btree_iterator) {
            break;
        }
        panic!("Shouldn't iterate on empty tree");
    }
}
//
#[test]
fn test_btree_iterator_1_next() {
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
    ); // overwrite
    let mut btree_iterator = new_btree_iterator(&mut nodes, position::begin, &mut tree);

    let mut count = 0;

    loop {
        if !next(&mut btree_iterator) {
            break;
        }
        count = count + 1;
        let mut key = item(&mut btree_iterator).key;

        match key {
            count => {
                let actual_value = key;
                let expected_value = count;
                if actual_value != expected_value {
                    panic!("Got {} expected {}", actual_value, expected_value);
                }
            }
            _ => {
                let actual_value = key;
                let expected_value = count;
                if actual_value != expected_value {
                    panic!("Got {} expected {}", actual_value, expected_value);
                }
            }
        }
    }
    let actual_value = count;
    let expected_value = btree_iterator.nodes.content_size;
    if actual_value != expected_value {
        panic!("Got {} expected {}", actual_value, expected_value);
    }
}

#[test]
fn test_btree_iterator_1_prev() {
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
    ); // overwrite
    let mut btree_iterator = new_btree_iterator(&mut nodes, position::begin, &mut tree);

    let mut count_down = btree_iterator.nodes.content_size;

    loop {
        if !prev(&mut btree_iterator) {
            break;
        }
        let mut key = item(&mut btree_iterator).key;

        match key {
            count => {
                let actual_value = key;
                let expected_value = count_down;
                if actual_value != expected_value {
                    panic!("Got {} expected {}", actual_value, expected_value);
                }
            }
            _ => {
                let actual_value = key;
                let expected_value = count_down;
                if actual_value != expected_value {
                    panic!("Got {} expected {}", actual_value, expected_value);
                }
            }
        }
        count_down = count_down - 1;
    }
    let actual_value = count_down;
    let expected_value = btree_iterator.nodes.content_size;
    if actual_value != expected_value {
        panic!("Got {} expected {}", actual_value, expected_value);
    }
}

#[test]
fn test_btree_iterator_2_next() {
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

    tree.put(
        Item {
            key: 3,
            value: String::from("c"),
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
    tree.put(
        Item {
            key: 2,
            value: String::from("b"),
        },
        &mut nodes,
    );
    let mut btree_iterator = new_btree_iterator(&mut nodes, position::begin, &mut tree);

    let mut count = 0;

    loop {
        if !next(&mut btree_iterator) {
            break;
        }
        count = count + 1;
        let mut key = item(&mut btree_iterator).key;

        match key {
            count => {
                let actual_value = key;
                let expected_value = count;
                if actual_value != expected_value {
                    panic!("Got {} expected {}", actual_value, expected_value);
                }
            }
            _ => {
                let actual_value = key;
                let expected_value = count;
                if actual_value != expected_value {
                    panic!("Got {} expected {}", actual_value, expected_value);
                }
            }
        }
    }
    let actual_value = count;
    let expected_value = btree_iterator.nodes.content_size;
    if actual_value != expected_value {
        panic!("Got {} expected {}", actual_value, expected_value);
    }
}

#[test]
fn test_btree_iterator_2_prev() {
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

    tree.put(
        Item {
            key: 3,
            value: String::from("c"),
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
    tree.put(
        Item {
            key: 2,
            value: String::from("b"),
        },
        &mut nodes,
    );
    let mut btree_iterator = new_btree_iterator(&mut nodes, position::begin, &mut tree);

    let mut count_down = btree_iterator.nodes.content_size;

    loop {
        if !prev(&mut btree_iterator) {
            break;
        }
        let mut key = item(&mut btree_iterator).key;

        match key {
            count => {
                let actual_value = key;
                let expected_value = count_down;
                if actual_value != expected_value {
                    panic!("Got {} expected {}", actual_value, expected_value);
                }
            }
            _ => {
                let actual_value = key;
                let expected_value = count_down;
                if actual_value != expected_value {
                    panic!("Got {} expected {}", actual_value, expected_value);
                }
            }
        }
        count_down = count_down - 1;
    }
    let actual_value = count_down;
    let expected_value = btree_iterator.nodes.content_size;
    if actual_value != expected_value {
        panic!("Got {} expected {}", actual_value, expected_value);
    }
}

#[test]
fn test_btree_iterator_3_next() {
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

    tree.put(
        Item {
            key: 1,
            value: String::from("a"),
        },
        &mut nodes,
    );
    let mut btree_iterator = new_btree_iterator(&mut nodes, position::begin, &mut tree);

    let mut count = 0;

    loop {
        if !next(&mut btree_iterator) {
            break;
        }
        count = count + 1;
        let mut key = item(&mut btree_iterator).key;

        match key {
            count => {
                let actual_value = key;
                let expected_value = count;
                if actual_value != expected_value {
                    panic!("Got {} expected {}", actual_value, expected_value);
                }
            }
            _ => {
                let actual_value = key;
                let expected_value = count;
                if actual_value != expected_value {
                    panic!("Got {} expected {}", actual_value, expected_value);
                }
            }
        }
    }
    let actual_value = count;
    let expected_value = btree_iterator.nodes.content_size;
    if actual_value != expected_value {
        panic!("Got {} expected {}", actual_value, expected_value);
    }
}

#[test]
fn test_btree_iterator_3_prev() {
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

    tree.put(
        Item {
            key: 1,
            value: String::from("a"),
        },
        &mut nodes,
    );
    let mut btree_iterator = new_btree_iterator(&mut nodes, position::begin, &mut tree);

    let mut count_down = btree_iterator.nodes.content_size;

    loop {
        if !prev(&mut btree_iterator) {
            break;
        }
        let mut key = item(&mut btree_iterator).key;

        match key {
            count => {
                let actual_value = key;
                let expected_value = count_down;
                if actual_value != expected_value {
                    panic!("Got {} expected {}", actual_value, expected_value);
                }
            }
            _ => {
                let actual_value = key;
                let expected_value = count_down;
                if actual_value != expected_value {
                    panic!("Got {} expected {}", actual_value, expected_value);
                }
            }
        }
        count_down = count_down - 1;
    }
    let actual_value = count_down;
    let expected_value = btree_iterator.nodes.content_size;
    if actual_value != expected_value {
        panic!("Got {} expected {}", actual_value, expected_value);
    }
}

#[test]
fn test_btree_iterator_4_next() {
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

    tree.put(Item2 { key: 13, value: 5 }, &mut nodes);
    tree.put(Item2 { key: 8, value: 3 }, &mut nodes);
    tree.put(Item2 { key: 17, value: 7 }, &mut nodes);
    tree.put(Item2 { key: 1, value: 1 }, &mut nodes);
    tree.put(Item2 { key: 11, value: 4 }, &mut nodes);
    tree.put(Item2 { key: 15, value: 6 }, &mut nodes);
    tree.put(Item2 { key: 25, value: 9 }, &mut nodes);
    tree.put(Item2 { key: 6, value: 2 }, &mut nodes);
    tree.put(Item2 { key: 22, value: 8 }, &mut nodes);
    tree.put(Item2 { key: 27, value: 10 }, &mut nodes);

    let mut btree_iterator = new_btree_iterator(&mut nodes, position::begin, &mut tree);

    let mut count = 0;

    loop {
        if !next(&mut btree_iterator) {
            break;
        }
        count = count + 1;
        let mut key = item(&mut btree_iterator).key;

        match key {
            count => {
                let actual_value = key;
                let expected_value = count;
                if actual_value != expected_value {
                    panic!("Got {} expected {}", actual_value, expected_value);
                }
            }
            _ => {
                let actual_value = key;
                let expected_value = count;
                if actual_value != expected_value {
                    panic!("Got {} expected {}", actual_value, expected_value);
                }
            }
        }
    }
    let actual_value = count;
    let expected_value = btree_iterator.nodes.content_size;
    if actual_value != expected_value {
        panic!("Got {} expected {}", actual_value, expected_value);
    }
}

#[test]
fn test_btree_iterator_4_prev() {
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

    tree.put(Item2 { key: 13, value: 5 }, &mut nodes);
    tree.put(Item2 { key: 8, value: 3 }, &mut nodes);
    tree.put(Item2 { key: 17, value: 7 }, &mut nodes);
    tree.put(Item2 { key: 1, value: 1 }, &mut nodes);
    tree.put(Item2 { key: 11, value: 4 }, &mut nodes);
    tree.put(Item2 { key: 15, value: 6 }, &mut nodes);
    tree.put(Item2 { key: 25, value: 9 }, &mut nodes);
    tree.put(Item2 { key: 6, value: 2 }, &mut nodes);
    tree.put(Item2 { key: 22, value: 8 }, &mut nodes);
    tree.put(Item2 { key: 27, value: 10 }, &mut nodes);

    let mut btree_iterator = new_btree_iterator(&mut nodes, position::begin, &mut tree);

    let mut count_down = btree_iterator.nodes.content_size;

    loop {
        if !prev(&mut btree_iterator) {
            break;
        }
        let mut key = item(&mut btree_iterator).key;

        match key {
            count => {
                let actual_value = key;
                let expected_value = count_down;
                if actual_value != expected_value {
                    panic!("Got {} expected {}", actual_value, expected_value);
                }
            }
            _ => {
                let actual_value = key;
                let expected_value = count_down;
                if actual_value != expected_value {
                    panic!("Got {} expected {}", actual_value, expected_value);
                }
            }
        }
        count_down = count_down - 1;
    }
    let actual_value = count_down;
    let expected_value = btree_iterator.nodes.content_size;
    if actual_value != expected_value {
        panic!("Got {} expected {}", actual_value, expected_value);
    }
}

#[test]
fn test_btree_iterator_begin() {
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

    tree.put(
        Item {
            key: 3,
            value: String::from("c"),
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
    tree.put(
        Item {
            key: 2,
            value: String::from("b"),
        },
        &mut nodes,
    );
    let mut btree_iterator = new_btree_iterator(&mut nodes, position::begin, &mut tree);
    if btree_iterator.node_id != -1 {
        panic!("Got {} expected {}", btree_iterator.node_id, -1)
    }

    begin(&mut btree_iterator);
    if btree_iterator.node_id != -1 {
        panic!("Got {} expected {}", btree_iterator.node_id, -1)
    }

    loop {
        if next(&mut btree_iterator) {
            break;
        }
    }

    begin(&mut btree_iterator);
    if btree_iterator.node_id != -1 {
        panic!("Got {} expected {}", btree_iterator.node_id, -1)
    }

    next(&mut btree_iterator);
    let item: Item = item(&mut btree_iterator);

    let expected_item = Item {
        key: 1,
        value: String::from("a"),
    };
    if expected_item.key != item.key || expected_item.value != item.value {
        panic!("Got {:?} expected {:?}", item, expected_item);
    }
}
//
#[test]
fn test_btree_iterator_end() {
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
    let mut btree_iterator = new_btree_iterator(&mut nodes, position::begin, &mut tree);

    if btree_iterator.node_id != -1 {
        panic!("Got {} expected {}", btree_iterator.node_id, -1)
    }

    end(&mut btree_iterator);
    if btree_iterator.node_id != -1 {
        panic!("Got {} expected {}", btree_iterator.node_id, -1)
    }

    btree_iterator.mbtree.put(
        Item {
            key: 3,
            value: String::from("c"),
        },
        &mut btree_iterator.nodes,
    );
    btree_iterator.mbtree.put(
        Item {
            key: 1,
            value: String::from("a"),
        },
        &mut btree_iterator.nodes,
    );
    btree_iterator.mbtree.put(
        Item {
            key: 2,
            value: String::from("b"),
        },
        &mut btree_iterator.nodes,
    );

    end(&mut btree_iterator);
    if btree_iterator.node_id != -1 {
        panic!("Got {} expected {}", btree_iterator.node_id, -1)
    }

    prev(&mut btree_iterator);
    let item: Item = item(&mut btree_iterator);
    let expected_item = Item {
        key: 3,
        value: String::from("c"),
    };

    if expected_item.key != item.key || expected_item.value != item.value {
        panic!("Got {:?} expected {:?}", item, expected_item);
    }
}

#[test]
fn test_btree_iterator_first() {
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

    tree.put(
        Item {
            key: 3,
            value: String::from("c"),
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
    tree.put(
        Item {
            key: 2,
            value: String::from("b"),
        },
        &mut nodes,
    );
    let mut btree_iterator = new_btree_iterator(&mut nodes, position::begin, &mut tree);

    let actual_value = first(&mut btree_iterator);
    if actual_value != true {
        panic!("Got {} expected {}", actual_value, true)
    }

    let item: Item = item(&mut btree_iterator);
    let expected_item = Item {
        key: 1,
        value: String::from("a"),
    };

    if expected_item.key != item.key || expected_item.value != item.value {
        panic!("Got {:?} expected {:?}", item, expected_item);
    }
}

#[test]
fn test_btree_iterator_last() {
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

    tree.put(
        Item {
            key: 3,
            value: String::from("c"),
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
    tree.put(
        Item {
            key: 2,
            value: String::from("b"),
        },
        &mut nodes,
    );
    let mut btree_iterator = new_btree_iterator(&mut nodes, position::begin, &mut tree);

    let actual_value = last(&mut btree_iterator);
    if actual_value != true {
        panic!("Got {} expected {}", actual_value, true)
    }

    let item: Item = item(&mut btree_iterator);
    let expected_item = Item {
        key: 3,
        value: String::from("c"),
    };

    if expected_item.key != item.key || expected_item.value != item.value {
        panic!("Got {:?} expected {:?}", item, expected_item);
    }
}

#[test]
fn test_btree_iterator_find_branch() {
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

    let (branch, nodes_map, index, found) =
        tree.find_branch_from_root(0, &Item3 { key: -4 }, &mut nodes);

    assert_eq!(branch, vec![0, 13, 11]);
    assert_eq!(index, 0);
    assert_eq!(found, true);
}
