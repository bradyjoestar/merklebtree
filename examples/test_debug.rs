use merklebtree::iterator::*;
use merklebtree::merklebtree::{MerkleBTree, Nodes};
use merklebtree::node::Node;
use std::cmp::Ordering;
use std::collections::HashMap;
use std::fmt::Debug;

#[derive(Clone, Debug)]
pub struct Item {
    pub key: i32,
    pub value: String,
}

impl PartialEq for Item {
    fn eq(&self, other: &Self) -> bool {
        self.key == other.key
    }
}
impl Eq for Item {}

impl Ord for Item {
    fn cmp(&self, other: &Self) -> Ordering {
        (self.key).cmp(&(other.key))
    }
}

impl PartialOrd for Item {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[derive(Clone, Debug)]
pub struct Item2 {
    pub key: i32,
    pub value: i32,
}

impl PartialEq for Item2 {
    fn eq(&self, other: &Self) -> bool {
        self.key == other.key
    }
}
impl Eq for Item2 {}

impl Ord for Item2 {
    fn cmp(&self, other: &Self) -> Ordering {
        (self.key).cmp(&(other.key))
    }
}

impl PartialOrd for Item2 {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[derive(Clone, Debug)]
pub struct Item3 {
    pub key: i32,
}

impl PartialEq for Item3 {
    fn eq(&self, other: &Self) -> bool {
        self.key == other.key
    }
}
impl Eq for Item3 {}

impl Ord for Item3 {
    fn cmp(&self, other: &Self) -> Ordering {
        (self.key).cmp(&(other.key))
    }
}

impl PartialOrd for Item3 {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

pub fn main() {
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
    //
    //    loop {
    //        if !next(&mut btree_iterator) {
    //            break;
    //        }
    //        count = count + 1;
    //        let mut key = item(&mut btree_iterator).key;
    //
    //        match key {
    //            count => {
    //                let actual_value = key;
    //                let expected_value = count;
    //                if actual_value != expected_value {
    //                    panic!("Got {} expected {}", actual_value, expected_value);
    //                }
    //            }
    //            _ => {
    //                let actual_value = key;
    //                let expected_value = count;
    //                if actual_value != expected_value {
    //                    panic!("Got {} expected {}", actual_value, expected_value);
    //                }
    //            }
    //        }
    //    }
    //    let actual_value = count;
    //    let expected_value = btree_iterator.nodes.content_size;
    //    if actual_value != expected_value {
    //        panic!("Got {} expected {}", actual_value, expected_value);
    //    }
    loop {
        if !next(&mut btree_iterator) {
            println!("outside break");
            break;
        }
    }
}
