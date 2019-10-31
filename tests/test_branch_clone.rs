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
fn test_clone_search_subnode_from_root() {
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

    let (node_id, index, found, mut subnodes) =
        tree.clone_search_subnode_from_root(0, &Item3 { key: -4 }, &mut nodes);

    let mut subtree = MerkleBTree {
        rootid: subnodes.root_id,
        m: subnodes.m,
    };

    let (value, found) = subtree.get(Item3 { key: -4 }, &mut subnodes);
    assert_eq!(true, found);
}

#[test]
fn test_sgxdb_put_clone_from_root() {
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

    tree.put(Item4 { key: 1, value: 0 }, &mut nodes);
    tree.put(Item4 { key: 2, value: 1 }, &mut nodes);
    tree.put(Item4 { key: 3, value: 2 }, &mut nodes);
    tree.put(Item4 { key: 4, value: 2 }, &mut nodes);
    tree.put(Item4 { key: 5, value: 2 }, &mut nodes);
    tree.put(Item4 { key: 6, value: 2 }, &mut nodes);
    tree.put(Item4 { key: 7, value: 2 }, &mut nodes);
    assert_valid_tree(&nodes, 7);
    assert_valid_tree_node_item4(&vec![0], 1, 2, &vec![4], false, &nodes);
    assert_valid_tree_node_item4(&vec![0, 0], 1, 2, &vec![2], true, &nodes);
    assert_valid_tree_node_item4(&vec![0, 1], 1, 2, &vec![6], true, &nodes);
    assert_valid_tree_node_item4(&vec![0, 0, 0], 1, 0, &vec![1], true, &nodes);
    assert_valid_tree_node_item4(&vec![0, 0, 1], 1, 0, &vec![3], true, &nodes);
    assert_valid_tree_node_item4(&vec![0, 1, 0], 1, 0, &vec![5], true, &nodes);
    assert_valid_tree_node_item4(&vec![0, 1, 1], 1, 0, &vec![7], true, &nodes);

    println!("-------------------put before------------------------");
    nodes.iterator();
    println!("nodes.merkleroot: {:?}", nodes.merkleroot());
    println!("{:?}", nodes.nodes_map);
    let mut subnodes = tree.put_clone(Item4 { key: 7, value: 1 }, &mut nodes);

    println!("-------------------put after------------------------");
    println!("nodes.merkleroot: {:?}", nodes.merkleroot());
    println!("subnodes.merkleroot:{:?}", subnodes.merkleroot());
    nodes.iterator();

    println!("-------------------subnodes put------------------------");
    tree.put(Item4 { key: 7, value: 1 }, &mut subnodes);
    println!("subnodes.merkleroot:{:?}", subnodes.merkleroot());
    println!("subnodes.nodemap:{:?}", subnodes.nodes_map);
}

#[test]
fn test_sgxdb_insert_clone_from_root() {
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

    tree.put(Item4 { key: 1, value: 0 }, &mut nodes);
    tree.put(Item4 { key: 2, value: 1 }, &mut nodes);
    tree.put(Item4 { key: 3, value: 2 }, &mut nodes);
    tree.put(Item4 { key: 4, value: 2 }, &mut nodes);
    tree.put(Item4 { key: 5, value: 2 }, &mut nodes);
    tree.put(Item4 { key: 6, value: 2 }, &mut nodes);
    tree.put(Item4 { key: 7, value: 2 }, &mut nodes);
    assert_valid_tree(&nodes, 7);
    assert_valid_tree_node_item4(&vec![0], 1, 2, &vec![4], false, &nodes);
    assert_valid_tree_node_item4(&vec![0, 0], 1, 2, &vec![2], true, &nodes);
    assert_valid_tree_node_item4(&vec![0, 1], 1, 2, &vec![6], true, &nodes);
    assert_valid_tree_node_item4(&vec![0, 0, 0], 1, 0, &vec![1], true, &nodes);
    assert_valid_tree_node_item4(&vec![0, 0, 1], 1, 0, &vec![3], true, &nodes);
    assert_valid_tree_node_item4(&vec![0, 1, 0], 1, 0, &vec![5], true, &nodes);
    assert_valid_tree_node_item4(&vec![0, 1, 1], 1, 0, &vec![7], true, &nodes);

    println!("-------------------put before------------------------");
    nodes.iterator();
    println!("nodes.merkleroot: {:?}", nodes.merkleroot());
    println!("{:?}", nodes.nodes_map);

    let mut subnodes = tree.put_clone(Item4 { key: 8, value: 3 }, &mut nodes);

    println!("-------------------put after------------------------");
    println!("nodes.merkleroot: {:?}", nodes.merkleroot());
    println!("subnodes.merkleroot: {:?}", subnodes.merkleroot());

    nodes.iterator();

    println!("-------------------subnodes put------------------------");
    tree.put(Item4 { key: 8, value: 3 }, &mut subnodes);
    println!("subnodes.merkleroot: {:?}", subnodes.merkleroot());
    println!("subnodes.nodemap: {:?}", subnodes.nodes_map);
}

#[test]
pub fn test_sgxdb_remove_clone_from_root() {
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

    tree.put(Item4 { key: 1, value: 0 }, &mut nodes);
    tree.put(Item4 { key: 2, value: 1 }, &mut nodes);
    tree.put(Item4 { key: 3, value: 2 }, &mut nodes);
    tree.put(Item4 { key: 4, value: 2 }, &mut nodes);
    tree.put(Item4 { key: 5, value: 2 }, &mut nodes);
    tree.put(Item4 { key: 6, value: 2 }, &mut nodes);
    tree.put(Item4 { key: 7, value: 2 }, &mut nodes);
    assert_valid_tree(&nodes, 7);
    assert_valid_tree_node_item4(&vec![0], 1, 2, &vec![4], false, &nodes);
    assert_valid_tree_node_item4(&vec![0, 0], 1, 2, &vec![2], true, &nodes);
    assert_valid_tree_node_item4(&vec![0, 1], 1, 2, &vec![6], true, &nodes);
    assert_valid_tree_node_item4(&vec![0, 0, 0], 1, 0, &vec![1], true, &nodes);
    assert_valid_tree_node_item4(&vec![0, 0, 1], 1, 0, &vec![3], true, &nodes);
    assert_valid_tree_node_item4(&vec![0, 1, 0], 1, 0, &vec![5], true, &nodes);
    assert_valid_tree_node_item4(&vec![0, 1, 1], 1, 0, &vec![7], true, &nodes);

    println!("-------------------remove before------------------------");
    nodes.iterator();
    println!("nodes.merkleroot: {:?}", nodes.merkleroot());
    println!("{:?}", nodes.nodes_map);

    let mut subnodes = tree.remove_clone(Item4 { key: 7, value: 0 }, &mut nodes);

    println!("-------------------remove after------------------------");
    println!("nodes.merkleroot: {:?}", nodes.merkleroot());
    println!("subnodes.merkleroot: {:?}", subnodes.merkleroot());
    nodes.iterator();

    println!("-------------------subnodes remove------------------------");
    tree.remove(Item4 { key: 7, value: 0 }, &mut subnodes);
    println!("subnodes.merkleroot: {:?}", subnodes.merkleroot());
    println!("subnodes.nodemap: {:?}", subnodes.nodes_map);
}

#[test]
fn test_verify_subnodes_of_get() {
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

    for j in 1..7 {
        let item = Item4 { key: j, value: 2 };
        let (node_id, index, found, mut subnodes) =
            tree.clone_search_subnode_from_root(0, &item, &mut nodes);

        print_subnodes_nodemap_existed(&mut subnodes);

        let (node_id, index, found) = tree.search_recursively(0, &item, &mut subnodes);
        assert_eq!(true, found);
    }
}

#[test]
fn test_batch_get_clone_subnode_from_root() {
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

    for j in 1..500 {
        let item = Item4 { key: j, value: 2 };
        let (node_id, index, found, mut subnodes) =
            tree.clone_search_subnode_from_root(0, &item, &mut nodes);

        print_subnodes_nodemap_existed(&mut subnodes);

        let (node_id, index, found) = tree.search_recursively(0, &item, &mut subnodes);
        assert_eq!(true, found)
    }
}

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

        print_subnodes_nodemap(&mut subnodes);

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
        print_subnodes_nodemap_existed(&mut subnodes);

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
