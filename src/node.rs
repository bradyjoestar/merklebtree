use crate::merklebtree::Nodes;
use crate::traits::CalculateHash;
use core::borrow::BorrowMut;
use std::fmt::Debug;
use std::io::BufRead;

use serde::{Deserialize, Serialize};

#[derive(Clone, Debug)]
pub struct Node<T>
where
    T: PartialEq + PartialOrd + Ord + Clone + Debug + CalculateHash,
{
    pub root_flag: bool, //whether is root node
    pub parent_id: i32,
    pub children_id: Vec<i32>,
    pub content: Vec<T>,
    pub node_id: i32,
    pub hash: String,
}

#[derive(Clone, Deserialize, Serialize, Debug)]
pub struct NodeSer {
    pub root_flag: bool, //whether is root node
    pub parent_id: i32,
    pub children_id: Vec<i32>,
    pub content: Vec<String>,
    pub node_id: i32,
    pub hash: String,
}

impl<T> Node<T>
where
    T: PartialEq + PartialOrd + Ord + Clone + Debug + CalculateHash,
{
    pub fn new_empty(id: i32) -> Self {
        Node {
            root_flag: false,
            parent_id: -1,
            children_id: vec![],
            content: vec![],
            node_id: id,
            hash: "".to_string(),
        }
    }

    pub fn new_node(value: T, id: i32) -> Self {
        let value_clone = value.clone();
        let mut node = Node {
            root_flag: false,
            parent_id: -1,
            children_id: vec![],
            content: vec![value],
            node_id: id,
            hash: "".to_string(),
        };
        node.hash = value_clone.calculate();
        node
    }

    pub fn get_content(&self) -> Option<&Vec<T>> {
        Some(&(self.content))
    }
}

pub fn calculate_hash<T>(node_id: i32, nodes: &mut Nodes<T>)
where
    T: PartialEq + PartialOrd + Ord + Clone + Debug + CalculateHash,
{
    let mut hash = String::new();
    let mut node = nodes.nodes_map.remove(&node_id).unwrap();
    for i in node.content.iter() {
        hash.push_str(i.calculate().as_str());
    }
    for i in node.children_id.iter() {
        let child_node = nodes.nodes_map.get(i).unwrap();
        hash.push_str(child_node.hash.as_str());
    }
    node.hash = hex::encode(hash);
    nodes.nodes_map.insert(node_id, node);
}

//ReCalculateMerkleRoot update Merkleroot from node to root node.
pub fn recalculate_hash<T>(nodes: &mut Nodes<T>, node_id: i32)
where
    T: PartialEq + PartialOrd + Ord + Clone + Debug + CalculateHash,
{
    let mut node = nodes.nodes_map.remove(&node_id).unwrap();
    if node.node_id == nodes.root_id {
        nodes.nodes_map.insert(node.node_id, node);
        return calculate_hash(node_id, nodes);
    } else {
        let parent_id = node.parent_id;
        nodes.nodes_map.insert(node.node_id, node);
        calculate_hash(node_id, nodes);
        return recalculate_hash(nodes, parent_id);
    }
}

pub fn is_leaf<T>(nodeid: i32, nodes: &Nodes<T>) -> bool
where
    T: PartialEq + PartialOrd + Ord + Clone + Debug + CalculateHash,
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
    T: PartialEq + PartialOrd + Ord + Clone + Debug + CalculateHash,
{
    if is_leaf(insert_id, nodes) {
        insert_into_leaf(insert_id, value, order, nodes)
    } else {
        insert_into_internal(insert_id, value, order, nodes)
    }
}

pub fn insert_into_leaf<T>(insert_id: i32, value: T, order: u32, nodes: &mut Nodes<T>) -> bool
where
    T: PartialEq + PartialOrd + Ord + Clone + Debug + CalculateHash,
{
    let node = nodes.nodes_map.get_mut(&insert_id).unwrap();
    let content_slice = node.content.as_slice();
    match content_slice.binary_search(&value) {
        Ok(t) => {
            node.content.remove(t);
            node.content.insert(t, value);
            recalculate_hash(nodes,insert_id);
            return false;
        }
        Err(e) => {
            node.content.insert(e, value);
            split_node(insert_id, order, nodes);
            return true;
        }
    }
}

pub fn insert_into_internal<T>(insert_id: i32, value: T, order: u32, nodes: &mut Nodes<T>) -> bool
where
    T: PartialEq + PartialOrd + Ord + Clone + Debug + CalculateHash,
{
    let node = nodes.nodes_map.get_mut(&insert_id).unwrap();
    let content_slice = node.content.as_slice();
    match content_slice.binary_search(&value) {
        Ok(t) => {
            node.content.remove(t);
            node.content.insert(t, value);
            recalculate_hash(nodes,insert_id);
            return false;
        }
        Err(e) => {
            return insert(*node.children_id.get(e).unwrap(), value, order, nodes);
        }
    }
}

pub fn split_node<T>(split_id: i32, order: u32, nodes: &mut Nodes<T>)
where
    T: PartialEq + PartialOrd + Ord + Clone + Debug + CalculateHash,
{
    let node = nodes.nodes_map.get_mut(&split_id).unwrap();
    if !(node.content.len() > (order - 1) as usize) {
        recalculate_hash(nodes,split_id);
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
    T: PartialEq + PartialOrd + Ord + Clone + Debug + CalculateHash,
{
    let middle = middle(order);

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
    root_node.children_id.push(left_node.node_id);
    root_node.children_id.push(right_node.node_id);

    nodes.nodes_map.insert(nodes.next_id, left_node);
    nodes.nodes_map.insert(nodes.next_id + 1, right_node);
    nodes.nodes_map.insert(nodes.root_id, root_node);

    nodes.next_id = nodes.next_id + 2;
    nodes.size = nodes.size + 2;

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
    T: PartialEq + PartialOrd + Ord + Clone + Debug + CalculateHash,
{
    let middle = middle(order);
    let mut node = nodes.nodes_map.remove(&split_id).unwrap();
    let parent_id = node.parent_id;
    let mut parent_node = nodes.nodes_map.remove(&parent_id).unwrap();

    //remove index of split_node in parent
    for i in 0..parent_node.children_id.len() {
        let find_id = *parent_node.children_id.get(i).unwrap();
        if find_id == split_id {
            parent_node.children_id.remove(i);
            break;
        }
    }

    let mut left_node = Node::new_empty(nodes.next_id);
    let mut right_node = Node::new_empty(nodes.next_id + 1);
    let mut medium_node = Node::new_empty(nodes.root_id);

    medium_node.content = node.content.split_off(middle as usize);
    right_node.content = medium_node.content.split_off(1);
    left_node.content = node.content.clone();
    left_node.parent_id = parent_id;
    right_node.parent_id = parent_id;

    if !(node.children_id.len() == 0) {
        right_node.children_id = node.children_id.split_off((middle + 1) as usize);
        left_node.children_id = node.children_id.clone();
        set_parent(&mut (left_node.children_id), left_node.node_id, nodes);
        set_parent(&mut (right_node.children_id), right_node.node_id, nodes);
    }

    // Insert middle key into parent
    let content_slice = parent_node.content.as_slice();
    let value = medium_node.content.remove(0);
    match content_slice.binary_search(&value) {
        Err(e) => {
            parent_node.content.insert(e, value);
            parent_node.children_id.insert(e, left_node.node_id);
            parent_node.children_id.insert(e + 1, right_node.node_id);

            println!("parent_node: node id,{}", parent_node.node_id);
            nodes.nodes_map.insert(parent_node.node_id, parent_node);
            nodes.nodes_map.insert(nodes.next_id, left_node);
            nodes.nodes_map.insert(nodes.next_id + 1, right_node);

            nodes.next_id = nodes.next_id + 2;
            nodes.size = nodes.size + 1;
            split_node(parent_id, order, nodes);
        }
        _ => {}
    }
}

pub fn middle(order: u32) -> u32 {
    return (order - 1) / 2;
}

pub fn set_parent<T>(childrens: &mut Vec<i32>, parent_id: i32, nodes: &mut Nodes<T>) -> bool
where
    T: PartialEq + PartialOrd + Ord + Clone + Debug + CalculateHash,
{
    for i in childrens {
        let node = nodes.nodes_map.get_mut(&i).unwrap();
        node.parent_id = parent_id;
    }
    true
}

pub fn delete<T>(node_id: i32, index: i32, nodes: &mut Nodes<T>) -> ()
where
    T: PartialEq + PartialOrd + Ord + Clone + Debug + CalculateHash,
{
    let mut node = nodes.nodes_map.remove(&node_id).unwrap();

    // deleting from a leaf node
    if node.children_id.len() == 0 {
        let delete_item = node.content.remove(index as usize);
        nodes.nodes_map.insert(node_id, node);
        rebalance(node_id, delete_item, nodes);
        return;
    }
    // deleting from an internal node
    let left_largest_node_id = right(*node.children_id.get_mut(index as usize).unwrap(), nodes); // largest node in the left sub-tree (assumed to exist)
    let mut left_largest_node = nodes.nodes_map.remove(&left_largest_node_id).unwrap();
    let left_largest_content_index = left_largest_node.content.len() - 1;

    node.content.remove(index as usize);
    let delete_item = left_largest_node.content.remove(left_largest_content_index);
    let delete_item_clone = delete_item.clone();
    node.content.insert(index as usize, delete_item);

    nodes.nodes_map.insert(node_id, node);
    nodes
        .nodes_map
        .insert(left_largest_node_id, left_largest_node);

    rebalance(left_largest_node_id, delete_item_clone, nodes);
}

pub fn rebalance<T>(node_id: i32, mut value: T, nodes: &mut Nodes<T>) -> bool
where
    T: PartialEq + PartialOrd + Ord + Clone + Debug + CalculateHash,
{
    if (node_id == -1) {
        return false;
    }

    let mut node = nodes.nodes_map.get(&node_id).unwrap();
    let parent_id = node.parent_id;
    // check if rebalancing is needed
    if node.content.len() >= min_contents(nodes) as usize {
        println!("min contents:{}", min_contents(nodes));
        println!("needn't to rebalance");
        recalculate_hash(nodes,node_id);
        return false;
    }

    println!("need to rebalance, deletedItem  is :{:?}", value);

    let (left_sibling_id, left_sibling_index) = left_sibling(node_id, &value, nodes);
    if left_sibling_id != -1 {
        let mut left_sibling_node = nodes.nodes_map.remove(&left_sibling_id).unwrap();
        let mut delete_node = nodes.nodes_map.remove(&node_id).unwrap();
        let mut parent_node = nodes.nodes_map.remove(&parent_id).unwrap();
        if left_sibling_node.content.len() > min_contents(nodes) as usize {
            let sibling_data = left_sibling_node.content.pop().unwrap();
            println!("{}", left_sibling_index);
            let parent_data = parent_node.content.remove((left_sibling_index) as usize);
            delete_node.content.insert(0, parent_data);
            parent_node
                .content
                .insert(left_sibling_index as usize, sibling_data);

            nodes.nodes_map.insert(left_sibling_id, left_sibling_node);

            if !is_leaf(left_sibling_id, nodes) {
                let mut left_sibling_node = nodes.nodes_map.remove(&left_sibling_id).unwrap();
                let left_sibling_left_most_child_id = left_sibling_node.children_id.pop().unwrap();
                let mut left_sibling_left_most_child_node = nodes
                    .nodes_map
                    .remove(&left_sibling_left_most_child_id)
                    .unwrap();
                left_sibling_left_most_child_node.parent_id = node_id;

                delete_node
                    .children_id
                    .insert(0, left_sibling_left_most_child_id);

                nodes.nodes_map.insert(left_sibling_id, left_sibling_node);
                nodes.nodes_map.insert(
                    left_sibling_left_most_child_id,
                    left_sibling_left_most_child_node,
                );
            }

            nodes.nodes_map.insert(parent_id, parent_node);
            nodes.nodes_map.insert(node_id, delete_node);

            return true;
        }
        nodes.nodes_map.insert(parent_id, parent_node);
        nodes.nodes_map.insert(left_sibling_id, left_sibling_node);
        nodes.nodes_map.insert(node_id, delete_node);
    }

    let (right_sibling_id, right_sibling_index) = right_sibling(node_id, &value, nodes);
    if right_sibling_id != -1 {
        let mut right_sibling_node = nodes.nodes_map.remove(&right_sibling_id).unwrap();
        let mut delete_node = nodes.nodes_map.remove(&node_id).unwrap();
        let mut parent_node = nodes.nodes_map.remove(&parent_id).unwrap();

        if right_sibling_node.content.len() > min_contents(nodes) as usize {
            let sibling_data = right_sibling_node.content.remove(0);
            let parent_data = parent_node
                .content
                .remove((right_sibling_index - 1) as usize);
            delete_node.content.push(parent_data);
            parent_node
                .content
                .insert((right_sibling_index - 1) as usize, sibling_data);

            nodes.nodes_map.insert(right_sibling_id, right_sibling_node);

            if !is_leaf(right_sibling_id, nodes) {
                let mut right_sibling_node = nodes.nodes_map.remove(&right_sibling_id).unwrap();
                let right_sibling_left_most_child_id =
                    right_sibling_node.children_id.remove(0 as usize);
                let mut right_sibling_left_most_child_node = nodes
                    .nodes_map
                    .remove(&right_sibling_left_most_child_id)
                    .unwrap();
                right_sibling_left_most_child_node.parent_id = node_id;
                delete_node
                    .children_id
                    .push(right_sibling_left_most_child_id);
                nodes.nodes_map.insert(right_sibling_id, right_sibling_node);
                nodes.nodes_map.insert(
                    right_sibling_left_most_child_id,
                    right_sibling_left_most_child_node,
                );
            }

            nodes.nodes_map.insert(parent_id, parent_node);
            nodes.nodes_map.insert(node_id, delete_node);
            return true;
        }
        nodes.nodes_map.insert(parent_id, parent_node);
        nodes.nodes_map.insert(right_sibling_id, right_sibling_node);
        nodes.nodes_map.insert(node_id, delete_node);
    }

    // merge with siblings
    if right_sibling_id != -1 {
        // merge with right sibling
        let mut delete_node = nodes.nodes_map.remove(&node_id).unwrap();
        let mut parent_node = nodes.nodes_map.remove(&parent_id).unwrap();
        let mut right_sibling_node = nodes.nodes_map.remove(&right_sibling_id).unwrap();

        let parent_data = parent_node
            .content
            .remove((right_sibling_index - 1) as usize);
        value = parent_data.clone();
        delete_node.content.push(parent_data);
        for i in 0..right_sibling_node.content.len() {
            let right_sibling_node_data = right_sibling_node.content.remove(0);
            delete_node.content.push(right_sibling_node_data);
        }

        for i in 0..right_sibling_node.children_id.len() {
            delete_node
                .children_id
                .push(right_sibling_node.children_id.remove(0));
        }
        set_parent(&mut (right_sibling_node.children_id), node_id, nodes);
        parent_node.children_id.remove(right_sibling_index as usize);

        nodes.nodes_map.insert(parent_id, parent_node);
        nodes.nodes_map.insert(right_sibling_id, right_sibling_node);
        nodes.nodes_map.insert(node_id, delete_node);

        nodes.size = nodes.size - 1;
    } else if left_sibling_id != -1 {
        // merge with left sibling
        println!("borrow from left_sibling_id");
        let mut delete_node = nodes.nodes_map.remove(&node_id).unwrap();
        let mut parent_node = nodes.nodes_map.remove(&parent_id).unwrap();
        let mut left_sibling_node = nodes.nodes_map.remove(&left_sibling_id).unwrap();

        let parent_data = parent_node.content.remove((left_sibling_index) as usize);
        value = parent_data.clone();
        delete_node.content.insert(0, parent_data);

        for i in 0..left_sibling_node.content.len() {
            delete_node
                .content
                .insert(0, left_sibling_node.content.pop().unwrap())
        }

        for i in 0..left_sibling_node.children_id.len() {
            delete_node
                .children_id
                .insert(0, left_sibling_node.children_id.pop().unwrap() as i32)
        }
        set_parent(&mut (left_sibling_node.children_id), node_id, nodes);
        parent_node.children_id.remove(left_sibling_index as usize);

        nodes.nodes_map.insert(parent_id, parent_node);
        nodes.nodes_map.insert(left_sibling_id, left_sibling_node);
        nodes.nodes_map.insert(node_id, delete_node);

        nodes.size = nodes.size - 1;
    }

    if parent_id == -1 {
        return false;
    }
    // make the merged node the root if its parent was the root and the root is empty
    let parent_node = nodes.nodes_map.get(&parent_id).unwrap();
    if parent_id == nodes.root_id && parent_node.content.len() == 0 {
        let mut node = nodes.nodes_map.remove(&node_id).unwrap();
        node.parent_id = -1;
        node.node_id = 0;
        set_parent(&mut node.children_id, node.node_id, nodes);
        nodes.nodes_map.remove(&parent_id);
        nodes.nodes_map.insert(parent_id, node);
        return false;
    }

    return rebalance(parent_id, value, nodes);
}

// leftSibling returns the node's left sibling and child index (in parent) if it exists, otherwise (-1,-1)
// key is any of keys in node (could even be deleted).
pub fn left_sibling<T>(node_id: i32, value: &T, nodes: &mut Nodes<T>) -> (i32, i32)
where
    T: PartialEq + PartialOrd + Ord + Clone + Debug + CalculateHash,
{
    let node = nodes.nodes_map.get_mut(&node_id).unwrap();
    let parent_id = node.parent_id;
    if parent_id != -1 {
        println!("{}", parent_id);
        let parent_node = nodes.nodes_map.get_mut(&parent_id).unwrap();
        let content_slice = parent_node.content.as_slice();
        match content_slice.binary_search(value) {
            Ok(t) => {
                let index = t as i32 - 1;
                if index >= 0 && index < parent_node.children_id.len() as i32 {
                    return (*parent_node.children_id.get(index as usize).unwrap(), index);
                }
            }
            Err(e) => {
                let index = e as i32 - 1;
                if index >= 0 && index < parent_node.children_id.len() as i32 {
                    return (*parent_node.children_id.get(index as usize).unwrap(), index);
                }
            }
        }
    }
    (-1, -1)
}

// rightSibling returns the node's right sibling and child index (in parent) if it exists, otherwise (-1,-1)
// key is any of keys in node (could even be deleted).
pub fn right_sibling<T>(node_id: i32, value: &T, nodes: &mut Nodes<T>) -> (i32, i32)
where
    T: PartialEq + PartialOrd + Ord + Clone + Debug + CalculateHash,
{
    let node = nodes.nodes_map.get_mut(&node_id).unwrap();
    let parent_id = node.parent_id;
    if parent_id != -1 {
        let parent_node = nodes.nodes_map.get_mut(&parent_id).unwrap();
        let content_slice = parent_node.content.as_slice();
        match content_slice.binary_search(value) {
            Ok(mut t) => {
                let index = t as i32 + 1;
                if index < parent_node.children_id.len() as i32 {
                    return (*parent_node.children_id.get(index as usize).unwrap(), index);
                }
            }
            Err(e) => {
                let index = e as i32 + 1;
                if index < parent_node.children_id.len() as i32 {
                    return (*parent_node.children_id.get(index as usize).unwrap(), index);
                }
            }
        }
    }
    (-1, -1)
}

fn right<T>(mut node_id: i32, nodes: &mut Nodes<T>) -> i32
where
    T: PartialEq + PartialOrd + Ord + Clone + Debug + CalculateHash,
{
    if nodes.size == 0 {
        return -1;
    }
    if nodes.size == 1 {
        let root_node = nodes.nodes_map.get_mut(&node_id).unwrap();
        if root_node.content.len() == 0 {
            return -1;
        }
    }

    loop {
        let node = nodes.nodes_map.get(&node_id).unwrap();
        if is_leaf(node_id, nodes) {
            return node_id;
        }

        node_id = *node.children_id.last().unwrap();
    }
}

fn min_children<T>(nodes: &Nodes<T>) -> i32
where
    T: PartialEq + PartialOrd + Ord + Clone + Debug + CalculateHash,
{
    return ((nodes.m + 1) / 2) as i32; // ceil(m/2)
}

fn min_contents<T>(nodes: &Nodes<T>) -> i32
where
    T: PartialEq + PartialOrd + Ord + Clone + Debug + CalculateHash,
{
    return min_children(nodes) - 1;
}

fn max_children<T>(nodes: &Nodes<T>) -> i32
where
    T: PartialEq + PartialOrd + Ord + Clone + Debug + CalculateHash,
{
    return nodes.m as i32;
}

fn max_contents<T>(nodes: &Nodes<T>) -> i32
where
    T: PartialEq + PartialOrd + Ord + Clone + Debug + CalculateHash,
{
    return max_children(nodes) - 1;
}
