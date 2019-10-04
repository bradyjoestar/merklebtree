use crate::merklebtree::Nodes;
use core::borrow::BorrowMut;
use std::fmt::Debug;
use std::io::BufRead;

#[derive(Clone, Debug)]
pub struct Node<T>
where
    T: PartialEq + PartialOrd + Ord + Clone + Debug,
{
    pub root_flag: bool, //whether is root node
    pub parent_id: i32,
    pub children_id: Vec<i32>,
    pub content: Vec<T>,
    pub node_id: i32,
}

impl<T> Node<T>
where
    T: PartialEq + PartialOrd + Ord + Clone + Debug,
{
    pub fn new_empty(id: i32) -> Self {
        Node {
            root_flag: false,
            parent_id: -1,
            children_id: vec![],
            content: vec![],
            node_id: id,
        }
    }

    pub fn new_node(value: T, id: i32) -> Self {
        Node {
            root_flag: false,
            parent_id: -1,
            children_id: vec![],
            content: vec![value],
            node_id: id,
        }
    }

    pub fn get_content(&self) -> Option<&Vec<T>> {
        Some(&(self.content))
    }
}

pub fn is_leaf<T>(nodeid: i32, nodes: &mut Nodes<T>) -> bool
where
    T: PartialEq + PartialOrd + Ord + Clone + Debug,
{
    let node = nodes.nodes_map.get_mut(&nodeid).unwrap();
    if node.children_id.len() == 0 {
        true
    } else {
        false
    }
}

pub fn insert<T>(nodeid: i32, value: T, order: u32, id: i32, nodes: &mut Nodes<T>) -> bool
where
    T: PartialEq + PartialOrd + Ord + Clone + Debug,
{
    if is_leaf(nodeid, nodes) {
        insert_into_leaf(nodeid, value, order, id, nodes)
    } else {
        insert_into_internal(nodeid, value, order, id, nodes)
    }
}

pub fn insert_into_leaf<T>(nodeid: i32, value: T, order: u32, id: i32, nodes: &mut Nodes<T>) -> bool
where
    T: PartialEq + PartialOrd + Ord + Clone + Debug,
{
    let node = nodes.nodes_map.get_mut(&nodeid).unwrap();
    let content_slice = node.content.as_slice();
    match content_slice.binary_search(&value) {
        Ok(t) => {
            node.content.remove(t);
            node.content.insert(t, value);
        }
        Err(e) => {
            node.content.insert(e, value);
            split_node(nodeid, order, id, nodes);
        }
    }
    true
}

pub fn insert_into_internal<T>(
    nodeid: i32,
    value: T,
    order: u32,
    id: i32,
    nodes: &mut Nodes<T>,
) -> bool
where
    T: PartialEq + PartialOrd + Ord + Clone + Debug,
{
    let node = nodes.nodes_map.get_mut(&nodeid).unwrap();
    let content_slice = node.content.as_slice();
    match content_slice.binary_search(&value) {
        Ok(t) => {}
        Err(e) => {
            node.content.insert(e, value);
        }
    }
    true
}

pub fn split_node<T>(nodeid: i32, order: u32, id: i32, nodes: &mut Nodes<T>)
where
    T: PartialEq + PartialOrd + Ord + Clone + Debug,
{
    let node = nodes.nodes_map.get_mut(&nodeid).unwrap();
    if !(node.content.len() > (order - 1) as usize) {
        return;
    } else {
        if node.root_flag {
            split_root(nodeid, order, id, nodes)
        } else {
            split_not_root(nodeid, order, id, nodes)
        }
    }
}

pub fn split_root<T>(rootid: i32, order: u32, id: i32, nodes: &mut Nodes<T>)
where
    T: PartialEq + PartialOrd + Ord + Clone + Debug,
{
    let middle = middle(order);

    println!("should split");
    let mut left_node = Node::new_empty(id);
    let mut right_node = Node::new_empty(id + 1);
    let mut root_node = Node::new_empty(id + 2);

    root_node.root_flag = true;
    let node = nodes.nodes_map.get_mut(&rootid).unwrap();

    root_node.content = node.content.split_off(middle as usize);
    right_node.content = root_node.content.split_off(1);
    left_node.content = node.content.clone();

    nodes.nodes_map.insert(id, left_node);
    nodes.nodes_map.insert(id + 1, right_node);
    nodes.nodes_map.insert(id + 2, root_node);

    nodes.root_id = id + 2;

    println!("left node:{:?}", nodes.nodes_map.get(&id).unwrap().content);
    println!(
        "right node: {:?}",
        nodes.nodes_map.get(&(id + 1)).unwrap().content
    );
    println!(
        "root node:{:?}",
        nodes.nodes_map.get(&(id + 2)).unwrap().content
    );
}

pub fn split_not_root<T>(nodeid: i32, order: u32, id: i32, nodes: &mut Nodes<T>)
where
    T: PartialEq + PartialOrd + Ord + Clone + Debug,
{

}

pub fn middle(order: u32) -> u32 {
    return (order - 1) / 2;
}

pub fn set_parent() -> bool {
    true
}
