use core::borrow::BorrowMut;
use std::fmt::Debug;
use std::io::BufRead;
use crate::merklebtree::Nodes;

#[derive(Clone, Debug)]
pub struct Node<T>
where
    T: PartialEq + PartialOrd + Ord + Clone + Debug,
{
    pub root_flag: bool, //whether is root node
    pub parent: Option<Box<Node<T>>>,
    pub children: Vec<Box<Node<T>>>,
    pub content: Vec<T>,
    pub id: u32,
}

impl<T> Node<T>
where
    T: PartialEq + PartialOrd + Ord + Clone + Debug,
{
    pub fn new_empty(id: u32) -> Self {
        Node {
            root_flag: false,
            parent: None,
            children: vec![],
            content: vec![],
            id: id,
        }
    }

    pub fn new_node(value: T, id: u32) -> Self {
        Node {
            root_flag: false,
            parent: None,
            children: vec![],
            content: vec![value],
            id: id,
        }
    }

    pub fn get_content(&self) -> Option<&Vec<T>> {
        Some(&(self.content))
    }
}

pub fn is_leaf<T>(node: &Box<Node<T>>) -> bool
where
    T: PartialEq + PartialOrd + Ord + Clone + Debug,
{
    if node.children.len() == 0 {
        true
    } else {
        false
    }
}

pub fn insert<T>(
    node: &mut Box<Node<T>>,
    value: T,
    order: u32,
    id: u32,
    nodes: &mut Nodes<T>,
) -> bool
where
    T: PartialEq + PartialOrd + Ord + Clone + Debug,
{
    if is_leaf(node) {
        insert_into_leaf(node, value, order, id, nodes)
    } else {
        insert_into_internal(node, value, order, id, nodes)
    }
}

pub fn insert_into_leaf<T>(
    node: &mut Box<Node<T>>,
    value: T,
    order: u32,
    id: u32,
    nodes: &mut Nodes<T>,
) -> bool
where
    T: PartialEq + PartialOrd + Ord + Clone + Debug,
{
    let content_slice = node.content.as_slice();
    match content_slice.binary_search(&value) {
        Ok(t) => {
            node.content.remove(t);
            node.content.insert(t, value);
        }
        Err(e) => {
            node.content.insert(e, value);
            split_node(node, order, id, nodes);
        }
    }
    true
}

pub fn insert_into_internal<T>(
    node: &mut Box<Node<T>>,
    value: T,
    order: u32,
    id: u32,
    nodes: &mut Nodes<T>,
) -> bool
where
    T: PartialEq + PartialOrd + Ord + Clone + Debug,
{
    let content_slice = node.content.as_slice();
    match content_slice.binary_search(&value) {
        Ok(t) => {}
        Err(e) => {
            node.content.insert(e, value);
        }
    }
    true
}

pub fn split_node<T>(node: &mut Box<Node<T>>, order: u32, id: u32, nodes: &mut Nodes<T>)
where
    T: PartialEq + PartialOrd + Ord + Clone + Debug,
{
    if !(node.content.len() > (order - 1) as usize) {
        return;
    } else {
        if node.root_flag {
            split_root(node, order, id, nodes)
        } else {
            split_not_root(node, order, id, nodes)
        }
        println!("should split node");
    }
}

pub fn split_root<T>(node: &mut Box<Node<T>>, order: u32, id: u32, nodes: &mut Nodes<T>)
where
    T: PartialEq + PartialOrd + Ord + Clone + Debug,
{
    let middle = middle(order);

    let mut left_node = Node::new_empty(id);
    let mut right_node = Node::new_empty(id + 1);
    let mut root_node = Node::new_empty(id + 2);
    root_node.root_flag = true;

    root_node.content = node.content.split_off(middle as usize);
    right_node.content = root_node.content.split_off(1);
    left_node.content = node.content.clone();

    *node = Box::new(root_node);

    println!("left node:{:?}", left_node.content);
    println!("right node: {:?}", right_node.content);
}

pub fn split_not_root<T>(node: &mut Box<Node<T>>, order: u32, id: u32, nodes: &mut Nodes<T>)
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
