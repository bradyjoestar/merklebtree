use crate::merklebtree::{MerkleBTree, Nodes};
use crate::traits::CalculateHash;
use std::fmt::Debug;

use serde::{Deserialize, Serialize};

pub enum position {
    begin,
    between,
    end,
}

pub struct btree_iterator<'a, T>
where
    T: PartialEq + PartialOrd + Ord + Clone + Debug + CalculateHash,
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
    T: PartialEq + PartialOrd + Ord + Clone + Debug + CalculateHash,
{
    btree_iterator {
        mbtree,
        nodes,
        position,
        node_id: -1,
        content: None,
    }
}

/// Next moves the iterator to the next element and returns true if there was a next element in the container.
/// If Next() returns true, then next element's key and value can be retrieved by Key() and Value().
/// If Next() was called for the first time, then it will point the iterator to the first element if it exists.
/// Modifies the state of the iterator.
pub fn next<T>(mut btree_iterator: &mut btree_iterator<T>) -> bool
where
    T: PartialEq + PartialOrd + Ord + Clone + Debug + CalculateHash,
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
            println!("btree_iterator.node_id {}", btree_iterator.node_id);
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
                println!("find_node.parent_id:{}", find_node.parent_id);
                if find_node.parent_id == -1 {
                    println!("break");
                    break;
                }
                btree_iterator.node_id = find_node.parent_id;
                find_node = btree_iterator
                    .nodes
                    .nodes_map
                    .get(&btree_iterator.node_id)
                    .unwrap();

                println!("find_node.parent_id:{}", find_node.parent_id);
                println!("btree_iterator.node_id:{}", btree_iterator.node_id);
                match find_node.content.binary_search(&entry) {
                    Ok(e) => {
                        let current_node = btree_iterator
                            .nodes
                            .nodes_map
                            .get(&btree_iterator.node_id)
                            .unwrap();

                        println!("wenbin test0.1");
                        if e < current_node.content.len() {
                            println!("wenbin test1");
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

                        println!("wenbin test0.2");
                        println!("{}", e);
                        println!("{}", current_node.content.len());
                        if e < current_node.content.len() {
                            println!("wenbin test1");
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

/// Prev moves the iterator to the previous element and returns true if there was a previous element in the container.
/// If Prev() returns true, then previous element's key and value can be retrieved by Key() and Value().
/// Modifies the state of the iterator.
pub fn prev<T>(mut btree_iterator: &mut btree_iterator<T>) -> bool
where
    T: PartialEq + PartialOrd + Ord + Clone + Debug + CalculateHash,
{
    // If already at beginning, go to begin
    match btree_iterator.position {
        position::begin => {
            begin(btree_iterator);
            return false;
        }
        position::end => {
            let right_node_id = btree_iterator
                .mbtree
                .right(btree_iterator.nodes.root_id, btree_iterator.nodes);
            if right_node_id == -1 {
                begin(btree_iterator);
                return false;
            }

            btree_iterator.node_id = right_node_id;
            let node = btree_iterator
                .nodes
                .nodes_map
                .remove(&right_node_id)
                .unwrap();
            let node_clone = node.clone();

            btree_iterator.nodes.nodes_map.insert(right_node_id, node);
            btree_iterator.content = Some(get_content_in_node(
                right_node_id,
                &mut btree_iterator,
                (node_clone.content.len() - 1) as i32,
            ));
            between(&mut btree_iterator);
            return true;
        }
        position::between => {
            // Find current entry position in current node
            let node = btree_iterator
                .nodes
                .nodes_map
                .remove(&btree_iterator.node_id)
                .unwrap();

            let node_clone = node.clone();
            btree_iterator
                .nodes
                .nodes_map
                .insert(btree_iterator.node_id, node);

            let entry = btree_iterator.content.clone().unwrap();
            let index = node_clone.content.binary_search(&entry).unwrap();

            // Try to go down to the child left of the current entry
            if index < node_clone.children_id.len() {
                btree_iterator.node_id = *node_clone.children_id.get(index).unwrap();
                // Try to go down to the child right of the current node
                let right_node_id = btree_iterator
                    .mbtree
                    .right(btree_iterator.node_id, btree_iterator.nodes);
                let right_node = btree_iterator
                    .nodes
                    .nodes_map
                    .remove(&right_node_id)
                    .unwrap();

                let right_node_clone = right_node.clone();
                btree_iterator
                    .nodes
                    .nodes_map
                    .insert(right_node_id, right_node);

                btree_iterator.node_id = right_node_id;
                btree_iterator.content = Some(get_content_in_node(
                    right_node_id,
                    &mut btree_iterator,
                    (right_node_clone.content.len() - 1) as i32,
                ));

                between(&mut btree_iterator);
                return true;
            }

            // Above assures that we have reached a leaf node, so return the previous entry in current node (if any)
            if index - 1 >= 0 {
                btree_iterator.content = Some(get_content_in_node(
                    node_clone.node_id,
                    &mut btree_iterator,
                    (index - 1) as i32,
                ));
            }

            let mut find_node = &node_clone.clone();
            // Reached leaf node and there are no contents to the left of the current entry, so go up to the parent
            loop {
                if find_node.parent_id == -1 {
                    break;
                }
                btree_iterator.node_id = find_node.parent_id;
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
                        if e - 1 >= 0 {
                            btree_iterator.content = Some(get_content_in_node(
                                btree_iterator.node_id,
                                &mut btree_iterator,
                                (e - 1) as i32,
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
                        if e - 1 >= 0 {
                            btree_iterator.content = Some(get_content_in_node(
                                btree_iterator.node_id,
                                &mut btree_iterator,
                                (e - 1) as i32,
                            ));
                            between(&mut btree_iterator);
                            return true;
                        }
                    }
                }
            }
        }
    }
    true
}

pub fn get_content_in_node<T>(
    node_id: i32,
    mut btree_iterator: &mut btree_iterator<T>,
    content_index: i32,
) -> T
where
    T: PartialEq + PartialOrd + Ord + Clone + Debug + CalculateHash,
{
    let node = btree_iterator.nodes.nodes_map.remove(&node_id).unwrap();
    let mut node_clone = node.clone();
    btree_iterator.nodes.nodes_map.insert(node_id, node);

    node_clone.content.remove(content_index as usize)
}

/// End moves the iterator past the last element (one-past-the-end).
/// Call Prev() to fetch the last element if any.
pub fn end<T>(btree_iterator: &mut btree_iterator<T>)
where
    T: PartialEq + PartialOrd + Ord + Clone + Debug + CalculateHash,
{
    btree_iterator.node_id = -1;
    btree_iterator.position = position::end;
    btree_iterator.content = None;
}

/// Begin resets the iterator to its initial state (one-before-first)
/// Call Next() to fetch the first element if any.
pub fn begin<T>(btree_iterator: &mut btree_iterator<T>)
where
    T: PartialEq + PartialOrd + Ord + Clone + Debug + CalculateHash,
{
    btree_iterator.node_id = -1;
    btree_iterator.position = position::begin;
    btree_iterator.content = None;
}

pub fn between<T>(btree_iterator: &mut btree_iterator<T>)
where
    T: PartialEq + PartialOrd + Ord + Clone + Debug + CalculateHash,
{
    btree_iterator.position = position::between;
}

pub fn item<T>(mut btree_iterator: &mut btree_iterator<T>) -> T
where
    T: PartialEq + PartialOrd + Ord + Clone + Debug + CalculateHash,
{
    return btree_iterator.content.clone().unwrap();
}

pub fn contents<T>(mut btree_iterator: &mut btree_iterator<T>) -> Vec<T>
where
    T: PartialEq + PartialOrd + Ord + Clone + Debug + CalculateHash,
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

/// First moves the iterator to the first element and returns true if there was a first element in the container.
/// If First() returns true, then first element's key and value can be retrieved by Key() and Value().
/// Modifies the state of the iterator
pub fn first<T>(btree_iterator: &mut btree_iterator<T>) -> bool
where
    T: PartialEq + PartialOrd + Ord + Clone + Debug + CalculateHash,
{
    begin(btree_iterator);
    next(btree_iterator)
}

/// Last moves the iterator to the last element and returns true if there was a last element in the container.
/// If Last() returns true, then last element's key and value can be retrieved by Key() and Value().
/// Modifies the state of the iterator.
pub fn last<T>(btree_iterator: &mut btree_iterator<T>) -> bool
where
    T: PartialEq + PartialOrd + Ord + Clone + Debug + CalculateHash,
{
    end(btree_iterator);
    prev(btree_iterator)
}
