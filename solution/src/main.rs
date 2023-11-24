mod test;

pub struct Solution;

// Definition for a binary tree node.
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

use std::rc::Rc;
use std::cell::RefCell;

type T = Option<Rc<RefCell<TreeNode>>>;

fn cnt(n:i32)-> i32 {
    (1..=9).map(|x| if (n>>x)&1==1 {1} else {0}).sum()
}
fn dfs(root: T, acc: i32) -> i32 {
    match root {
        Some(rf) => {
            let cur = acc ^ (1 << rf.borrow().val);
            if rf.borrow().left.is_none() && rf.borrow().right.is_none() && cnt(cur) <= 1 {
                1
            } else {
                dfs(rf.borrow().left.clone(),cur) +dfs(rf.borrow().right.clone(), cur)
            }
        }
        None => 0
    }
}

impl Solution {
    pub fn pseudo_palindromic_paths(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        dfs(root, 0)
    }
}


fn main() {}
