use std::fmt::Debug;

#[derive(Clone, Debug)]
pub struct Node<T>
where
    T: PartialEq + PartialOrd + Ord + Clone + Debug,
{
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
            parent: None,
            children: vec![],
            content: vec![],
        }
    }

    pub fn new_node(value: T) -> Self {
        Node {
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

pub fn insert<T>(node: &mut Box<Node<T>>, value: T) -> bool
where
    T: PartialEq + PartialOrd + Ord + Clone + Debug,
{
    if is_leaf(node) {
        insert_into_leaf(node, value)
    } else {
        insert_into_internal(node, value)
    }
}

pub fn insert_into_leaf<T>(node: &mut Box<Node<T>>, value: T) -> bool
where
    T: PartialEq + PartialOrd + Ord + Clone + Debug,
{
    println!("insert to leaf");
    let content_slice = node.content.as_slice();
    match content_slice.binary_search(&value) {
        Ok(t) => {
            node.content.remove(t);
            node.content.insert(t,value);
            println!("found,not insert");
        }
        Err(e) => {
            println!("not found,insert");
            node.content.insert(e, value);
        }
    }
    true
}

pub fn insert_into_internal<T>(node: &mut Box<Node<T>>, value: T) -> bool
where
    T: PartialEq + PartialOrd + Ord + Clone + Debug,
{
    println!("insert to internal");
    let content_slice = node.content.as_slice();
    match content_slice.binary_search(&value) {
        Ok(t) => {
            println!("found,not insert");
        }
        Err(e) => {
            println!("not found,insert");
            node.content.insert(e, value);
        }
    }
    true
}
