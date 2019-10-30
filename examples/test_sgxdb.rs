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
    tree.put(Item4 { key: 0, value: 2 }, &mut nodes);
    tree.put(Item4 { key: 1, value: 2 }, &mut nodes);

    for i in 2..11 {
        let item = Item4 { key: i, value: 2 };
        tree.put(item, &mut nodes);
    }

    nodes.iterator();

    for j in 2..5 {
        println!("remove loop:\n\n");
        println!("{}", j);
        nodes.iterator();
        tree.remove(Item4 { key: j, value: 2 }, &mut nodes);

        //        print_subnodes_nodemap(&mut subnodes);
        let node_hash = nodes.merkleroot();
        //        tree.remove_clone(Item4 { key: j, value: 2 }, &mut subnodes);
        //        let subnode_hash = subnodes.merkleroot();
        println!("node_hash:{}", node_hash);
        //        println!("subnode_hash:{}", subnode_hash);
    }
    println!("remove end:\n\n");
    nodes.iterator();
}

fn print_subnodes_nodemap<T>(subnodes: &mut Nodes<T>)
where
    T: PartialEq + PartialOrd + Ord + Clone + Debug + CalculateHash,
{
    println!("------------------subnodes node_map start---------------------");
    let mut a = Vec::new();

    let mut looptime = 0;

    'outer: loop {
        println!("a.len:{}", a.len());
        if a.len() == 0 {
            let mut b: Vec<&Node<T>> = Vec::new();
            let node = subnodes.nodes_map.get(&0).unwrap();
            b.push(node);
            a.push(b);
            looptime = looptime + 1;
        } else {
            let pre_vec = a.remove(looptime - 1);
            let len = pre_vec.len();
            println!("pre_vec.len:{:?}", pre_vec.len());
            println!("{:?}", pre_vec);
            let mut b: Vec<&Node<T>> = Vec::new();
            for j in 0..len {
                let node = pre_vec.get(j).unwrap();
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
            println!("node.content: {:?}", node.content);
            println!("node.parent_id: {:?}", node.parent_id);
            println!("node.hash:{}", node.hash);
        }
        println!("****************************************************");
    }

    println!("------------------subnodes node_map end---------------------");
}
