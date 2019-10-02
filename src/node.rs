#[derive(Clone, Debug)]
pub struct Node<T>
where
    T: PartialEq + PartialOrd + Ord,
{
    parent: Option<Box<Node<T>>>,
    children: Vec<Box<Node<T>>>,
    content: Vec<T>,
}

impl<T> Node<T>
where
    T: PartialEq + PartialOrd + Ord,
{
    pub fn new_node(value: T) -> Self {
        Node {
            parent: None,
            children: vec![],
            content: vec![value],
        }
    }
    //
    pub fn put(&mut self, value: T) -> bool {
        let content_slice = self.content.as_slice();
        match content_slice.binary_search(&value) {
            Ok(t) => println!("found,not insert"),
            Err(e) => {
                println!("not found,insert");
                self.content.insert(e, value);
            }
        }
        true
    }

    pub fn get_content(&self) -> Option<&Vec<T>> {
        Some(&(self.content))
    }
}

pub fn test() {
    println!("this is a test");
}
