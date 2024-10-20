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

impl<T> BinarySearchTree<T>
where
    T: Ord,
{

    fn new() -> Self {
        BinarySearchTree { root: None }
    }

    // Insert a value into the BST
    fn insert(&mut self, value: T) {


        fn insert_recursive<T>(node: &mut Option<Box<TreeNode<T>>>, new_value: T)
        where
            T: Ord,
        {
            match node {
                Some(ref mut current_node) => {
                    if new_value < current_node.value {
                        insert_recursive(&mut current_node.left, new_value);
                    } else if new_value > current_node.value {
                        insert_recursive(&mut current_node.right, new_value);
                    } // Do nothing if values are equal (no duplicates allowed)
                }
                None => {
                    *node = Some(Box::new(TreeNode::new(new_value)));
                }
            }
        }

        insert_recursive(&mut self.root, value);
        /*let new_node = Box::new(TreeNode {  
            value,  
            left: None,  
            right: None,  
        });  
  
        fn insert_recursive(node: &mut Option<Box<TreeNode<T>>>, new_value: T) {  
            match node {  
                Some(ref mut current_node) => {  
                    if new_value < current_node.value {  
                        insert_recursive(&mut current_node.left, new_value);  
                    } else if new_value > current_node.value {  
                        insert_recursive(&mut current_node.right, new_value);  
                    }  
                    // Do nothing if values are equal (no duplicates allowed)  
                }  
                None => {  
                    *node = Some(new_node);  
                }  
            }  
        }  
  
        insert_recursive(&mut self.root, value);  */


        //TODO
        /*let r=self.root;
        let pre:=None<Box<TreeNode<T>>>;
        while  r!=None {*/
            /*if value<=*(r.Deref()).value{
                r=*(a.Deref()).left;
            }else{
                r=*(a.Deref()).right;
            }*/
            
            /*let t=Some(Box::new(TreeNode::new(value)));
            let r=root;*/
            //while let Some(a)=r {
                /*if value<=*(r.Deref()).value{
                    r=*(a.Deref()).left;
                }else{
                    r=*(a.Deref()).right;
                }*/
                /*pre=Some(a);
                if value<=a.deref().value {
                    r=a.deref().left;
                }else{
                    r=a.deref().right;
                }
            }
            
        }
            if let value <= pre.unwarp().deref().value {
                pre.left=Some(Box::new(TreeNode::new(value)));
            }else{
                pre.right=Some(Box::new(TreeNode::new(value)));
            }*/



            /*let new_node = Rc::new(RefCell::new(TreeNode {  
                value,  
                left: None,  
                right: None,  
            }));  
      
            let mut root_node = Rc::clone(&new_node);  
      
            fn insert_recursive(  
                root: &mut Option<Rc<RefCell<TreeNode<T>>>>,  
                new_value: Rc<RefCell<TreeNode<T>>>,  
            ) {  
                match root {  
                    Some(node_ref) => {  
                        let node_mut = node_ref.borrow_mut();  
                        match new_value.borrow().value.cmp(&node_mut.value) {  
                            Ordering::Less => {  
                                match &mut node_mut.left {  
                                    Some(left_child) => {  
                                        insert_recursive(&mut Some(Rc::clone(left_child)), new_value);  
                                    }  
                                    None => {  
                                        node_mut.left = Some(Rc::clone(&new_value));  
                                    }  
                                }  
                            }  
                            Ordering::Greater => {  
                                match &mut node_mut.right {  
                                    Some(right_child) => {  
                                        insert_recursive(&mut Some(Rc::clone(right_child)), new_value);  
                                    }  
                                    None => {  
                                        node_mut.right = Some(Rc::clone(&new_value));  
                                    }  
                                }  
                            }  
                            Ordering::Equal => {} // 不插入重复的值  
                        }  
                    }  
                    None => {  
                        *root = Some(Rc::clone(&new_node));  
                    }  
                }  
            }  
      
            insert_recursive(&mut self.root, root_node);  */
    }
        
    

    // Search for a value in the BST
    fn search(&self, value: T) -> bool {
        //TODO
        //true
        /*fn search_recursive(node: &Option<Box<TreeNode<T>>>, target: &T) -> bool {  
            match node {  
                Some(ref current_node) => {  
                    if target == &current_node.value {  
                        return true;  
                    } else if target < &current_node.value {  
                        return search_recursive(&current_node.left, target);  
                    } else {  
                        return search_recursive(&current_node.right, target);  
                    }  
                }  
                None => false,  
            }  
        }  
  
        search_recursive(&self.root, value) */
        fn search_recursive<T>(node: &Option<Box<TreeNode<T>>>, target: &T) -> bool
        where
            T: Ord,
        {
            match node {
                Some(ref current_node) => {
                    if target == &current_node.value {
                        true
                    } else if target < &current_node.value {
                        search_recursive(&current_node.left, target)
                    } else {
                        search_recursive(&current_node.right, target)
                    }
                }
                None => false,
            }
        }

        search_recursive(&self.root, &value)
    }
}

impl<T> TreeNode<T>
where
    T: Ord,
{
    // Insert a node into the tree
    fn insert(&mut self, value: T) {
        //TODO  
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


