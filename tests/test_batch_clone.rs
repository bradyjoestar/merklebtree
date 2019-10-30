use merklebtree::merklebtree::{MerkleBTree, Nodes};
use merklebtree::node::Node;
use merklebtree::traits::CalculateHash;
use serde::{Deserialize, Serialize};
use std::cmp::Ordering;
use std::collections::HashMap;
use std::fmt::Debug;

mod utils;
use ring::digest;
use utils::*;

#[test]
fn test_batch_remove_clone_subnode_from_root() {
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

    for i in 2..200 {
        let item = Item4 { key: i, value: 2 };
        tree.put(item, &mut nodes);
    }
    let mut subnodes;

    nodes.iterator();

    for j in 2..200 {
        println!("remove loop:\n\n");
        println!("{}", j);
        subnodes = tree.remove_clone(Item4 { key: j, value: 2 }, &mut nodes);

        let node_hash = nodes.merkleroot();
        tree.remove(Item4 { key: j, value: 2 }, &mut subnodes);
        let subnode_hash = subnodes.merkleroot();
        println!("node_hash:{}", node_hash);
        println!("subnode_hash:{}", subnode_hash);
        assert_eq!(node_hash, subnode_hash);
    }
}

#[test]
fn test_batch_put_clone_subnode_from_root() {
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
        let item = Item4 { key: j, value: 3 };
        subnodes = tree.put_clone(item.clone(), &mut nodes);
        tree.put(item, &mut subnodes);
        let node_hash = nodes.merkleroot();
        let subnode_hash = subnodes.merkleroot();
        assert_eq!(node_hash, subnode_hash);
    }
}

#[test]
fn test_batch_insert_clone_subnode_from_root() {
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
    nodes.iterator();
    let mut subnodes;

    for i in 3..500 {
        println!("loop time:{}", i);
        let item = Item4 { key: i, value: 2 };
        nodes.iterator();
        subnodes = tree.put_clone(item.clone(), &mut nodes);
        print_subnodes_nodemap(&mut subnodes);

        nodes.iterator();
        tree.put(item, &mut subnodes);

        let node_hash = nodes.merkleroot();
        let subnodes_hash = subnodes.merkleroot();
        println!("node_hash:{}", node_hash);
        println!("subnodes_hash:{}", subnodes_hash);
        assert_eq!(node_hash, subnodes_hash);
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
            a.push(b);
            looptime = looptime + 1;
        } else {
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
