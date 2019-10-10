use crate::node;
use crate::node::is_leaf;
use crate::node::Node;
use core::borrow::{Borrow, BorrowMut};
use std::collections::HashMap;
use std::fmt::Debug;

pub struct Nodes<T>
where
    T: PartialEq + PartialOrd + Ord + Clone + Debug,
{
    pub nodes_map: HashMap<i32, Node<T>>,
    pub size: u32, //the number of nodes
    pub root_id: i32,
    pub next_id: i32, //generate the index of new node
    pub m: u32,       // order (maximum number of children)
}

impl<T> Nodes<T>
where
    T: PartialEq + PartialOrd + Ord + Clone + Debug,
{
    pub fn iterator(&self) -> () {
        let mut a = Vec::new();

        let mut looptime = 0;

        'outer: loop {
            if a.len() == 0 {
                let mut b: Vec<&Node<T>> = Vec::new();
                let node = self.nodes_map.get(&0).unwrap();
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
                        let node = self.nodes_map.get(node_id).unwrap();
                        b.push(node);
                    }
                }
                a.insert(looptime - 1, pre_vec);
                a.push(b);
                looptime = looptime + 1;
            }
        }
        println!("println nodes");

        for i in 0..a.len() {
            println!("****************************************************");
            let sub_vec = a.get(i).unwrap();
            for j in 0..sub_vec.len() {
                let node = *sub_vec.get(j).unwrap();
                //                if node.content.len() == 0 {
                //                    panic!("nil node");
                //                }
                println!("node.node_id: {}", node.node_id);
                println!("node.children_id: {:?}", node.children_id);
                println!("node.content: {:?}", node.content);
                println!("node.parent_id: {:?}", node.parent_id);
            }
            println!("****************************************************");
        }
    }
}

#[derive(Clone, Debug)]
pub struct MerkleBTree {
    pub rootid: i32,
    pub m: u32, // order (maximum number of children)
}

impl MerkleBTree {
    pub fn new_empty<T>(order: u32, nodes: &mut Nodes<T>) -> Self
    where
        T: PartialEq + PartialOrd + Ord + Clone + Debug,
    {
        let mut tree = MerkleBTree {
            rootid: -1,
            m: order,
        };
        nodes.m = order;

        tree
    }

    pub fn new_with<T>(order: u32, value: T, nodes: &mut Nodes<T>) -> Self
    where
        T: PartialEq + PartialOrd + Ord + Clone + Debug,
    {
        println!("{:?}", value);
        let mut tree = MerkleBTree {
            rootid: nodes.root_id,
            m: order,
        };
        nodes
            .nodes_map
            .insert(nodes.root_id, Node::new_node(value, nodes.root_id));
        nodes.nodes_map.get_mut(&(nodes.root_id)).unwrap().root_flag = true;
        nodes.next_id = nodes.next_id + 1;
        nodes.size = nodes.size + 1;
        tree
    }

    pub fn put<T>(&mut self, value: T, nodes: &mut Nodes<T>) -> ()
    where
        T: PartialEq + PartialOrd + Ord + Clone + Debug,
    {
        if nodes.size == 0 {
            //insert root node
            self.rootid = nodes.root_id;
            nodes
                .nodes_map
                .insert(nodes.root_id, Node::new_node(value, self.rootid));
            nodes.nodes_map.get_mut(&(nodes.root_id)).unwrap().root_flag = true;
            nodes.next_id = nodes.next_id + 1;
            nodes.size = nodes.size + 1;
        } else {
            let a = self.rootid;
            node::insert(a, value, nodes.m, nodes);
        }
    }

    pub fn remove<T>(&mut self, start_node_id: i32, value: T, nodes: &mut Nodes<T>) -> ()
    where
        T: PartialEq + PartialOrd + Ord + Clone + Debug,
    {
        let (search_node_id, index, found) = self.search_recursively(start_node_id, &value, nodes);
        println!(
            "search_node_id:{},index:{},found:{}",
            search_node_id, index, found
        );
        if found {
            println!("try to remove node");
            node::delete(search_node_id, index, nodes);
        }
    }

    pub fn get<T>(&mut self, value: T, nodes: &mut Nodes<T>) -> (T, bool)
    where
        T: PartialEq + PartialOrd + Ord + Clone + Debug,
    {
        let (node_id, content_index, found) = self.search_recursively(0, &value, nodes);
        if found {
            let mut node = nodes.nodes_map.remove(&node_id).unwrap();
            let value = node.content.remove(content_index as usize);
            let value_copy = value.clone();
            node.content.insert(content_index as usize, value_copy);
            nodes.nodes_map.insert(node_id, node);
            return (value, true);
        }
        (value, false)
    }

    pub fn search_recursively<T>(
        &mut self,
        mut start_node_id: i32,
        value: &T,
        nodes: &mut Nodes<T>,
    ) -> (i32, i32, bool)
    where
        T: PartialEq + PartialOrd + Ord + Clone + Debug,
    {
        if nodes.size == 0 {
            return (-1, -1, false);
        }
        loop {
            let node = nodes.nodes_map.get_mut(&start_node_id).unwrap();
            let content_slice = node.content.as_slice();
            match content_slice.binary_search(&value) {
                Ok(t) => {
                    println!("found");
                    return (node.node_id, t as i32, true);
                }
                Err(e) => {
                    println!("not found");
                    if node.children_id.len() == 0 {
                        return (-1, -1, false);
                    }
                    start_node_id = *node.children_id.get(e).unwrap();
                }
            }
        }
    }
}
