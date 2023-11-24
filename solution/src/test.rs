#[cfg(test)]
mod test {
    use std::collections::VecDeque;
    fn to_vec(tree:Option<Rc<RefCell<TreeNode>>>) -> Vec<Option<i32>> {
        if tree.is_none() {return vec![]}
        let mut ret = vec![];
        let mut q = VecDeque::new();
        q.push_back(tree);
        while !q.is_empty() {
            let cur = q.pop_front().unwrap();
            match cur {
                None=>ret.push(None),
                Some(rf)=>{
                    ret.push(Some(rf.borrow().val));
                    if rf.borrow().left.is_none() && rf.borrow().right.is_none() && q.is_empty() {continue}
                    q.push_back(rf.borrow().left.clone());
                    q.push_back(rf.borrow().right.clone());
                }
            }
        }
        ret
    }
    fn from_vec(tree:Vec<Option<i32>>)-> Option<Rc<RefCell<TreeNode>>> {
        fn construct(i: Option<i32>) -> Option<Rc<RefCell<TreeNode>>> {
            match i {
                Some(var)=>Some(Rc::new(RefCell::new(TreeNode{left:None, right:None, val:var}))),
                None=>None
            }
        }
        match tree.len() {
            0=>None,
            _=>{
                let mut q = VecDeque::new();
                let ret =construct(tree[0]);
                q.push_back(ret.clone());
                let mut idx = 1;
                while !q.is_empty() {
                    let cur = q.pop_front().unwrap();
                    match cur {
                        None=> continue,
                        Some(rf)=>{
                            if idx<tree.len() {
                                let left = construct(tree[idx]);
                                q.push_back(left.clone());
                                rf.borrow_mut().left = left;
                                idx+=1;
                            }
                            if idx<tree.len() {
                                let right = construct(tree[idx]);
                                q.push_back(right.clone());
                                rf.borrow_mut().right = right;
                                idx+=1;
                            }
                        }
                    }
                }
                ret
            }
        }
    }

    use std::cell::RefCell;
    use std::rc::Rc;
    use crate::{Solution, TreeNode};
    #[test]
    fn testcase_1() {
        assert_eq!(2, Solution::pseudo_palindromic_paths(from_vec(vec![Some(2),Some(3),Some(1),Some(3),Some(1),None,Some(1)])))
    }


    #[test]
    fn testcase_2() {
        assert_eq!(1, Solution::pseudo_palindromic_paths(from_vec(vec![Some(2),Some(1),Some(1),Some(1),Some(3),None,None,None,None,None,Some(1)])))
    }

    #[test]
    fn testcase_3() {
        assert_eq!(1, Solution::pseudo_palindromic_paths(from_vec(vec![Some(9)])))
    }
    #[test]
    fn testcase_4() {
        assert_eq!(1, Solution::pseudo_palindromic_paths(from_vec(vec![Some(9),Some(5),Some(4),Some(5),None,Some(2),Some(6),Some(2),Some(5),None,Some(8),Some(3),Some(9),Some(2),Some(3),Some(1),Some(1),None,Some(4),Some(5),Some(4),Some(2),Some(2),Some(6),Some(4),None,None,Some(1),Some(7),None,Some(5),Some(4),Some(7),None,None,Some(7),None,Some(1),Some(5),Some(6),Some(1),None,None,None,None,Some(9),Some(2),None,Some(9),Some(7),Some(2),Some(1),None,None,None,Some(6),None,None,None,None,None,None,None,None,None,Some(5),None,None,Some(3),None,None,None,Some(8),None,Some(1),None,None,Some(8),None,None,None,None,Some(2),None,Some(8),Some(7)])))
    }
    #[test]
    fn test_helper() {
        assert_eq!(vec![Some(2),Some(1),Some(1),Some(1),Some(3),None,None,None,None,None,Some(1)], to_vec(from_vec(vec![Some(2),Some(1),Some(1),Some(1),Some(3),None,None,None,None,None,Some(1)])));
    }
}