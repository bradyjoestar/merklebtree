use merklebtree::merklebtree::{MerkleBTree, Nodes};
use merklebtree::node::Node;
use merklebtree::traits::CalculateHash;
use serde::{Deserialize, Serialize};
use std::cmp::Ordering;
use std::collections::HashMap;
use std::fmt::Debug;

extern crate ring;
use ring::digest;
use ring::digest::Digest;

#[derive(Clone, Serialize, Deserialize, Debug)]
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

impl CalculateHash for Item {
    fn calculate(&self) -> String {
        let hash = digest::digest(&digest::SHA256, self.key.to_string().as_ref());
        let hex = hex::encode(hash);
        hex
    }
}

impl PartialOrd for Item {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[derive(Clone, Serialize, Deserialize, Debug)]
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

impl CalculateHash for Item2 {
    fn calculate(&self) -> String {
        let hash = digest::digest(&digest::SHA256, self.key.to_string().as_ref());
        let hex = hex::encode(hash);
        hex
    }
}

#[derive(Clone, Serialize, Deserialize, Debug)]
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

impl CalculateHash for Item3 {
    fn calculate(&self) -> String {
        let hash = digest::digest(&digest::SHA256, self.key.to_string().as_ref());
        let hex = hex::encode(hash);
        hex
    }
}

pub fn assert_valid_tree<T>(nodes: &Nodes<T>, expected_size: i32)
where
    T: PartialEq + PartialOrd + Ord + Clone + Debug + CalculateHash,
{
    let (actual_value, expected_value) = (nodes.content_size, expected_size);
    if actual_value != expected_value as i32 {
        panic!(
            "Got {} expected {} for content size",
            actual_value, expected_value
        );
    }
}

pub fn assert_valid_tree_node(
    branch: &Vec<i32>, //from root  i.e vec![0,1,2] //0 replace root
    expected_contents: i32,
    expected_children: i32,
    keys: &Vec<i32>,
    has_parent: bool,
    nodes: &Nodes<Item2>,
) {
    let node_id = find_nodeid_by_branch(branch, nodes);
    let node = nodes.nodes_map.get(&node_id).unwrap();
    let actual_value = node.parent_id != -1;
    if actual_value != has_parent {
        panic!(
            "Got {} expected {} for has_parent",
            actual_value, has_parent
        );
    }
    let actual_value = node.content.len();
    if actual_value != expected_contents as usize {
        panic!(
            "Got {} expected {} for contents size",
            actual_value, expected_contents
        );
    }
    let actual_value = node.children_id.len();
    if actual_value != expected_children as usize {
        panic!(
            "Got {} expected {} for contents size",
            actual_value, expected_children
        );
    }

    let mut loop_time = 0;
    for i in keys.iter() {
        let actual_vale = node.content.get(loop_time).unwrap();
        println!("{:?}", actual_vale);
        if actual_vale.key != *i {
            panic!("Got {} expected {} for for Key", actual_vale.key, *i);
        }
        loop_time = loop_time + 1;
    }
}

pub fn assert_valid_tree_node_item3(
    branch: &Vec<i32>, //from root  i.e vec![0,1,2] //0 replace root
    expected_contents: i32,
    expected_children: i32,
    keys: &Vec<i32>,
    has_parent: bool,
    nodes: &Nodes<Item3>,
) {
    let node_id = find_nodeid_by_branch(branch, nodes);
    let node = nodes.nodes_map.get(&node_id).unwrap();
    let actual_value = node.parent_id != -1;
    if actual_value != has_parent {
        panic!(
            "Got {} expected {} for has_parent",
            actual_value, has_parent
        );
    }
    let actual_value = node.content.len();
    if actual_value != expected_contents as usize {
        panic!(
            "Got {} expected {} for contents size",
            actual_value, expected_contents
        );
    }
    let actual_value = node.children_id.len();
    if actual_value != expected_children as usize {
        panic!(
            "Got {} expected {} for contents size",
            actual_value, expected_children
        );
    }

    let mut loop_time = 0;
    for i in keys.iter() {
        let actual_vale = node.content.get(loop_time).unwrap();
        println!("{:?}", actual_vale);
        if actual_vale.key != *i {
            panic!("Got {} expected {} for for Key", actual_vale.key, *i);
        }
        loop_time = loop_time + 1;
    }
}

pub fn find_nodeid_by_branch<T>(branch: &Vec<i32>, nodes: &Nodes<T>) -> i32
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
