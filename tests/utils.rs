use merklebtree::merklebtree::{MerkleBTree, Nodes};
use merklebtree::node::Node;
use merklebtree::traits::CalculateHash;
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

impl CalculateHash for Item {
    fn calculate(&self) -> String {
        String::new()
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

impl CalculateHash for Item2 {
    fn calculate(&self) -> String {
        String::new()
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

impl CalculateHash for Item3 {
    fn calculate(&self) -> String {
        String::new()
    }
}

pub fn assertValidTree<T>(nodes: &Nodes<T>, expectedSize: i32)
where
    T: PartialEq + PartialOrd + Ord + Clone + Debug + CalculateHash,
{
    let (actualValue, expectedValue) = (nodes.content_size, expectedSize);
    if actualValue != expectedValue as i32 {
        panic!(
            "Got {} expected {} for content size",
            actualValue, expectedValue
        );
    }
}

pub fn assertValidTreeNode(
    branch: &Vec<i32>, //from root  i.e vec![0,1,2] //0 replace root
    expectedContents: i32,
    expectedChildren: i32,
    keys: &Vec<i32>,
    hasParent: bool,
    nodes: &Nodes<Item2>,
) {
    let node_id = find_nodeid_by_branch(branch, nodes);
    let node = nodes.nodes_map.get(&node_id).unwrap();
    let actualValue = node.parent_id != -1;
    if actualValue != hasParent {
        panic!("Got {} expected {} for hasParent", actualValue, hasParent);
    }
    let actualValue = node.content.len();
    if actualValue != expectedContents as usize {
        panic!(
            "Got {} expected {} for contents size",
            actualValue, expectedContents
        );
    }
    let actualValue = node.children_id.len();
    if actualValue != expectedChildren as usize {
        panic!(
            "Got {} expected {} for contents size",
            actualValue, expectedChildren
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

pub fn assertValidTreeNodeItem3(
    branch: &Vec<i32>, //from root  i.e vec![0,1,2] //0 replace root
    expectedContents: i32,
    expectedChildren: i32,
    keys: &Vec<i32>,
    hasParent: bool,
    nodes: &Nodes<Item3>,
) {
    let node_id = find_nodeid_by_branch(branch, nodes);
    let node = nodes.nodes_map.get(&node_id).unwrap();
    let actualValue = node.parent_id != -1;
    if actualValue != hasParent {
        panic!("Got {} expected {} for hasParent", actualValue, hasParent);
    }
    let actualValue = node.content.len();
    if actualValue != expectedContents as usize {
        panic!(
            "Got {} expected {} for contents size",
            actualValue, expectedContents
        );
    }
    let actualValue = node.children_id.len();
    if actualValue != expectedChildren as usize {
        panic!(
            "Got {} expected {} for contents size",
            actualValue, expectedChildren
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
