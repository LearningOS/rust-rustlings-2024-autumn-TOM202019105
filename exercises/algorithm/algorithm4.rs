/*
	binary_search tree
	This problem requires you to implement a basic interface for a binary tree
*/
use std::cmp::Ordering;
use std::fmt::Debug;


#[derive(Debug)]
struct TreeNode<T>
where
    T: Ord,
{
    value: T,
    left: Option<Box<TreeNode<T>>>,
    right: Option<Box<TreeNode<T>>>,
}

#[derive(Debug)]
struct BinarySearchTree<T>
where
    T: Ord,
{
    root: Option<Box<TreeNode<T>>>,
}

impl<T> TreeNode<T>
where
    T: Ord,
{
    fn new(value: T) -> Self {
        TreeNode {
            value,
            left: None,
            right: None,
        }
    }
}

impl<T:PartialOrd + Copy> BinarySearchTree<T>
where
    T: Ord,
{

    fn new() -> Self {
        BinarySearchTree { root: None }
    }

    // Insert a value into the BST
    fn insert(&mut self, value: T) {
        //TODO
        if let None = self.root {
            self.root = Some(Box::new(TreeNode::new(value)));
            return;
        } else {
            BinarySearchTree::match_node_insert(&mut self.root,value);
        }
    }

    // Search for a value in the BST
    fn search(&self, value: T) -> bool {
        //TODO
        if let None = self.root {
            return false;
        } else {
            return BinarySearchTree::search_node(&self.root,value);
        }
    }

    fn match_node_insert(option_node:&mut Option<Box<TreeNode<T>>>,value:T){

        let mut node = option_node.as_deref_mut().unwrap();

        if (value == node.value) {
            return;
        }else if (value > node.value){
            match node.right {
                Some(_) => BinarySearchTree::match_node_insert(&mut node.right,value),
                None => {
                    node.right = Some(Box::new(TreeNode::new(value)));
                    //node.right.unwrap()
                },
            }
        } else {
            match node.left {
                Some(_) => BinarySearchTree::match_node_insert(&mut node.left,value),
                None => {
                    node.left = Some(Box::new(TreeNode::new(value)));
                    //node.left.unwrap()
                },
            }
        }
    }

    fn search_node(option_node:&Option<Box<TreeNode<T>>>,value:T) -> bool{
        let mut node = option_node.as_ref().unwrap();
        if (value == node.value){
            return true;
        } else if (value > node.value){
            match node.right {
                Some(_) => {
                    return BinarySearchTree::search_node(&node.right,value);
                },
                None => {
                    return false;
                },
            }
        } else {
            match node.left {
                Some(_) =>{ 
                    return BinarySearchTree::search_node(&node.left,value);
                },
                None => {
                    return false;
                },
            }
        }
    }
}

impl<T> TreeNode<T>
where
    T: Ord,
{
    // Insert a node into the tree
    fn insert(&mut self, value: T) {
        //TODO
        self.value = value;
        self.left = None;
        self.right = None;
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_insert_and_search() {
        let mut bst = BinarySearchTree::new();

        
        assert_eq!(bst.search(1), false);

        
        bst.insert(5);
        bst.insert(3);
        bst.insert(7);
        bst.insert(2);
        bst.insert(4);

        
        assert_eq!(bst.search(5), true);
        assert_eq!(bst.search(3), true);
        assert_eq!(bst.search(7), true);
        assert_eq!(bst.search(2), true);
        assert_eq!(bst.search(4), true);

        
        assert_eq!(bst.search(1), false);
        assert_eq!(bst.search(6), false);
    }

    #[test]
    fn test_insert_duplicate() {
        let mut bst = BinarySearchTree::new();

        
        bst.insert(1);
        bst.insert(1);

        
        assert_eq!(bst.search(1), true);

        
        match bst.root {
            Some(ref node) => {
                assert!(node.left.is_none());
                assert!(node.right.is_none());
            },
            None => panic!("Root should not be None after insertion"),
        }
    }
}    


