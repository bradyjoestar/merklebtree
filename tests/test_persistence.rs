mod utils;
use merklebtree::merklebtree::{MerkleBTree, Nodes, NodesSerialize};
use merklebtree::node::{Node, NodeSer};
use merklebtree::traits::CalculateHash;
use serde;
use serde_json;
use std::collections::HashMap;
use std::fmt::Debug;
use std::fs;
use std::fs::{File, OpenOptions};
use std::io::{BufRead, BufReader, Write};
use utils::*;

#[test]
fn test_btree_persistence_1() {
    let mut nodes_map: HashMap<i32, Node<Item2>> = HashMap::new();
    let mut nodes = Nodes {
        nodes_map,
        size: 0,
        root_id: 0,
        content_size: 0,
        next_id: 0,
        m: 0,
    };
    let mut tree = MerkleBTree::new_empty(3, &mut nodes);

    tree.put(Item2 { key: 1, value: 0 }, &mut nodes);
    tree.put(Item2 { key: 2, value: 1 }, &mut nodes);
    tree.put(Item2 { key: 3, value: 2 }, &mut nodes);
    tree.put(Item2 { key: 4, value: 2 }, &mut nodes);
    tree.put(Item2 { key: 5, value: 2 }, &mut nodes);
    tree.put(Item2 { key: 6, value: 2 }, &mut nodes);
    tree.put(Item2 { key: 7, value: 2 }, &mut nodes);
    assertValidTree(&nodes, 7);
    assertValidTreeNode(&vec![0], 1, 2, &vec![4], false, &nodes);
    assertValidTreeNode(&vec![0, 0], 1, 2, &vec![2], true, &nodes);
    assertValidTreeNode(&vec![0, 1], 1, 2, &vec![6], true, &nodes);
    assertValidTreeNode(&vec![0, 0, 0], 1, 0, &vec![1], true, &nodes);
    assertValidTreeNode(&vec![0, 0, 1], 1, 0, &vec![3], true, &nodes);
    assertValidTreeNode(&vec![0, 1, 0], 1, 0, &vec![5], true, &nodes);
    assertValidTreeNode(&vec![0, 1, 1], 1, 0, &vec![7], true, &nodes);

    let nodes_serialize = NodesSerialize {
        size: nodes.size,
        root_id: nodes.root_id,
        content_size: nodes.content_size,
        next_id: nodes.next_id,
        m: nodes.m,
    };

    let serialized = serde_json::to_string(&nodes_serialize).unwrap();
    write_msg(serialized);
    save_nodes(&mut nodes);
    let nodes_load = load_nodes();

    assertValidTree(&nodes, 7);
    assertValidTreeNode(&vec![0], 1, 2, &vec![4], false, &nodes_load);
    assertValidTreeNode(&vec![0, 0], 1, 2, &vec![2], true, &nodes_load);
    assertValidTreeNode(&vec![0, 1], 1, 2, &vec![6], true, &nodes_load);
    assertValidTreeNode(&vec![0, 0, 0], 1, 0, &vec![1], true, &nodes_load);
    assertValidTreeNode(&vec![0, 0, 1], 1, 0, &vec![3], true, &nodes_load);
    assertValidTreeNode(&vec![0, 1, 0], 1, 0, &vec![5], true, &nodes_load);
    assertValidTreeNode(&vec![0, 1, 1], 1, 0, &vec![7], true, &nodes_load);

    remove_file();
}

pub fn write_msg(message: String) {
    let mut f = OpenOptions::new()
        .create(true)
        .append(true)
        .open("foo.txt")
        .unwrap();

    f.write_all(message.as_bytes());
    f.write_all(String::from("\n").as_bytes());
}

pub fn remove_file() {
    fs::remove_file("foo.txt").unwrap();
}

pub fn save_nodes(nodes: &mut Nodes<Item2>) -> () {
    let mut vec = Vec::new();

    let mut looptime = 0;

    'outer: loop {
        if vec.len() == 0 {
            let mut b: Vec<&Node<Item2>> = Vec::new();
            let node = nodes.nodes_map.get(&0).unwrap();
            b.push(node);
            vec.push(b);
            looptime = looptime + 1;
        } else {
            let pre_vec = vec.remove(looptime - 1);
            let len = pre_vec.len();
            let mut b: Vec<&Node<Item2>> = Vec::new();
            for i in 0..len {
                let node = pre_vec.get(i).unwrap();
                if node.children_id.len() == 0 {
                    vec.insert(looptime - 1, pre_vec);
                    break 'outer;
                }

                for i in 0..node.children_id.len() {
                    let node_id = node.children_id.get(i).unwrap();
                    let node = nodes.nodes_map.get(node_id).unwrap();
                    b.push(node);
                }
            }
            vec.insert(looptime - 1, pre_vec);
            vec.push(b);
            looptime = looptime + 1;
        }
    }

    for i in 0..vec.len() {
        let mut sub_vec = vec.remove(0);
        for j in 0..sub_vec.len() {
            let node = sub_vec.remove(0);
            //                if node.content.len() == 0 {
            //                    panic!("nil node");
            //                }
            let mut nodeser = NodeSer {
                root_flag: node.root_flag,
                parent_id: node.parent_id,
                children_id: node.children_id.clone(),
                content: vec![],
                node_id: node.node_id,
                hash: node.hash.clone(),
            };
            for m in node.content.iter() {
                let serialized_content = serde_json::to_string(m).unwrap();
                nodeser.content.push(serialized_content);
            }
            let serialized_node = serde_json::to_string(&nodeser).unwrap();
            write_msg(serialized_node);
        }
    }
}

pub fn load_nodes() -> Nodes<Item2> {
    let file = File::open("foo.txt").unwrap();

    let mut fin = BufReader::new(file);
    let mut deserialized: NodesSerialize;
    let mut nodes: Nodes<Item2> = Nodes {
        nodes_map: Default::default(),
        size: 0,
        root_id: 0,
        content_size: 0,
        next_id: 0,
        m: 0,
    };
    let mut count = 0;
    for line in fin.lines() {
        if count == 0 {
            let nodes_serailize = line.unwrap();
            deserialized = serde_json::from_str(&nodes_serailize).unwrap();
            nodes.m = deserialized.m;
            nodes.next_id = deserialized.next_id;
            nodes.content_size = deserialized.content_size;
            nodes.root_id = deserialized.root_id;
        } else {
            let node_deserailize = line.unwrap();
            let deserialized: NodeSer = serde_json::from_str(&node_deserailize).unwrap();
            let mut node = Node {
                root_flag: deserialized.root_flag,
                parent_id: deserialized.parent_id,
                children_id: deserialized.children_id.clone(),
                content: vec![],
                node_id: deserialized.node_id,
                hash: deserialized.hash.clone(),
            };
            for i in deserialized.content.iter() {
                let content_serialize: Item2 = serde_json::from_str(i.as_str()).unwrap();
                node.content.push(content_serialize);
            }
            nodes.nodes_map.insert(node.node_id, node);
        }
        count = count + 1;
    }

    nodes
}
