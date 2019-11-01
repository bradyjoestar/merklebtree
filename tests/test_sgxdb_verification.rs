use merklebtree::merklebtree::{MerkleBTree, Nodes};
use merklebtree::node::Node;
use merklebtree::traits::CalculateHash;
use serde::{Deserialize, Serialize};
use std::cmp::Ordering;
use std::collections::HashMap;
use std::fmt::Debug;

pub mod utils;
use merklebtree::sgxdb::verify_subnodes_hash;
use ring::digest;
use utils::*;

#[test]
fn test_sgx_verification() {
    println!("test sgx verification");

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

    let item = Item4 { key: 0, value: 2 };
    let mut root_node = Node::new_node(item.clone(), 0);
    root_node.root_flag = true;

    tree.put_clone(item, &mut nodes);
    let mut subnodes;

    //insert
    for i in 1..500 {
        let insert_item = Item4 { key: i, value: 2 };
        subnodes = tree.put_clone(insert_item.clone(), &mut nodes);

        subnodes.nodes_map.remove(&0).unwrap();
        subnodes.nodes_map.insert(0, root_node.clone());

        verify_subnodes_hash(&subnodes);

        tree.put(insert_item.clone(), &mut subnodes);

        let node_hash = nodes.merkleroot();
        let subnode_hash = subnodes.merkleroot();
        assert_eq!(node_hash, subnode_hash);

        root_node = subnodes.nodes_map.remove(&0).unwrap();
    }

    //put
    for i in 1..500 {
        let insert_item = Item4 { key: i, value: 3 };
        subnodes = tree.put_clone(insert_item.clone(), &mut nodes);

        subnodes.nodes_map.remove(&0).unwrap();
        subnodes.nodes_map.insert(0, root_node.clone());

        verify_subnodes_hash(&subnodes);

        tree.put(insert_item.clone(), &mut subnodes);

        let node_hash = nodes.merkleroot();
        let subnode_hash = subnodes.merkleroot();
        assert_eq!(node_hash, subnode_hash);

        root_node = subnodes.nodes_map.remove(&0).unwrap();
    }

    //get
    for j in 1..500 {
        let item = Item4 { key: j, value: 3 };
        let (node_id, index, found, mut subnodes) =
            tree.clone_search_subnode_from_root(0, &item, &mut nodes);

        subnodes.nodes_map.remove(&0).unwrap();
        subnodes.nodes_map.insert(0, root_node.clone());

        verify_subnodes_hash(&subnodes);

        let (node_id, index, found) = tree.search_recursively(0, &item, &mut subnodes);
        assert_eq!(true, found)
    }

    //remove
    for j in 1..500 {
        subnodes = tree.remove_clone(Item4 { key: j, value: 3 }, &mut nodes);
        subnodes.nodes_map.remove(&0).unwrap();
        subnodes.nodes_map.insert(0, root_node.clone());

        tree.remove(Item4 { key: j, value: 3 }, &mut subnodes);

        let node_hash = nodes.merkleroot();
        let subnode_hash = subnodes.merkleroot();

        assert_eq!(node_hash, subnode_hash);
        root_node = subnodes.nodes_map.remove(&0).unwrap();
    }

    //remove the last node
    let last_item = Item4 { key: 0, value: 2 };
    if root_node.children_id.len() == 0 && root_node.content.len() == 1 {
        let content = root_node.content.pop().unwrap();
        if last_item == content {
            root_node = Node::new_empty(0);
            tree.remove_clone(last_item, &mut nodes);
        } else {
        }
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
