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
    pub nodes_map: HashMap<u32, Node<T>>,
    pub number: u32,
    pub root_id: u32,
}

#[derive(Clone, Debug)]
pub struct MerkleBTree {
    pub empty: bool,
    pub rootid: u32,
    pub m: u32, // order (maximum number of children)
    pub id: u32,
}

impl MerkleBTree {
    pub fn new_empty<T>(order: u32, nodes: &mut Nodes<T>) -> Self
    where
        T: PartialEq + PartialOrd + Ord + Clone + Debug,
    {
        let mut tree = MerkleBTree {
            empty: true,
            rootid: 0,
            m: order,
            id: 0,
        };

        nodes.nodes_map.insert(0, Node::new_empty(0));
        nodes.nodes_map.get_mut(&0).unwrap().root_flag = true;

        tree.id = tree.id + 1;
        tree
    }

    pub fn new_with<T>(order: u32, value: T, nodes: &mut Nodes<T>) -> Self
    where
        T: PartialEq + PartialOrd + Ord + Clone + Debug,
    {
        println!("{:?}", value);
        let mut tree = MerkleBTree {
            empty: false,
            rootid: 0,
            m: order,
            id: 0,
        };
        nodes.nodes_map.insert(0, Node::new_node(value, 0));
        nodes.nodes_map.get_mut(&0).unwrap().root_flag = true;
        tree.id = tree.id + 1;
        tree
    }

    pub fn put<T>(&mut self, value: T, nodes: &mut Nodes<T>) -> ()
    where
        T: PartialEq + PartialOrd + Ord + Clone + Debug,
    {
        if self.empty {
            self.rootid = self.id;
            self.empty = false;
            nodes.nodes_map.insert(0, Node::new_node(value, self.id));
            nodes.nodes_map.get_mut(&0).unwrap().root_flag = true;
            self.id = self.id + 1;
        } else {
            let a = self.rootid;
            node::insert(a, value, self.m, self.id, nodes);
        }
    }
}
