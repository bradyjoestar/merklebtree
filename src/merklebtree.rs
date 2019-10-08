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
    pub fn iterator(&self) -> Vec<Vec<Node<T>>> {
        let mut a = Vec::new();

        let b: Vec<Node<T>> = Vec::new();

        println!("println nodes");

        a.push(b);

        a
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
            rootid: nodes.root_id,
            m: order,
        };

        nodes
            .nodes_map
            .insert(nodes.root_id, Node::new_empty(nodes.root_id));
        nodes.nodes_map.get_mut(&(nodes.root_id)).unwrap().root_flag = true;

        nodes.next_id = nodes.next_id + 1;
        nodes.size = nodes.size + 1;
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
        let (search_node_id, index, found) = self.searchRecursively(start_node_id, value, nodes);
        println!(
            "search_node_id:{},index:{},found:{}",
            search_node_id, index, found
        );
        if found {
            println!("try to remove node");
            node::delete(search_node_id, index, nodes);
        }
    }

    pub fn searchRecursively<T>(
        &mut self,
        mut start_node_id: i32,
        value: T,
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
