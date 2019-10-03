use std::fmt::Debug;
use std::io::BufRead;

#[derive(Clone, Debug)]
pub struct Node<T>
where
    T: PartialEq + PartialOrd + Ord + Clone + Debug,
{
    pub root_flag: bool, //whether is root node
    pub parent: Option<Box<Node<T>>>,
    pub children: Vec<Box<Node<T>>>,
    pub content: Vec<T>,
}

impl<T> Node<T>
where
    T: PartialEq + PartialOrd + Ord + Clone + Debug,
{
    pub fn new_empty() -> Self {
        Node {
            root_flag: false,
            parent: None,
            children: vec![],
            content: vec![],
        }
    }

    pub fn new_node(value: T) -> Self {
        Node {
            root_flag: false,
            parent: None,
            children: vec![],
            content: vec![value],
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

pub fn insert<T>(node: &mut Box<Node<T>>, value: T, order: u32) -> bool
where
    T: PartialEq + PartialOrd + Ord + Clone + Debug,
{
    if is_leaf(node) {
        insert_into_leaf(node, value, order)
    } else {
        insert_into_internal(node, value, order)
    }
}

pub fn insert_into_leaf<T>(node: &mut Box<Node<T>>, value: T, order: u32) -> bool
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
            split_node(node, order);
        }
    }
    true
}

pub fn insert_into_internal<T>(node: &mut Box<Node<T>>, value: T, order: u32) -> bool
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

pub fn split_node<T>(node: &mut Box<Node<T>>, order: u32)
where
    T: PartialEq + PartialOrd + Ord + Clone + Debug,
{
    if !(node.content.len() > (order - 1) as usize) {
        return;
    } else {
        if node.root_flag {
            split_root(node, order)
        } else {
            split_not_root(node, order)
        }
        println!("should split node");
    }
}

pub fn split_root<T>(node: &mut Box<Node<T>>, order: u32)
where
    T: PartialEq + PartialOrd + Ord + Clone + Debug,
{

}

pub fn split_not_root<T>(node: &mut Box<Node<T>>, order: u32)
where
    T: PartialEq + PartialOrd + Ord + Clone + Debug,
{

}
