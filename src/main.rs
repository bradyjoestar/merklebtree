extern crate merklebtree;
use merklebtree::merklebtree::{MerkleBTree, Nodes};

mod bean;
use crate::bean::Item;
use crate::bean::Item2;
use merklebtree::node::Node;
use std::cmp::Ordering;
use std::collections::HashMap;
use std::fmt::Debug;

fn main() {
    println!("Hello, world!");

    test1();
}

fn test1() {
    let mut nodes = Nodes {
        nodes_map: Default::default(),
        size: 0,
        root_id: 0,
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
}

fn test2() {
    let mut nodes = Nodes {
        nodes_map: Default::default(),
        size: 0,
        root_id: 0,
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
