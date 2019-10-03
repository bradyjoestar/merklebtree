#[derive(Clone, Debug)]
pub struct Node<T>
where
    T: PartialEq + PartialOrd + Ord + Clone,
{
    pub parent: Option<Box<Node<T>>>,
    pub children: Vec<Box<Node<T>>>,
    pub content: Vec<T>,
}

impl<T> Node<T>
where
    T: PartialEq + PartialOrd + Ord + Clone,
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
    T: PartialEq + PartialOrd + Ord + Clone,
{
    if node.children.len() == 0 {
        true
    } else {
        false
    }
}

pub fn insert<T>(node: &mut Box<Node<T>>, value: T) -> bool
where
    T: PartialEq + PartialOrd + Ord + Clone,
{
    if is_leaf(node) {
        println!("is node");
    }
    let content_slice = node.content.as_slice();
    match content_slice.binary_search(&value) {
        Ok(t) => println!("found,not insert"),
        Err(e) => {
            println!("not found,insert");
            node.content.insert(e, value);
        }
    }
    true
}

pub fn test() {
    println!("this is a test");
}
