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
}

#[derive(Clone, Debug)]
pub struct MerkleBTree {
    pub empty: bool,
    pub rootid: i32,
    pub m: u32, // order (maximum number of children)
}

impl MerkleBTree {
    pub fn new_empty<T>(order: u32, nodes: &mut Nodes<T>) -> Self
    where
        T: PartialEq + PartialOrd + Ord + Clone + Debug,
    {
        let mut tree = MerkleBTree {
            empty: true,
            rootid: nodes.root_id,
            m: order,
        };

        nodes
            .nodes_map
            .insert(nodes.root_id, Node::new_empty(nodes.root_id));
        nodes.nodes_map.get_mut(&(nodes.root_id)).unwrap().root_flag = true;

        nodes.next_id = nodes.next_id + 1;
        tree
    }

    pub fn new_with<T>(order: u32, value: T, nodes: &mut Nodes<T>) -> Self
    where
        T: PartialEq + PartialOrd + Ord + Clone + Debug,
    {
        println!("{:?}", value);
        let mut tree = MerkleBTree {
            empty: false,
            rootid: nodes.root_id,
            m: order,
        };
        nodes
            .nodes_map
            .insert(nodes.root_id, Node::new_node(value, nodes.root_id));
        nodes.nodes_map.get_mut(&(nodes.root_id)).unwrap().root_flag = true;
        nodes.next_id = nodes.next_id + 1;
        tree
    }

    pub fn put<T>(&mut self, value: T, nodes: &mut Nodes<T>) -> ()
    where
        T: PartialEq + PartialOrd + Ord + Clone + Debug,
    {
        if self.empty {
            //insert root node
            self.rootid = nodes.root_id;
            self.empty = false;
            nodes
                .nodes_map
                .insert(nodes.root_id, Node::new_node(value, self.rootid));
            nodes.nodes_map.get_mut(&(nodes.root_id)).unwrap().root_flag = true;
            nodes.next_id = nodes.next_id + 1;
        } else {
            let a = self.rootid;
            node::insert(a, value, self.m, nodes);
        }
    }
}
