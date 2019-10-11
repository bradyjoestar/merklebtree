extern crate merklebtree;
use merklebtree::merklebtree::{MerkleBTree, Nodes};

mod bean;
use crate::bean::Item;
use crate::bean::Item2;
use merklebtree::node::Node;
use std::cmp::Ordering;
use std::collections::HashMap;
use std::fmt::Debug;

mod testdebug;

fn main() {
    println!("Hello, world!");

    test1();
    //    testdebug::test_debug();
}

fn test1() {
    let mut nodes = Nodes {
        nodes_map: Default::default(),
        size: 0,
        root_id: 0,
        content_size: 0,
        next_id: 0,
        m: 0,
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

    //    println!("--------------remove the content from leaf node---------------------");
    //    tree.remove(nodes.root_id, Item2 { key: 15 }, &mut nodes);
    //
    //    nodes.iterator();

    println!("--------------remove the content from leaf node---------------------");
    println!("wenbin test");
    tree.remove(nodes.root_id, Item2 { key: 2 }, &mut nodes);

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
    tree.remove(nodes.root_id, Item { key: 2, value: 2 }, &mut nodes);

    nodes.iterator();

    println!("--------------remove the content from leaf node---------------------");
    tree.remove(nodes.root_id, Item { key: 0, value: 1 }, &mut nodes);

    nodes.iterator();
}

fn find_nodeid_by_branch<T>(branch: &Vec<i32>, nodes: &Nodes<T>) -> i32
where
    T: PartialEq + PartialOrd + Ord + Clone + Debug,
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
