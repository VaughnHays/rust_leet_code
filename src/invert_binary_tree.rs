use std::cell::RefCell;
use std::rc::Rc;
// #226 Invert Binary Tree

#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        TreeNode {
            val,
            left: None,
            right: None,
        }
    }
}

// if-let solution
pub fn invert_tree(root: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
    if let Some(root) = &root {
        if let Ok(mut root) = root.try_borrow_mut() {
            let left = root.left.clone();
            root.left = invert_tree(root.right.clone());
            root.right = invert_tree(left);
        }
    }
    root
}

// map()/match solution
// pub fn invert_tree(root: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
//     root.map(|root| {
//         match root.borrow_mut() {
//             mut node => {
//                 let left = node.left.clone();
//                 node.left = invert_tree(node.right.clone());
//                 node.right = invert_tree(left);
//             }
//         };
//         root
//     })
// }
