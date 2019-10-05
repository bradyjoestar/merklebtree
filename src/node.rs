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

pub fn is_leaf<T>(nodeid: i32, nodes: &Nodes<T>) -> bool
where
    T: PartialEq + PartialOrd + Ord + Clone + Debug,
{
    let node = nodes.nodes_map.get(&nodeid).unwrap();
    if node.children_id.len() == 0 {
        true
    } else {
        false
    }
}

pub fn insert<T>(insert_id: i32, value: T, order: u32, nodes: &mut Nodes<T>) -> bool
where
    T: PartialEq + PartialOrd + Ord + Clone + Debug,
{
    if is_leaf(insert_id, nodes) {
        insert_into_leaf(insert_id, value, order, nodes)
    } else {
        insert_into_internal(insert_id, value, order, nodes)
    }
}

pub fn insert_into_leaf<T>(insert_id: i32, value: T, order: u32, nodes: &mut Nodes<T>) -> bool
where
    T: PartialEq + PartialOrd + Ord + Clone + Debug,
{
    let node = nodes.nodes_map.get_mut(&insert_id).unwrap();
    let content_slice = node.content.as_slice();
    match content_slice.binary_search(&value) {
        Ok(t) => {
            node.content.remove(t);
            node.content.insert(t, value);
        }
        Err(e) => {
            node.content.insert(e, value);
            split_node(insert_id, order, nodes);
        }
    }
    true
}

pub fn insert_into_internal<T>(insert_id: i32, value: T, order: u32, nodes: &mut Nodes<T>) -> bool
where
    T: PartialEq + PartialOrd + Ord + Clone + Debug,
{
    let node = nodes.nodes_map.get_mut(&insert_id).unwrap();
    let content_slice = node.content.as_slice();
    match content_slice.binary_search(&value) {
        Ok(t) => {
            node.content.remove(t);
            node.content.insert(t, value);
        }
        Err(e) => {
            node.content.insert(e, value);
        }
    }
    true
}

pub fn split_node<T>(split_id: i32, order: u32, nodes: &mut Nodes<T>)
where
    T: PartialEq + PartialOrd + Ord + Clone + Debug,
{
    let node = nodes.nodes_map.get_mut(&split_id).unwrap();
    if !(node.content.len() > (order - 1) as usize) {
        return;
    } else {
        if node.root_flag {
            split_root(split_id, order, nodes)
        } else {
            split_not_root(split_id, order, nodes)
        }
    }
}

pub fn split_root<T>(split_id: i32, order: u32, nodes: &mut Nodes<T>)
where
    T: PartialEq + PartialOrd + Ord + Clone + Debug,
{
    let middle = middle(order);

    println!("should split");
    let mut left_node = Node::new_empty(nodes.next_id);
    let mut right_node = Node::new_empty(nodes.next_id + 1);
    let mut root_node = Node::new_empty(nodes.root_id);

    root_node.root_flag = true;
    let node = nodes.nodes_map.get_mut(&split_id).unwrap();

    root_node.content = node.content.split_off(middle as usize);
    right_node.content = root_node.content.split_off(1);
    left_node.content = node.content.clone();

    // Move children from the node to be split into left and right nodes
    if !(node.children_id.len() == 0) {
        right_node.children_id = node.children_id.split_off((middle + 1) as usize);
        left_node.children_id = node.children_id.clone();
        set_parent(&mut (left_node.children_id), left_node.node_id, nodes);
        set_parent(&mut (right_node.children_id), right_node.node_id, nodes);
    }

    right_node.parent_id = root_node.node_id;
    left_node.parent_id = root_node.node_id;

    nodes.nodes_map.insert(nodes.next_id, left_node);
    nodes.nodes_map.insert(nodes.next_id + 1, right_node);
    nodes.nodes_map.insert(nodes.root_id, root_node);

    nodes.next_id = nodes.next_id + 2;

    println!(
        "left node:{:?}",
        nodes.nodes_map.get(&(nodes.next_id - 2)).unwrap().content
    );
    println!(
        "right node: {:?}",
        nodes.nodes_map.get(&(nodes.next_id - 1)).unwrap().content
    );
    println!(
        "root node:{:?}",
        nodes.nodes_map.get(&(nodes.root_id)).unwrap().content
    );
}

pub fn split_not_root<T>(split_id: i32, order: u32, nodes: &mut Nodes<T>)
where
    T: PartialEq + PartialOrd + Ord + Clone + Debug,
{

}

pub fn middle(order: u32) -> u32 {
    return (order - 1) / 2;
}

pub fn set_parent<T>(childrens: &mut Vec<i32>, parent_id: i32, nodes: &mut Nodes<T>) -> bool
where
    T: PartialEq + PartialOrd + Ord + Clone + Debug,
{
    for i in childrens {
        let node = nodes.nodes_map.get_mut(&i).unwrap();
        node.parent_id = parent_id;
    }
    true
}
