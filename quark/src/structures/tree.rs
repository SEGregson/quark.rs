use std::{thread, sync::{RwLock, Arc}};

struct TreeNode<T> {
    key: String,
    value: T,
    arcs: Vec<TreeNode<T>>
}

impl<T> Clone for TreeNode<T> where T: Clone {
    fn clone(&self) -> Self {
        Self { 
            key: self.key.clone(),
            value: self.value.clone(), 
            arcs: self.arcs.clone() 
        }
    }
}

impl<T> TreeNode<T> {
    fn new(key: &str, value: T) -> TreeNode<T>{
        TreeNode{
            key: key.to_string(),
            value: value,
            arcs: vec![]
        }
    }

    fn add_subtree(&mut self, tree: TreeNode<T>) {
        self.arcs.insert(self.arcs.len(), tree);
    }
}

pub struct Tree<T> {
    root: TreeNode<T>
}

impl<T> Tree<T> where T: Clone, T: Send {
    pub fn new(key: &str, value: T) -> Tree<T> {
        Tree { root: TreeNode::new(key, value) }
    }

    pub fn get_size(&self) -> usize {
        let count = 0;
        Tree::_get_size(self.root.clone(), count)
    }

    fn _get_size(sub_tree: TreeNode<T>, mut count: usize) -> usize {
        for node in sub_tree.arcs {
            count = Tree::_get_size(node, count);
        }
        return count + 1;
    }

    pub fn bredth_first(&self, key: &str) -> Option<T> {
        let mut queue = self.root.arcs.clone();

        while queue.len() > 0 {
            let mut node = queue.pop().unwrap();
            if node.key == key {
                return Some(node.value);
            }
            queue.append(&mut node.arcs)
            
        }

        return None;
    }

    pub fn depth_first(&self, key: &str) -> Option<T> {
        Tree::_depth_first(self.root.clone(), key.to_string())
    }

    fn _depth_first(sub_tree: TreeNode<T>, key: String) -> Option<T> {
        let mut out:Option<T> = None;
        if sub_tree.key == key {return Some(sub_tree.value);}
        for node in sub_tree.arcs {
            out = Tree::_depth_first(node, key.clone());
        }
        return out;
    }

    fn node_search(&self, key: &str) -> Option<TreeNode<T>> {
        let mut queue = self.root.arcs.clone();

        while queue.len() > 0 {
            let mut node = queue.pop().unwrap();
            if node.key == key {
                return Some(node);
            }
            queue.append(&mut node.arcs)
            
        }

        return None;
    }



    // fn flood_search(&self, key: &str) -> Option<T> {
    //     Tree::_flood_search(self.root.clone(), key.to_string())
    // }

    // fn _flood_search(sub_tree: TreeNode<T>, key: String) -> Option<T> {
    //     if sub_tree.key == key {return Some(sub_tree.value);}
    //     thread::spawn(move || {
    //         let mut out = None;
    //         for node in sub_tree.arcs {
    //             out = Tree::_flood_search(node, key);
    //         }
    //         return out;
    //     }).join().unwrap()
    // }

    pub fn insert_value(&mut self, parent_key: &str, key: &str, value: T) -> bool {
        match self.node_search(parent_key) {
            Some(mut parent) => {
                parent.arcs.insert(parent.arcs.len(), TreeNode::new(key, value));
                true
            },
            None => false
        }
    }


    pub fn remove_subtree(&mut self, parent_tree: &str) -> bool {
        match self.node_search(parent_tree) {
            Some(mut tree) => {
                tree.arcs = vec![];
                true
            },
            None => false,
        }
    }

}