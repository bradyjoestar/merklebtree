use crate::node;
use crate::node::Node;
use crate::node::{calculate_hash, is_leaf};
use crate::traits::CalculateHash;
use core::borrow::{Borrow, BorrowMut};
use std::collections::HashMap;
use std::fmt::Debug;

use serde::{Deserialize, Serialize};

#[derive(Clone, Debug)]
pub struct Nodes<T>
where
    T: PartialEq + PartialOrd + Ord + Clone + Debug + CalculateHash,
{
    pub nodes_map: HashMap<i32, Node<T>>,
    pub size: u32, //the number of nodes
    pub root_id: i32,
    pub content_size: i32, //the number of content_item
    pub next_id: i32,      //generate the index of new node
    pub m: u32,            // order (maximum number of children)
}

#[derive(Clone, Deserialize, Serialize, Debug)]
pub struct NodesSerialize {
    pub size: u32, //the number of nodes
    pub root_id: i32,
    pub content_size: i32, //the number of content_item
    pub next_id: i32,      //generate the index of new node
    pub m: u32,            // order (maximum number of children)
}

impl<T> Nodes<T>
where
    T: PartialEq + PartialOrd + Ord + Clone + Debug + CalculateHash,
{
    pub fn iterator(&self) {
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
                println!("node.hash:{}", node.hash);
            }
            println!("****************************************************");
        }
    }

    pub fn merkleroot(&self) -> String {
        let node = self.nodes_map.get(&0).unwrap();
        node.hash.clone()
    }

    pub fn recalculate_merkleroot(&mut self) -> Self {
        let mut a = Vec::new();

        let mut nodes_clone = self.clone();
        for (i, j) in self.nodes_map.iter() {
            let mut node = nodes_clone.nodes_map.remove(i).unwrap();
            node.hash = String::new();
            nodes_clone.nodes_map.insert(node.node_id, node);
        }

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

        for i in 0..a.len() {
            println!("****************************************************");
            let sub_vec = a.get(a.len() - 1 - i).unwrap();
            for j in 0..sub_vec.len() {
                let node = *sub_vec.get(j).unwrap();
                let node_in_clone = nodes_clone.nodes_map.get(&node.node_id).unwrap();
                calculate_hash(node.node_id, &mut nodes_clone);
            }
            println!("****************************************************");
        }
        nodes_clone
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
        T: PartialEq + PartialOrd + Ord + Clone + Debug + CalculateHash,
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
        T: PartialEq + PartialOrd + Ord + Clone + Debug + CalculateHash,
    {
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
        nodes.content_size = nodes.content_size + 1;
        tree
    }

    pub fn put<T>(&mut self, value: T, nodes: &mut Nodes<T>) -> ()
    where
        T: PartialEq + PartialOrd + Ord + Clone + Debug + CalculateHash,
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
            nodes.content_size = nodes.content_size + 1;
        } else {
            let a = self.rootid;
            let mut pre_not_existed = node::insert(a, value, nodes.m, nodes);
            if pre_not_existed {
                nodes.content_size = nodes.content_size + 1;
            }
        }
    }

    pub fn remove<T>(&mut self, value: T, nodes: &mut Nodes<T>) -> ()
    where
        T: PartialEq + PartialOrd + Ord + Clone + Debug + CalculateHash,
    {
        let (search_node_id, index, found) = self.search_recursively(nodes.root_id, &value, nodes);

        if found {
            node::delete(search_node_id, index, nodes);
            nodes.content_size = nodes.content_size - 1;
        }
    }
    pub fn height<T>(&self, nodes: &Nodes<T>) -> i32
    where
        T: PartialEq + PartialOrd + Ord + Clone + Debug + CalculateHash,
    {
        let mut height = 1;
        let mut node_id = nodes.root_id;
        loop {
            if nodes.content_size == 0 {
                return 0;
            } else {
                let node = nodes.nodes_map.get(&node_id).unwrap();
                if node.children_id.len() != 0 {
                    height = height + 1;
                    node_id = *node.children_id.get(0).unwrap();
                } else {
                    return height;
                }
            }
        }
    }

    pub fn get<T>(&mut self, value: T, nodes: &mut Nodes<T>) -> (T, bool)
    where
        T: PartialEq + PartialOrd + Ord + Clone + Debug + CalculateHash,
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
        T: PartialEq + PartialOrd + Ord + Clone + Debug + CalculateHash,
    {
        if nodes.size == 0 {
            return (-1, -1, false);
        }
        loop {
            let node = nodes.nodes_map.get_mut(&start_node_id).unwrap();
            let content_slice = node.content.as_slice();
            match content_slice.binary_search(&value) {
                Ok(t) => {
                    return (node.node_id, t as i32, true);
                }
                Err(e) => {
                    if node.children_id.len() == 0 {
                        return (-1, -1, false);
                    }
                    start_node_id = *node.children_id.get(e).unwrap();
                }
            }
        }
    }

    pub fn left<T>(&self, mut node_id: i32, nodes: &Nodes<T>) -> i32
    where
        T: PartialEq + PartialOrd + Ord + Clone + Debug + CalculateHash,
    {
        if nodes.content_size == 0 {
            return -1;
        }
        let mut node = nodes.nodes_map.get(&node_id).unwrap();
        loop {
            if node.children_id.len() == 0 {
                return node_id;
            } else {
                node_id = *node.children_id.get(0).unwrap();
                node = nodes.nodes_map.get(&node_id).unwrap();
            }
        }
    }

    pub fn leftItem<T>(&self, mut node_id: i32, nodes: &mut Nodes<T>) -> Option<T>
    where
        T: PartialEq + PartialOrd + Ord + Clone + Debug + CalculateHash,
    {
        let node_id = self.left(node_id, nodes);
        if node_id == -1 {
            return None;
        } else {
            let mut node = nodes.nodes_map.remove(&node_id).unwrap();
            let mut node_clone = node.clone();
            nodes.nodes_map.insert(node_id, node);
            return Some(node_clone.content.remove(0));
        }
    }

    pub fn right<T>(&self, mut node_id: i32, nodes: &Nodes<T>) -> i32
    where
        T: PartialEq + PartialOrd + Ord + Clone + Debug + CalculateHash,
    {
        if nodes.content_size == 0 {
            return -1;
        }
        let mut node = nodes.nodes_map.get(&node_id).unwrap();
        loop {
            if node.children_id.len() == 0 {
                return node_id;
            } else {
                node_id = *node.children_id.last().unwrap();
                node = nodes.nodes_map.get(&node_id).unwrap();
            }
        }
    }

    pub fn rightItem<T>(&self, mut node_id: i32, nodes: &mut Nodes<T>) -> Option<T>
    where
        T: PartialEq + PartialOrd + Ord + Clone + Debug + CalculateHash,
    {
        let node_id = self.right(node_id, nodes);
        if node_id == -1 {
            return None;
        } else {
            let mut node = nodes.nodes_map.remove(&node_id).unwrap();
            let mut node_clone = node.clone();
            nodes.nodes_map.insert(node_id, node);
            Some(node_clone.content.pop().unwrap())
        }
    }
}
