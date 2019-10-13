extern crate merklebtree;
use merklebtree::merklebtree::{MerkleBTree, Nodes};

use merklebtree::node::Node;
use merklebtree::traits::CalculateHash;
use std::cmp::Ordering;
use std::collections::HashMap;
use std::fmt::Debug;
extern crate serde;

use serde::{Deserialize, Serialize};

#[derive(Clone, Deserialize, Serialize, Debug)]
pub struct Item {
    pub key: u32,
    pub value: u32,
}

impl PartialEq for Item {
    fn eq(&self, other: &Self) -> bool {
        self.key + self.value == other.key + other.value
    }
}
impl Eq for Item {}

impl Ord for Item {
    fn cmp(&self, other: &Self) -> Ordering {
        (self.key + self.value).cmp(&(other.key + other.value))
    }
}

impl PartialOrd for Item {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
impl CalculateHash for Item {
    fn calculate(&self) -> String {
        String::new()
    }
}

#[derive(Clone, Deserialize, Serialize, Debug)]
pub struct Item2 {
    pub key: i32,
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
impl CalculateHash for Item2 {
    fn calculate(&self) -> String {
        String::new()
    }
}

fn main() {
    println!("Hello, world!");

    test1();
}

fn test1() {
    let mut nodes = Nodes {
        nodes_map: Default::default(),
        size: 0,
        root_id: 0,
        content_size: 0,
        next_id: 0,
        m: 0,
        merkleroot_hash: "".to_string(),
    };
    let mut tree = MerkleBTree::new_with(3, Item2 { key: 0 }, &mut nodes);

    nodes.m = tree.m;

    for i in 1..21 {
        let item = Item2 { key: i };
        tree.put(item, &mut nodes);
        println!("total node:{}", nodes.size);
    }

    nodes.iterator();

    let item = Item2 { key: 21 };
    tree.put(item, &mut nodes);
    nodes.iterator();

    println!("--------------remove the content from leaf node---------------------");
    println!("wenbin test");
    tree.remove(Item2 { key: 2 }, &mut nodes);

    nodes.iterator();

    let mut branch = vec![0, 0, 0, 2];
    let find_id = find_nodeid_by_branch(&branch, &nodes);
    println!("{}", find_id);
}

fn test2() {
    let mut nodes = Nodes {
        nodes_map: Default::default(),
        size: 0,
        root_id: 0,
        content_size: 0,
        next_id: 0,
        m: 0,
        merkleroot_hash: "".to_string(),
    };

    let mut tree = MerkleBTree::new_with(5, Item { key: 1, value: 4 }, &mut nodes);

    nodes.m = tree.m;

    for i in 0..30 {
        let item = Item { key: i, value: i };
        tree.put(item, &mut nodes);
        println!("total node:{}", nodes.size);
    }
    let item = Item { key: 3, value: 4 };
    tree.put(item, &mut nodes);

    nodes.iterator();

    tree.put(Item { key: 0, value: 1 }, &mut nodes);
    println!("-----------------------------------");
    let node = nodes.nodes_map.get(&1).unwrap();

    nodes.iterator();

    println!("--------------remove the content from internal node---------------------");
    tree.remove(Item { key: 2, value: 2 }, &mut nodes);

    nodes.iterator();

    println!("--------------remove the content from leaf node---------------------");
    tree.remove(Item { key: 0, value: 1 }, &mut nodes);

    nodes.iterator();
}

fn find_nodeid_by_branch<T>(branch: &Vec<i32>, nodes: &Nodes<T>) -> i32
where
    T: PartialEq + PartialOrd + Ord + Clone + Debug + CalculateHash,
{
    let root_id = *branch.get(0).unwrap();
    let mut node = nodes.nodes_map.get(&root_id).unwrap();
    let mut node_id = 0;
    let mut iter_time = 0;
    for i in branch.iter() {
        if iter_time == 0 {
        } else {
            node_id = *node.children_id.get(*i as usize).unwrap();
            node = nodes.nodes_map.get(&node_id).unwrap();
        }
        iter_time = iter_time + 1;
    }
    node_id
}
