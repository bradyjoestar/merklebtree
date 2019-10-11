use crate::Node;
use crate::{MerkleBTree, Nodes};
use std::cmp::Ordering;
use std::collections::HashMap;

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

pub fn test_debug() {
    let mut nodes_map: HashMap<i32, Node<Item>> = HashMap::new();
    let mut nodes = Nodes {
        nodes_map,
        size: 0,
        root_id: 0,
        content_size: 0,
        next_id: 0,
        m: 0,
    };
    let mut tree = MerkleBTree::new_empty(3, &mut nodes);
    let mut testdata: Vec<Vec<(Item, bool)>> = Vec::new();

    println!("test_debug");
    tree.put(
        Item {
            key: 7,
            value: String::from("g"),
        },
        &mut nodes,
    );
    println!("test_debug1");
    tree.put(
        Item {
            key: 9,
            value: String::from("i"),
        },
        &mut nodes,
    );
    println!("test_debug2");
    tree.put(
        Item {
            key: 10,
            value: String::from("j"),
        },
        &mut nodes,
    );
    println!("test_debug3");
    tree.put(
        Item {
            key: 6,
            value: String::from("f"),
        },
        &mut nodes,
    );
    nodes.iterator();
    println!("test_debug4");
    tree.put(
        Item {
            key: 3,
            value: String::from("c"),
        },
        &mut nodes,
    );
    println!("test_debug5");
    tree.put(
        Item {
            key: 4,
            value: String::from("d"),
        },
        &mut nodes,
    );
    println!("test_debug6");
    tree.put(
        Item {
            key: 5,
            value: String::from("e"),
        },
        &mut nodes,
    );
    println!("test_debug7");
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
}
