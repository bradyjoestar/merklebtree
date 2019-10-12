use crate::merklebtree::{MerkleBTree, Nodes};
use std::fmt::Debug;

pub enum position {
    begin,
    between,
    end,
}

pub struct btree_iterator<'a, T>
where
    T: PartialEq + PartialOrd + Ord + Clone + Debug,
{
    pub mbtree: &'a mut MerkleBTree,

    pub nodes: &'a mut Nodes<T>,

    pub position: position,

    pub node_id: i32,

    pub content: Option<T>,
}

pub fn new_btree_iterator<'a, T>(
    nodes: &'a mut Nodes<T>,
    position: position,
    mbtree: &'a mut MerkleBTree,
) -> btree_iterator<'a, T>
where
    T: PartialEq + PartialOrd + Ord + Clone + Debug,
{
    btree_iterator {
        mbtree,
        nodes,
        position,
        node_id: 0,
        content: None,
    }
}

// Next moves the iterator to the next element and returns true if there was a next element in the container.
// If Next() returns true, then next element's key and value can be retrieved by Key() and Value().
// If Next() was called for the first time, then it will point the iterator to the first element if it exists.
// Modifies the state of the iterator.
pub fn next<T>(mut btree_iterator: &mut btree_iterator<T>) -> bool
where
    T: PartialEq + PartialOrd + Ord + Clone + Debug,
{
    match btree_iterator.position {
        position::end => {
            end(btree_iterator);
            return false;
        }
        position::begin => {
            let left_node_id = btree_iterator
                .mbtree
                .left(btree_iterator.nodes.root_id, btree_iterator.nodes);
            if left_node_id == -1 {
                end(btree_iterator);
                return false;
            }

            btree_iterator.node_id = left_node_id;
            btree_iterator.content =
                Some(get_content_in_node(left_node_id, &mut btree_iterator, 0));
            between(&mut btree_iterator);
            return true;
        }
        position::between => {
            println!("{}", btree_iterator.node_id);
            let node = btree_iterator
                .nodes
                .nodes_map
                .get(&btree_iterator.node_id)
                .unwrap();
            let entry = btree_iterator.content.clone().unwrap();
            let index = node.content.binary_search(&entry).unwrap();

            if index + 1 < node.children_id.len() {
                btree_iterator.node_id = *node.children_id.get(index + 1).unwrap();
                // Try to go down to the child left of the current node
                let left_node_id = btree_iterator
                    .mbtree
                    .left(btree_iterator.node_id, btree_iterator.nodes);
                btree_iterator.node_id = left_node_id;
                btree_iterator.content =
                    Some(get_content_in_node(left_node_id, &mut btree_iterator, 0));

                between(&mut btree_iterator);
                return true;
            }
            if index + 1 < node.content.len() {
                btree_iterator.content = Some(get_content_in_node(
                    node.node_id,
                    &mut btree_iterator,
                    (index + 1) as i32,
                ));

                between(&mut btree_iterator);
                return true;
            }

            let mut find_node = &node.clone();
            // Reached leaf node and there are no contents to the right of the current entry, so go up to the parent
            loop {
                if find_node.parent_id == -1 {
                    break;
                }
                btree_iterator.node_id = node.parent_id;
                find_node = btree_iterator
                    .nodes
                    .nodes_map
                    .get(&btree_iterator.node_id)
                    .unwrap();
                match find_node.content.binary_search(&entry) {
                    Ok(e) => {
                        let current_node = btree_iterator
                            .nodes
                            .nodes_map
                            .get(&btree_iterator.node_id)
                            .unwrap();
                        if e < current_node.content.len() {
                            btree_iterator.content = Some(get_content_in_node(
                                btree_iterator.node_id,
                                &mut btree_iterator,
                                e as i32,
                            ));
                            between(&mut btree_iterator);
                            return true;
                        }
                    }
                    Err(e) => {
                        // Check that there is a next entry position in current node
                        let current_node = btree_iterator
                            .nodes
                            .nodes_map
                            .get(&btree_iterator.node_id)
                            .unwrap();
                        if e < current_node.content.len() {
                            btree_iterator.content = Some(get_content_in_node(
                                btree_iterator.node_id,
                                &mut btree_iterator,
                                e as i32,
                            ));
                            between(&mut btree_iterator);
                            return true;
                        }
                    }
                }
            }
        }
    }

    between(btree_iterator);
    return false;
}

pub fn get_content_in_node<T>(
    node_id: i32,
    mut btree_iterator: &mut btree_iterator<T>,
    content_index: i32,
) -> T
where
    T: PartialEq + PartialOrd + Ord + Clone + Debug,
{
    let node = btree_iterator.nodes.nodes_map.remove(&node_id).unwrap();
    let mut node_clone = node.clone();
    btree_iterator.nodes.nodes_map.insert(node_id, node);

    node_clone.content.remove(content_index as usize)
}

// End moves the iterator past the last element (one-past-the-end).
// Call Prev() to fetch the last element if any.
fn end<T>(btree_iterator: &mut btree_iterator<T>)
where
    T: PartialEq + PartialOrd + Ord + Clone + Debug,
{
    btree_iterator.node_id = -1;
    btree_iterator.position = position::end;
    btree_iterator.content = None;
}

fn between<T>(btree_iterator: &mut btree_iterator<T>)
where
    T: PartialEq + PartialOrd + Ord + Clone + Debug,
{
    btree_iterator.position = position::between;
}

pub fn item<T>(mut btree_iterator: &mut btree_iterator<T>) -> T
where
    T: PartialEq + PartialOrd + Ord + Clone + Debug,
{
    return btree_iterator.content.clone().unwrap();
}

pub fn contents<T>(mut btree_iterator: &mut btree_iterator<T>) -> Vec<T>
where
    T: PartialEq + PartialOrd + Ord + Clone + Debug,
{
    let mut content = Vec::new();
    loop {
        if next(btree_iterator) {
            content.push(item(btree_iterator))
        } else {
            break;
        }
    }
    content
}
