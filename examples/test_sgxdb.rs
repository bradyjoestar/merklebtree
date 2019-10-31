use merklebtree::merklebtree::{MerkleBTree, Nodes};
use merklebtree::node::Node;
use merklebtree::traits::CalculateHash;
use serde::{Deserialize, Serialize};
use std::cmp::Ordering;
use std::collections::HashMap;
use std::fmt::Debug;

use ring::digest;

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct Item4 {
    pub key: i32,
    pub value: i32,
}

impl PartialEq for Item4 {
    fn eq(&self, other: &Self) -> bool {
        self.key == other.key
    }
}
impl Eq for Item4 {}

impl Ord for Item4 {
    fn cmp(&self, other: &Self) -> Ordering {
        (self.key).cmp(&(other.key))
    }
}

impl PartialOrd for Item4 {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl CalculateHash for Item4 {
    fn calculate(&self) -> String {
        let mut hash_str = self.key.to_string();
        hash_str.push_str(self.value.to_string().as_str());
        let hash = digest::digest(&digest::SHA256, hash_str.as_str().as_ref());
        let hex = hex::encode(hash);
        hex
    }
}

fn main() {
    println!("test_batch_get_clone_subnode_from_root");

    let mut nodes_map: HashMap<i32, Node<Item4>> = HashMap::new();
    let mut nodes = Nodes {
        nodes_map,
        size: 0,
        root_id: 0,
        content_size: 0,
        next_id: 0,
        m: 0,
    };
    let mut tree = MerkleBTree::new_empty(3, &mut nodes);
    for i in 1..500 {
        let item = Item4 { key: i, value: 2 };
        tree.put(item, &mut nodes);
    }
    let mut subnodes;

    for j in 1..500 {
        let item = Item4 { key: j, value: 2 };
        subnodes = tree.clone_search_subnode_from_root(0, &item, &mut nodes);

        print_subnodes_nodemap_existed(&mut subnodes);

        if !verify_subnodes_hash(&subnodes) {
            panic!("verified failed");
        }

        let (node_id, index, found) = tree.search_recursively(0, &item, &mut subnodes);
        assert_eq!(true, found)
    }
}

fn print_subnodes_nodemap_existed<T>(subnodes: &mut Nodes<T>)
where
    T: PartialEq + PartialOrd + Ord + Clone + Debug + CalculateHash,
{
    println!("\n\n------------------subnodes node_map start---------------------");
    let mut a = Vec::new();

    let mut looptime = 0;

    'outer: loop {
        if a.len() == 0 {
            let mut b: Vec<&Node<T>> = Vec::new();
            let node = subnodes.nodes_map.get(&0).unwrap();
            b.push(node);
            a.push(b);
            looptime = looptime + 1;
        } else {
            let mut existed = false;
            let pre_vec = a.remove(looptime - 1);
            let len = pre_vec.len();
            let mut b: Vec<&Node<T>> = Vec::new();
            for i in 0..len {
                let node = pre_vec.get(i).unwrap();
                if node.children_id.len() == 0 {
                    a.insert(looptime - 1, pre_vec);
                    break 'outer;
                }

                for i in 0..node.children_id.len() {
                    let node_id = node.children_id.get(i).unwrap();
                    if subnodes.nodes_map.contains_key(node_id) {
                        existed = true;
                        let node = subnodes.nodes_map.get(node_id).unwrap();
                        b.push(node);
                    }
                }
            }
            a.insert(looptime - 1, pre_vec);
            a.push(b);
            looptime = looptime + 1;
            if !existed {
                break 'outer;
            }
        }
    }

    for i in 0..a.len() {
        println!("****************************************************");
        let sub_vec = a.get(i).unwrap();
        for j in 0..sub_vec.len() {
            let node = *sub_vec.get(j).unwrap();
            println!("node.node_id: {}", node.node_id);
            println!("node.children_id: {:?}", node.children_id);
            println!("node.children_hash:{:?}", node.children_hash);
            println!("node.content: {:?}", node.content);
            println!("node.parent_id: {:?}", node.parent_id);
            println!("node.hash:{}", node.hash);
        }
        println!("****************************************************");
    }

    println!("------------------subnodes node_map end---------------------");
}

fn verify_subnodes_hash<T>(subnodes: &Nodes<T>) -> bool
where
    T: PartialEq + PartialOrd + Ord + Clone + Debug + CalculateHash,
{
    return verify_nodes_hash(0, subnodes);
}

fn verify_nodes_hash<T>(node_id: i32, subnodes: &Nodes<T>) -> bool
where
    T: PartialEq + PartialOrd + Ord + Clone + Debug + CalculateHash,
{
    let mut verify_result = false;

    let node = subnodes.nodes_map.get(&node_id).unwrap();
    let mut hash = String::new();
    let mut compute_hash;
    for i in node.content.iter() {
        hash.push_str(i.calculate().as_str());
    }
    let mut looptime = 0;
    for i in node.children_id.iter() {
        if subnodes.nodes_map.contains_key(i) {
            let child_node = subnodes.nodes_map.get(i).unwrap();
            let child_node_hash = verify_nodes_hash(child_node.node_id, subnodes);
            if !child_node_hash {
                panic!("verified failed");
            };
            hash.push_str(child_node.hash.as_str());
        } else {
            hash.push_str(node.children_hash.get(looptime).unwrap())
        }
        looptime = looptime + 1;
    }

    compute_hash = hex::encode(digest::digest(&digest::SHA256, hash.as_ref()));
    if compute_hash == node.hash {
        return true;
    } else {
        return false;
    }
}

fn print_subnodes_nodemap<T>(subnodes: &mut Nodes<T>)
where
    T: PartialEq + PartialOrd + Ord + Clone + Debug + CalculateHash,
{
    println!("\n\n------------------subnodes node_map start---------------------");
    let mut a = Vec::new();

    let mut looptime = 0;

    'outer: loop {
        if a.len() == 0 {
            let mut b: Vec<&Node<T>> = Vec::new();
            let node = subnodes.nodes_map.get(&0).unwrap();
            b.push(node);
            println!("b.len:{}", b.len());
            a.push(b);
            looptime = looptime + 1;
        } else {
            let pre_vec = a.remove(looptime - 1);
            let len = pre_vec.len();
            let mut b: Vec<&Node<T>> = Vec::new();
            for i in 0..len {
                let node = pre_vec.get(i).unwrap();
                println!("node:{:?}", node);
                if node.children_id.len() == 0 {
                    a.insert(looptime - 1, pre_vec);
                    break 'outer;
                }

                for i in 0..node.children_id.len() {
                    let node_id = node.children_id.get(i).unwrap();
                    if subnodes.nodes_map.contains_key(node_id) {
                        let node = subnodes.nodes_map.get(node_id).unwrap();
                        b.push(node);
                    }
                }
            }
            a.insert(looptime - 1, pre_vec);
            a.push(b);
            looptime = looptime + 1;
        }
    }

    for i in 0..a.len() {
        println!("****************************************************");
        let sub_vec = a.get(i).unwrap();
        for j in 0..sub_vec.len() {
            let node = *sub_vec.get(j).unwrap();
            println!("node.node_id: {}", node.node_id);
            println!("node.children_id: {:?}", node.children_id);
            println!("node.children_hash:{:?}", node.children_hash);
            println!("node.content: {:?}", node.content);
            println!("node.parent_id: {:?}", node.parent_id);
            println!("node.hash:{}", node.hash);
        }
        println!("****************************************************");
    }

    println!("------------------subnodes node_map end---------------------");
}
